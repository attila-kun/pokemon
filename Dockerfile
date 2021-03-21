FROM rust:1.50 as builder
WORKDIR /usr/src

RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl

RUN USER=root mkdir pokemon && mkdir pokemon/pokemon_core && mkdir pokemon/server
WORKDIR /usr/src/pokemon
COPY Cargo.toml Cargo.lock ./
COPY pokemon_core ./pokemon_core
COPY server ./server
RUN cargo build --release

USER 1000
CMD ["./target/release/server"]