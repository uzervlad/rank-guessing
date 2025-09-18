FROM rust:1.89 AS builder

WORKDIR /usr/src/app
COPY back/ .

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder /usr/src/app/target/release/back /app/bin

WORKDIR /app
ENTRYPOINT [ "/app/bin" ]