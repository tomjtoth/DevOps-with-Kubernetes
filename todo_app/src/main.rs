use dioxus::prelude::*;

#[cfg(feature = "server")]
mod conf;
#[cfg(feature = "server")]
mod server;

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

        Ok(dioxus::server::router(App).route(
            "/10min-image",
            dioxus::server::axum::routing::get(|| async {
                tokio::fs::read(&*conf::IMAGE_PATH).await.unwrap_or(vec![])
            }),
        ))
    });
}

#[post("/check-on-image")]
async fn check_on_image() -> Result<()> {
    println!("check_on_image()");
    server::replace_image_if_needed();

    Ok(())
}

#[component]
pub fn App() -> Element {
    use_server_future(check_on_image).unwrap();

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        h1 { "The project App" }
        img { src: "/10min-image", class: "max-w-100" }
        h3 { "DevOps with Kubernetes 2025" }
    }
}
