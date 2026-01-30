use std::env;

use axum::{Router, http::StatusCode, routing::get};

#[tokio::main]
async fn main() {
    let msg = env::var("MESSAGE").expect("missing env var MESSAGE");

    let app = Router::new()
        .route("/", get(|| async { msg }))
        .route("/healthz", get(|| async { StatusCode::OK }));

    let ip = env::var("IP").expect("missing env var IP");
    let port = env::var("PORT").expect("missing env var PORT");

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", &ip, &port))
        .await
        .unwrap();

    println!("GREETER listening at :{}/", &port);
    axum::serve(listener, app).await.unwrap();
}
