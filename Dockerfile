FROM rust:slim as builder
WORKDIR /tmp
RUN apt update && apt upgrade -y && apt install -y libssl-dev pkg-config

# this builds and caches all the dependencies in an extra layer
RUN cargo new --bin certgen
WORKDIR /tmp/certgen
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

# now, copy project sources and build again
RUN rm -rf src
COPY ./src ./src
RUN cargo clean && cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY .env .env
COPY --from=builder /tmp/certgen/target/release/certgen certgen
ENTRYPOINT ["./certgen"]
