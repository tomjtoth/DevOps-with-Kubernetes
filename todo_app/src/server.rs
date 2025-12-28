use std::{fs, io::Write, path::Path, time::SystemTime};

use dioxus::fullstack::reqwest;

use crate::{
    conf::{CHANGE_INTERVAL, IMAGE_PATH},
    log2,
};

pub(super) fn replace_image_if_needed() {
    tokio::spawn(async {
        let metadata = fs::metadata(&*IMAGE_PATH);
        if let Ok(metadata) = metadata {
            if let Ok(mtime) = metadata.modified() {
                let diff = { *CHANGE_INTERVAL };

                if SystemTime::now().duration_since(mtime).unwrap().as_secs() > diff {
                    let _ = get_image().await;
                }
            }
        } else {
            let _ = get_image().await;
        }
    });
}

async fn get_image() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://picsum.photos/1200")
        .await
        .inspect_err(log2("posting todo to backend"))?
        .error_for_status()?;

    let img = response
        .bytes()
        .await
        .inspect_err(log2("response.bytes failed"))?;

    let path = Path::new(&*IMAGE_PATH);
    if let Some(parent) = path.parent() {
        if !fs::exists(&parent).inspect_err(log2(format!(
            "checking existence of {} failed",
            &parent.to_str().unwrap()
        )))? {
            fs::create_dir_all(parent)?;
            println!("created dir: {}", parent.to_str().unwrap());
        }
    }

    let mut file = fs::File::create(&*IMAGE_PATH)
        .inspect_err(log2(format!("creating {} failed", &*IMAGE_PATH)))?;

    file.write_all(&img)
        .inspect_err(log2(format!("failed to write to {}", &*IMAGE_PATH)))?;

    println!("replaced image");

    Ok(())
}
