use dioxus::prelude::*;

#[cfg(feature = "server")]
mod conf;
#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
use dioxus::{fullstack::reqwest, server::axum::routing::get};

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        server::replace_image_if_needed();

        println!(
            "TODO app listening at http://{}:{}/",
            conf::IP.to_string(),
            conf::PORT.to_string()
        );

        Ok(dioxus::server::router(App)
            .route("/10min-image", get(server::serve_image))
            .route("/healthz", get(server::healthz)))
    });
}

#[post("/check-on-image")]
async fn check_on_image() -> Result<()> {
    println!("check_on_image()");
    server::replace_image_if_needed();

    Ok(())
}

fn log2<T: std::error::Error>(ops: impl std::fmt::Display) -> impl Fn(&T) {
    move |e| eprintln!("{ops}:\n\t{e}")
}

#[get("/todos")]
async fn get_todos() -> Result<Vec<String>> {
    let arr = reqwest::get(&*conf::BACKEND_URL)
        .await
        .inspect_err(log2("request to backend failed"))?
        .json::<Vec<String>>()
        .await
        .inspect_err(log2("parsing json failed"))?;

    Ok(arr)
}

#[post("/todos")]
async fn post_todo(todo: String) -> Result<String> {
    reqwest::Client::new()
        .post(&*conf::BACKEND_URL)
        .json(&todo)
        .send()
        .await
        .inspect_err(log2("posting todo to backend"))?;

    Ok(todo)
}

#[component]
pub fn App() -> Element {
    let todos_query_res = use_server_future(get_todos)?;
    use_server_future(check_on_image)?;

    let mut value = use_signal(|| String::new());
    let mut todos = use_signal(Vec::<String>::new);

    use_effect(move || {
        if let Some(Ok(list)) = todos_query_res() {
            todos.set(list);
        }
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        h1 { "The project App" }
        img { src: "/10min-image", class: "max-w-100" }
        form {
            onsubmit: move |ev| async move {
                ev.prevent_default();
                let todo = value.read().clone();
                if let Ok(todo) = post_todo(todo).await {
                    todos.push(todo);
                    value.set(String::new());
                }
            },
            input {
                value,
                max: 140,
                onchange: move |ev| value.set(ev.value()),
            }
            button { "Create todo" }
        }
        ul {
            for todo in todos.iter() {
                li { "{todo}" }
            }
        }
        h3 { "DevOps with Kubernetes 2025" }
    }
}
