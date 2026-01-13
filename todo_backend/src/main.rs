use std::sync::LazyLock;
use std::{env, sync::Arc};

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, query, query_as};

struct AppState(Pool<Postgres>);

static DATABASE_URL: LazyLock<String> =
    LazyLock::new(|| env::var("DATABASE_URL").expect("missing env var DATABASE_URL"));

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&*DATABASE_URL)
        .await
        .expect(&format!("unable to connect to DB @ {}", &*DATABASE_URL));

    let state = Arc::new(AppState(pool));

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

async fn retrieve_todos(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let todos: Vec<(String,)> = query_as("SELECT todo FROM todos")
        .fetch_all(&state.0)
        .await
        .expect("retrieving todos failed");

    println!("serving {} todos", todos.len());

    let mapped = todos
        .iter()
        .map(|(todo,)| todo.to_string())
        .collect::<Vec<String>>();

    axum::response::Json::from(mapped)
}

async fn add_todo(
    State(state): State<Arc<AppState>>,
    Json(todo): Json<String>,
) -> impl IntoResponse {
    let len = todo.len();

    println!("Adding todo:\n\t{}", todo);

    if len > 140 {
        eprintln!("todo longer, than 140 ({}), aborting!", len);
        return StatusCode::BAD_REQUEST;
    }

    query("INSERT INTO todos (todo) VALUES ($1)")
        .bind(todo)
        .execute(&state.0)
        .await
        .expect("adding todo failed");

    StatusCode::CREATED
}
