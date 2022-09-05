FROM rust:latest as builder
WORKDIR /app
COPY Cargo.toml ./
COPY src/ ./src
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/rust-rocket  /usr/local/bin/rust-rocket

EXPOSE 8000
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["rust-rocket"]