FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev

WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine
WORKDIR /app
COPY --from=builder /app/target/release/log_output /app/log_output
ENTRYPOINT ["/app/log_output"]
