FROM rust:alpine

WORKDIR /code

RUN apk add build-base

COPY . /code/

RUN cargo build

CMD ["cargo", "run"]