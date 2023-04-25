FROM rust:1
COPY ./ ./
# parity's real public key
RUN gpg --recv 9D4B2B6EB8F97156D19669A9FF0812D491B96798
# dummy public key used for testing sake
RUN gpg --import parity-public-key
RUN cargo build --release
ENTRYPOINT ["./target/release/parity-gpg-assignment"]
