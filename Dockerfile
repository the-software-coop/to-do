FROM rust:alpine

WORKDIR /code

RUN apk add build-base

COPY ./src /code/src
COPY ./Cargo.toml /code/

RUN cargo build

COPY . /code/


CMD ["cargo", "run"]