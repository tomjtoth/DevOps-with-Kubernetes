FROM rust:alpine AS builder
WORKDIR /app

COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    apk add --no-cache musl-dev && \
    cargo build --release && \
    mv /app/target/release/todo_backend /app.built

FROM scratch
WORKDIR /app

COPY --from=builder /app.built /app/bin
ENTRYPOINT ["/app/bin"]
