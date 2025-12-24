FROM tomjtoth/dioxus-docker:0.7.2-alpine AS builder

WORKDIR /app

# Compiling dummy app, so Docker can cache Dioxus deps.
# Changing app deps invalidates the cache.
COPY Cargo.toml Cargo.lock Dioxus.toml .
RUN mkdir assets && \
    mkdir src && \
    echo "\
    use dioxus::prelude::*;\
    fn main() {\
        dioxus::launch(|| rsx! {});\
    }" >> src/main.rs && \
    dx build --release

# Compiling real app with access to cache
COPY . .
RUN tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --minify && \
    dx build --release

FROM scratch

WORKDIR /app
COPY --from=builder /app/target/dx/todo_app/release/web /app
ENV IP=0.0.0.0 PORT=3000
ENTRYPOINT ["/app/todo_app"]
