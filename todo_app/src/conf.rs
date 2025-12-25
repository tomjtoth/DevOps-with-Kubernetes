use std::{env, sync::LazyLock};

pub(crate) static IMAGE_PATH: LazyLock<String> =
    LazyLock::new(|| env::var("IMAGE_PATH").unwrap_or(String::from("public/image")));

pub(super) static IP: LazyLock<String> =
    LazyLock::new(|| env::var("IP").unwrap_or(String::from("127.0.0.1")));

pub(super) static PORT: LazyLock<String> =
    LazyLock::new(|| env::var("PORT").unwrap_or(String::from("8080")));
