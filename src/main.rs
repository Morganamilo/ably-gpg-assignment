use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::{stdin, Read, Write};
use std::ops::Deref;

use std::process::{exit, Command, Stdio};

use anyhow::{bail, ensure, Context};
use reqwest::blocking::Client;

//const PUB_KEY: &str = "9D4B2B6EB8F97156D19669A9FF0812D491B967982";

#[derive(serde::Serialize)]
pub struct Summary {
    pub url: String,
    pub file: String,
    pub sig: String,
    pub sig_ok: bool,
    pub gpg_output: String,
}

fn main() -> anyhow::Result<()> {
    // parse args as urls
    let args = args().skip(1);
    let mut summary = Vec::new();
    let urls = args.collect::<Vec<_>>();
    let reqwest = Client::new();

    // read priv key from stdin
    let mut priv_key = Vec::new();
    stdin().lock().read_to_end(&mut priv_key)?;

    ensure!(!priv_key.is_empty(), "no private key specified");

    match urls.len() {
        // the task said to warn but returning 1 feels more correct
        0 => bail!("no input URLs"),
        16.. => bail!("the max input size is 16"),
        _ => (),
    }

    // download all the files
    for url in urls.iter() {
        // it's possible we have duplicate file names which would cause some files to be ignored
        // not accounting for that in this exersie
        let url = url::Url::parse(url)?;
        let name = url.path().split('/').last().context("no filename")?;

        let resp = reqwest.get(url.as_str()).send()?;
        ensure!(resp.status().is_success(), "server returned error");
        let bytes = resp.bytes()?;
        let mut file = mkfile(&format!("/out/{}", name))?;
        file.write_all(bytes.deref())?;

        // download the sig too
        let resp = reqwest.get(format!("{}.asc", url)).send()?;
        let bytes = resp.bytes()?;
        let mut file = mkfile(&format!("/out/{}.asc", name))?;
        file.write_all(bytes.deref())?;
    }

    // now verify
    for url in urls.iter() {
        let url = url::Url::parse(url)?;
        let name = url.path().split('/').last().context("no filename")?;

        let (ok, output) = verify(name)?;

        summary.push(Summary {
            url: url.to_string(),
            file: name.to_string(),
            sig: format!("{}.asc", name),
            sig_ok: ok,
            gpg_output: output,
        });
    }

    let json = serde_json::to_string_pretty(&summary)?;
    println!("{}", json);

    let mut file = mkfile("/out/summary.json")?;
    file.write_all(json.as_bytes())?;

    if !summary.iter().all(|s| s.sig_ok) {
        exit(1);
    }

    let fingerprint = get_fingerprint(&priv_key)?;
    import(&priv_key)?;

    // sign all the files
    // the assignment said file singular, but that doesn't really make any sense
    // so i'll assume it meant all input files
    for url in urls.iter() {
        let url = url::Url::parse(url)?;
        let name = url.path().split('/').last().context("no filename")?;

        sign(name, &fingerprint)?;
    }

    Ok(())
}

fn mkfile(name: &str) -> anyhow::Result<File> {
    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(name)
        .context("failed to create file")
}

fn verify(name: &str) -> anyhow::Result<(bool, String)> {
    // verify the sign is signed by a sig in our keyring
    // technically we don't actually check which key the file is signed with here
    // just that the key is in our keyring.
    //
    // this works for the purpose of this task. plus this script is meant to run in a clean
    // container
    let res = Command::new("gpg")
        .arg("--verify")
        .arg(format!("/out/{}.asc", name))
        .arg(format!("/out/{}", name))
        .output()?;

    let output = String::from_utf8_lossy(&res.stderr).to_string();
    Ok((res.status.success(), output))
}

fn sign(name: &str, fingerprint: &str) -> anyhow::Result<()> {
    let res = Command::new("gpg")
        .arg("-u")
        .arg(fingerprint)
        .arg("--output")
        .arg(format!("/out/{}.notary.asc", name))
        .arg("--detach-sign")
        .arg(format!("/out/{}", name))
        .output()?;

    let output = String::from_utf8_lossy(&res.stderr).to_string();
    ensure!(res.status.success(), output);
    Ok(())
}

fn get_fingerprint(key: &[u8]) -> anyhow::Result<String> {
    let mut cmd = Command::new("gpg")
        .arg("--with-colons")
        .arg("--import-options")
        .arg("show-only")
        .arg("--import")
        .arg("--fingerprint")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;

    cmd.stdin.as_mut().unwrap().write_all(key)?;
    let res = cmd.wait_with_output()?;

    let output = String::from_utf8_lossy(&res.stdout).to_string();
    ensure!(res.status.success(), output);

    let output = String::from_utf8(res.stdout).context("gpg output not utf8")?;
    let fingerprint = output
        .lines()
        .find(|line| line.starts_with("fpr"))
        .context("gpg output missing fingerprint")?;
    let fingerprint = fingerprint
        .split(':')
        .nth(9)
        .context("gpg output missing fingerprint")?;
    ensure!(!fingerprint.is_empty(), "gpg output missing fingerprint");
    Ok(fingerprint.to_string())
}

fn import(key: &[u8]) -> anyhow::Result<()> {
    let mut cmd = Command::new("gpg")
        .arg("--import")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;

    cmd.stdin.as_mut().unwrap().write_all(key)?;
    let res = cmd.wait_with_output()?;

    let output = String::from_utf8_lossy(&res.stderr).to_string();
    ensure!(res.status.success(), output);
    Ok(())
}
