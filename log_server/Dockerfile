FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev

WORKDIR /app
COPY . .
RUN cargo build --release

FROM scratch
WORKDIR /app

COPY --from=builder /app/target/release/log_server /app/log_server
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt 
ENTRYPOINT ["/app/log_server"]
