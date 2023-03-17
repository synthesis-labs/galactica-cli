FROM rust:1-buster as builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    build-essential \
    libssl-dev \
    pkg-config 

WORKDIR /build

RUN git clone https://github.com/synthesis-labs/galactica-lib.git
RUN git clone https://github.com/synthesis-labs/galactica-cli.git

WORKDIR /build/galactica-cli

RUN cargo build --release --target x86_64-unknown-linux-gnu
RUN cp /build/galactica-cli/target/x86_64-unknown-linux-gnu/release/galactica /build/galactica