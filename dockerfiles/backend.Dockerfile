FROM rust:1.89 AS builder

WORKDIR /usr/src/app
COPY back/ .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/back /app/bin

WORKDIR /app
ENTRYPOINT [ "/app/bin" ]