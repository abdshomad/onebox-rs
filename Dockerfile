# Multi-stage Dockerfile for onebox-rs (server and client)

FROM rust:1.82 as builder
WORKDIR /app

# Cache dependencies
COPY Cargo.toml ./
COPY onebox-core/Cargo.toml onebox-core/Cargo.toml
COPY onebox-client/Cargo.toml onebox-client/Cargo.toml
COPY onebox-server/Cargo.toml onebox-server/Cargo.toml
RUN mkdir -p onebox-core/src onebox-client/src onebox-server/src \
    && echo "pub fn main(){}" > onebox-client/src/main.rs \
    && echo "pub fn main(){}" > onebox-server/src/main.rs \
    && echo "pub mod lib{}" > onebox-core/src/lib.rs \
    && cargo build -q || true

# Build actual binaries
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim as runtime
RUN useradd -u 10001 -m onebox && \
    apt-get update && apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/onebox-server /usr/local/bin/onebox-server
COPY --from=builder /app/target/release/onebox-client /usr/local/bin/onebox-client
USER onebox
WORKDIR /home/onebox

# Default command does nothing; docker-compose will override
CMD ["/bin/sh", "-c", "echo onebox image ready"]

