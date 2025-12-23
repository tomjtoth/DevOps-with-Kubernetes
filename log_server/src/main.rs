use std::{
    env,
    fs::File,
    io::{SeekFrom, prelude::*},
    sync::Arc,
};

use axum::{Router, extract::State, routing::get};

#[derive(Clone)]
struct AppState {
    log_file: Arc<File>,
    pong_file: Arc<File>,
}

#[tokio::main]
async fn main() {
    let log_path = env::var("LOG_PATH").unwrap_or(String::from("data/log"));
    let pong_path = env::var("PONG_PATH").unwrap_or(String::from("data/pong"));

    let app_state = AppState {
        log_file: Arc::new(
            File::options()
                .read(true)
                .open(&log_path)
                .expect(&format!(r#"unable to open LOG_PATH="{log_path}""#)),
        ),

        pong_file: Arc::new(
            File::options()
                .read(true)
                .open(&pong_path)
                .expect(&format!(r#"unable to open PONG_PATH="{pong_path}""#)),
        ),
    };

    let app = Router::new()
        .route(
            "/",
            get(
                |State(AppState {
                     mut log_file,
                     mut pong_file,
                 }): State<AppState>| async move {
                    let mut log_contents = String::new();
                    let _ = log_file.seek(SeekFrom::Start(0));
                    let _ = log_file.read_to_string(&mut log_contents);

                    let mut pong_contents = String::new();
                    let _ = pong_file.seek(SeekFrom::Start(0));
                    let _ = pong_file.read_to_string(&mut pong_contents);

                    let log_lines = Vec::from_iter(log_contents.trim().split('\n'));

                    let backup_line = &format!("no lines found in {}", &log_path);
                    let backup_ref = backup_line.as_ref();
                    let last_line = log_lines.get(log_lines.len() - 1).unwrap_or(&backup_ref);
                    format!("{}\nPing / Pongs: {}", last_line, pong_contents)
                },
            ),
        )
        .with_state(app_state);

    let port = env::var("PORT").unwrap_or(String::from("3000"));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &port))
        .await
        .unwrap();

    println!("LOG_SERVER listening at :{}/", &port);
    axum::serve(listener, app).await.unwrap();
}
