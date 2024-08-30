FROM rust AS builder
WORKDIR /tmp
RUN apt update && apt upgrade -y && apt install -y libssl-dev pkg-config

# this builds and caches all the dependencies in an extra layer
RUN cargo new --bin certgen
WORKDIR /tmp/certgen
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# now, copy project sources and build again
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt update && apt upgrade -y && apt install -y openssl
COPY --from=builder /tmp/certgen/target/release/certgen .
COPY .env certs.yml ./
ENTRYPOINT ["./certgen"]
