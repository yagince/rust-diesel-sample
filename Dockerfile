FROM rust:1.30.0-stretch

MAINTAINER yagince <straitwalk@gmail.com>

RUN apt-get -y -q update \
  && apt-get install -y -q \
     libpq-dev \
  && cargo install diesel_cli --no-default-features --features postgres

ENV CARGO_BUILD_TARGET_DIR=/tmp/target

RUN USER=root cargo new --bin app
WORKDIR /app
COPY ./Cargo.toml Cargo.toml
COPY ./Cargo.lock Cargo.lock

RUN cargo build --release --color never && \
    rm src/*.rs
