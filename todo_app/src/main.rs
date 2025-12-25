use dioxus::prelude::*;

mod conf;
#[cfg(feature = "server")]
mod server;

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        server::replace_image_if_needed();

        println!(
            "TODO app listening at http://{}:{}/",
            conf::IP.to_string(),
            conf::PORT.to_string()
        );

        Ok(dioxus::server::router(App))
    });
}

#[post("/check-on-image")]
async fn check_on_image() -> Result<()> {
    server::replace_image_if_needed();

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
