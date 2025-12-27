use std::{env, sync::Arc};

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use tokio::sync::Mutex;

type AppState = Arc<Mutex<Vec<String>>>;

#[tokio::main]
async fn main() {
    let state: AppState = Arc::new(Mutex::new(
        "Learn Rust, Learn Dioxus, Build something"
            .split(", ")
            .map(String::from)
            .collect(),
    ));

    let app = Router::new()
        .route("/todos", get(retrieve_todos))
        .route("/todos", post(add_todo))
        .with_state(state);

    let ip = env::var("IP").unwrap_or(String::from("127.0.0.1"));
    let port = env::var("PORT").unwrap_or(String::from("3000"));

    let addr = format!("{}:{}", &ip, &port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("listening at http://{}/todos", &addr);
    axum::serve(listener, app).await.unwrap();
}

async fn retrieve_todos(State(todos): State<AppState>) -> impl IntoResponse {
    let todos = todos.lock().await.clone();
    axum::response::Json::from(todos)
}

async fn add_todo(State(todos): State<AppState>, Json(todo): Json<String>) -> impl IntoResponse {
    todos.lock().await.push(todo);
    StatusCode::CREATED
}
