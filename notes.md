# Notes

## Getting to the Assignment

So the assignment is on the Kusama blockchain, after searching, it just seems to be the test network for polkadot. Akin to hyperspace on filecoin. Time to find a polkadot explorer then.

Using polkascan and searching for 17562336 brings up the block. I have no no idea about what events are on polkadot but I'll assume "remark" means some sort of comment or attached data.

Clicking that gives me the extrinsic (whatever that is) and inside the call arguments (what makes this a call argument and not just data?) I find the assignment URL bit.ly/parity-rel-ra02

A quick look to try and satisfy my questions above. An extrinsic seems to be a function (or set of functions) on chain that can be called. So I guess the "remark" is just a function to store plain text in a block? that would explain why the URL was passed as a call argument.

Anyway on to the assignment!

## Technical Questions

The assignment is two technical questions and a programming exercise. I'll be answering the technical questions in their own file and focusing on the exercise here.

## The Assignment

So the Assignment wants me to make a containerised tool that does some signature signing/validation with gpg.

My initial thoughts are to use Rust for the programming because I like rust :). But also because it works great for general tasks and is quite fast for me to hack with. I'm also probably going to be calling gpg as a command instead of relying on a library just because it will end up easier to write and is probably the better choice for a simple script.

Bash is for sure  a great choice here too as it's a simple script that mostly resolves around calling gpg. However there does appear to be enough extra logic here for me personally to prefer something a bit more structured.

After writing the program it was a bit more annoying than I thought it would be (I forgot how awful gpg is). Maybe I should have used a library instead. Too late now though. Overall I think the solution is not *that* bad. There's basic error handling though the messages are probably not the most useful.

So I added some tests, it went very smoothly and I'm pretty happy. Rust's tests always please me. I did not write a test for the import function partially because of time constraints and partially because I don't really want the tests to modify the gpg keyring. Writing this more proper I would make the tests point to a custom keyring directory that I construct then destruct for each test.

Onto the GitHub action, this was also pretty straight forward apart from the usual pains of debugging code you can't run locally. I added an artefact so the output can actually be inspected.

Readme is now also done and I think that's it for the project now.

