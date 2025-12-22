FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev

WORKDIR /app
COPY . .
RUN cargo build --release

FROM scratch
WORKDIR /app
COPY --from=builder /app/target/release/log_server /app/log_server
ENTRYPOINT ["/app/log_server"]
