FROM rust:latest as builder
WORKDIR /app
COPY Cargo.toml ./
COPY src/ ./src
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/actix_web  /usr/local/bin/actix_web

EXPOSE 80
CMD ["actix_web"]