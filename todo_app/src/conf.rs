use std::{env, sync::LazyLock};

use tokio::sync::Mutex;

pub(crate) static IMAGE_PATH: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new(env::var("IMAGE_PATH").unwrap_or(String::from("public/image"))));

pub(super) static IP: LazyLock<String> =
    LazyLock::new(|| env::var("IP").unwrap_or(String::from("127.0.0.1")));

pub(super) static PORT: LazyLock<String> =
    LazyLock::new(|| env::var("PORT").unwrap_or(String::from("8080")));

pub(crate) static CHANGE_INTERVAL: LazyLock<Mutex<u64>> = LazyLock::new(|| {
    let mut change_interval = 5;

    if let Ok(str) = env::var("CHANGE_INTERVAL") {
        if let Ok(x) = str.parse::<u64>() {
            change_interval = x;
        }
    }

    Mutex::new(change_interval)
});
