FROM rust:alpine

WORKDIR /code

RUN apk add build-base
RUN apk add libpq-dev

COPY ./src /code/src
COPY ./Cargo.toml /code/

RUN cargo build --release

COPY . /code/


CMD ["cargo", "run"]