use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

use axum::{Router, extract::State, routing::get};

struct AppState {
    counter: AtomicU8,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        counter: AtomicU8::new(0),
    });

    let app = Router::new()
        .route(
            "/",
            get(|State(state): State<Arc<AppState>>| async move {
                let current = state.counter.fetch_add(1, Ordering::SeqCst);
                format!("pong {}", current)
            }),
        )
        .with_state(state);

    let port = env::var("PORT").unwrap_or(String::from("3000"));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &port))
        .await
        .unwrap();

    println!("Server started at port {}", &port);
    axum::serve(listener, app).await.unwrap();
}
