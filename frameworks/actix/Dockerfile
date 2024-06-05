FROM rust:1.78-buster as builder
WORKDIR /app
COPY Cargo.toml ./
COPY src/ ./src
RUN cargo install --path .

FROM debian:buster
COPY --from=builder /usr/local/cargo/bin/actix_web  /usr/local/bin/actix_web

CMD ["actix_web"]