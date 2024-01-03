FROM rust

WORKDIR /code

COPY ./src /code/src
COPY ./Cargo.toml /code/

RUN cargo build --release

COPY . /code/


CMD ["cargo", "run"]