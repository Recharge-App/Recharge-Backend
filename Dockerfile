FROM rust:latest

EXPOSE 8080

WORKDIR /usr/actix/
RUN mkdir src
COPY ./src ./src
COPY ./Cargo.lock ./Cargo.toml ./.env .

RUN cargo install --path .

CMD ["cargo", "run"]
