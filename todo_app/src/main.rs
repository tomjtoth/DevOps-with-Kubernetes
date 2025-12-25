use dioxus::prelude::*;

#[cfg(feature = "server")]
mod server;

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        tokio::spawn(async { server::replace_image_if_needed().await });

        let ip = std::env::var("IP").unwrap_or(String::from("127.0.0.1"));
        let port = std::env::var("PORT").unwrap_or(String::from("8080"));
        println!("TODO app listening at {}:{}/", ip, port);

        Ok(dioxus::server::router(App))
    });
}

#[post("/check-on-image")]
async fn check_on_image() -> Result<()> {
    tokio::spawn(async { server::replace_image_if_needed().await });

    Ok(())
}

#[component]
pub fn App() -> Element {
    use_server_future(check_on_image).unwrap();

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        h1 { "The project App" }
        img { src: "/data/image", class: "max-w-100" }
        h3 { "DevOps with Kubernetes 2025" }
    }
}
