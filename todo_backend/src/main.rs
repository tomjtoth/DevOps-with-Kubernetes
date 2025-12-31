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

    let ip = env::var("IP").expect("missing env var IP");
    let port = env::var("PORT").expect("missing env var PORT");

    let addr = format!("{}:{}", &ip, &port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("listening at http://{}/todos", &addr);
    axum::serve(listener, app).await.unwrap();
}

async fn retrieve_todos(State(todos): State<AppState>) -> impl IntoResponse {
    let todos = todos.lock().await.clone();
    println!("serving {} todos", todos.len());
    axum::response::Json::from(todos)
}

async fn add_todo(State(todos): State<AppState>, Json(todo): Json<String>) -> impl IntoResponse {
    println!("pushing \"{}\" into todos", &todo);
    todos.lock().await.push(todo);
    StatusCode::CREATED
}
