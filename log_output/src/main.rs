use std::{env, sync::Arc, time::Duration};

use axum::{Router, routing::get};
use tokio::time::sleep;
use uuid::Uuid;

use time::UtcDateTime;

fn current_status(id: &Arc<Uuid>) -> String {
    format!("{}: {}", UtcDateTime::now(), id)
}

#[tokio::main]
async fn main() {
    let id = Arc::new(Uuid::new_v4());

    let id_for_loop = id.clone();
    tokio::spawn(async move {
        loop {
            println!("{}", current_status(&id_for_loop));
            sleep(Duration::from_secs(5)).await;
        }
    });

    let app = Router::new().route(
        "/",
        get(move || {
            let id = Arc::clone(&id);
            async move { current_status(&id) }
        }),
    );

    let port = env::var("PORT").unwrap_or(String::from("3000"));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
