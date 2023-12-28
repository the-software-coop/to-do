FROM rust

WORKDIR /code

COPY . /code/

RUN cargo build

CMD ["cargo", "run"]