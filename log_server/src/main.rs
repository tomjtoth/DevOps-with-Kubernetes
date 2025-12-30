use std::{
    env,
    fs::File,
    io::{SeekFrom, prelude::*},
    sync::Arc,
};

use axum::{Router, extract::State, routing::get};
use reqwest::Error;

#[derive(Clone)]
struct AppState {
    log_file: Arc<File>,
    file_content: String,
    message: String,
}

async fn fetch_pings() -> Result<String, Error> {
    let resp = reqwest::get("http://pingpong-svc:55555/pings")
        .await?
        .error_for_status()?;
    let text = resp.text().await?;
    Ok(text)
}

async fn root_handler(
    State(AppState {
        mut log_file,
        file_content,
        message,
    }): State<AppState>,
) -> String {
    let mut log_contents = String::new();
    let _ = log_file.seek(SeekFrom::Start(0));
    let _ = log_file.read_to_string(&mut log_contents);

    let pings = fetch_pings().await.unwrap_or("none, yet...".to_string());

    let log_lines = Vec::from_iter(log_contents.trim().split('\n'));

    let last_line = log_lines.last().unwrap_or(&"log is empty");

    format!(
        "file content: {}\nenv variable: {}\n{}\nPing / Pongs: {}",
        file_content, message, last_line, pings
    )
}

#[tokio::main]
async fn main() {
    let log_path = env::var("LOG_PATH").unwrap_or(String::from("data/log"));

    let mut file_content = String::new();

    File::options()
        .read(true)
        .open("information.txt")
        .expect("unable to open information.txt")
        .read_to_string(&mut file_content)
        .expect("could not read contents of information.txt");

    let app_state = AppState {
        log_file: Arc::new(
            File::options()
                .read(true)
                .open(&log_path)
                .expect(&format!(r#"unable to open LOG_PATH="{log_path}""#)),
        ),

        file_content,

        message: env::var("MESSAGE").expect("cannot see env var MESSAGE"),
    };

    let app = Router::new()
        .route("/", get(root_handler))
        .with_state(app_state);

    let port = env::var("PORT").unwrap_or(String::from("3000"));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &port))
        .await
        .unwrap();

    println!("LOG_SERVER listening at :{}/", &port);
    axum::serve(listener, app).await.unwrap();
}
