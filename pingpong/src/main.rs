use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::{
    env,
    sync::{Arc, LazyLock},
};

use axum::{Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};

static DATABASE_URL: LazyLock<String> =
    LazyLock::new(|| env::var("DATABASE_URL").expect("missing env var DATABASE_URL"));

struct AppState {
    pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&*DATABASE_URL)
        .await
        .expect(&format!("unable to connect to DB @ {}", &*DATABASE_URL));

    let state = Arc::new(AppState { pool });

    let app = Router::new()
        .route("/pingpong", get(handle_browser))
        .route("/healthz", get(healthcheck))
        // for the ingress or whichever manifest that required `GET / -> 200 OK`
        .route("/", get(healthcheck))
        .route("/pings", get(handle_ping))
        .with_state(state);

    let ip = env::var("IP").expect("missing env var IP");
    let port = env::var("PORT").expect("missing env var PORT");

    let addr = format!("{}:{}", &ip, &port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("listening at http://{}/pingpong", &addr);
    axum::serve(listener, app).await.unwrap();
}

async fn healthcheck(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let res: Result<(i64,), sqlx::Error> = sqlx::query_as("SELECT count(*) FROM pings;")
        .fetch_one(&state.pool)
        .await;

    if res.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::FAILED_DEPENDENCY
    }
}

async fn handle_ping(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let count: (i64,) = sqlx::query_as("SELECT count(*) FROM pings;")
        .fetch_one(&state.pool)
        .await
        .expect("query failed");

    axum::response::Json::from(count.0)
}

async fn handle_browser(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let count: (i64,) = sqlx::query_as(
        "
            WITH ins AS (
                INSERT INTO pings DEFAULT VALUES
            )
            SELECT count(*) FROM pings;
        ",
    )
    .fetch_one(&state.pool)
    .await
    .expect("query failed");

    format!("pong {}", count.0)
}
