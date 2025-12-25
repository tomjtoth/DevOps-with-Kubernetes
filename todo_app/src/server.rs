use std::{env, fs, io::Write, path::Path, time::SystemTime};

use dioxus::prelude::debug;

pub(super) async fn replace_image_if_needed() -> std::io::Result<()> {
    let image_path = env::var("IMAGE_PATH").unwrap_or(String::from("public/data/image"));

    let metadata = fs::metadata(&image_path);
    if let Ok(metadata) = metadata {
        if let Ok(mtime) = metadata.modified() {
            if SystemTime::now().duration_since(mtime).unwrap().as_secs() > 5 {
                let _ = get_image(image_path).await;
            }
        }
    } else {
        let _ = get_image(image_path).await;
    }

    Ok(())
}

async fn get_image(image_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://picsum.photos/1200")
        .await?
        .error_for_status()?;

    let img = response.bytes().await?;

    let path = Path::new(&image_path);
    if let Some(parent) = path.parent() {
        debug!("creating dir: {}", parent.to_str().unwrap());
        fs::create_dir_all(parent)?;
    }

    let mut file = std::fs::File::create(image_path)?;

    file.write_all(&img)?;

    debug!("replaced image");

    Ok(())
}
