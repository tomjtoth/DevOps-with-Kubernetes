use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
mod conf;
#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
use dioxus::{fullstack::reqwest, server::axum::routing::get};

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: i64,
    done: bool,
    task: String,
}

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
async fn get_todos() -> Result<Vec<Todo>> {
    let arr = reqwest::get(&*conf::BACKEND_URL)
        .await
        .inspect_err(log2("request to backend failed"))?
        .json()
        .await
        .inspect_err(log2("parsing json failed"))?;

    Ok(arr)
}

#[post("/todos")]
async fn post_todo(todo: String) -> Result<Todo> {
    let res = reqwest::Client::new()
        .post(&*conf::BACKEND_URL)
        .json(&todo)
        .send()
        .await
        .inspect_err(log2("posting todo to backend"))?;

    let todo = res.json::<Todo>().await?;
    Ok(todo)
}

#[put("/todos/:id")]
async fn mark_done(id: i64) -> Result<()> {
    reqwest::Client::new()
        .put(&*conf::BACKEND_URL)
        .json(&id)
        .send()
        .await
        .inspect_err(log2("marking todo done"))?;

    Ok(())
}

#[component]
pub fn App() -> Element {
    let todos_query_res: Resource<std::result::Result<Vec<Todo>, dioxus::CapturedError>> =
        use_server_future(get_todos)?;
    use_server_future(check_on_image)?;

    let mut value = use_signal(|| String::new());
    let mut todos = use_signal(Vec::<Todo>::new);

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
        h3 { "Todo" }
        ul {
            for todo in todos.iter() {
                if !todo.done {
                    button {
                        onclick: {
                            let id = todo.id.clone();
                            move |_| async move {
                                if mark_done(id).await.is_ok() {
                                    for mut todo in todos.iter_mut() {
                                        if todo.id == id {
                                            todo.done = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        },
                        "Mark as done"
                    }
                }
            }
        }
        h3 { "Done" }
        ul {
            for todo in todos.read().iter() {
                if todo.done {
                    li { key: "{todo.id}", "{todo.task}" }
                }
            }
        }
        h3 { "DevOps with Kubernetes 2025" }
    }
}
