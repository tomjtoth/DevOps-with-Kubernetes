use std::{
    env,
    sync::{
        Arc,
        atomic::{AtomicU8, Ordering},
    },
};

use axum::{Router, extract::State, response::IntoResponse, routing::get};

struct AppState {
    counter: AtomicU8,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        counter: AtomicU8::new(0),
    });

    let app = Router::new()
        .route("/pingpong", get(handle_browser))
        .route("/pings", get(handle_ping))
        .with_state(state);

    let port = env::var("PORT").unwrap_or(String::from("3000"));

    let addr = format!("0.0.0.0:{}", &port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("listening at http://{}/pingpong", &addr);
    axum::serve(listener, app).await.unwrap();
}

async fn handle_ping(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    axum::response::Json::from(state.counter.load(Ordering::SeqCst))
}

async fn handle_browser(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let current = state.counter.fetch_add(1, Ordering::SeqCst);

    format!("pong {}", current)
}
