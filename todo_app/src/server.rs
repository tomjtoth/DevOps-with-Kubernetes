use std::{env, fs, io::Write, path::Path, time::SystemTime};

use crate::conf::IMAGE_PATH;

pub(super) fn replace_image_if_needed() {
    tokio::spawn(async {
        // 5 seconds for dev purposes
        let mut change_interval = 5;

        if let Ok(str) = env::var("CHANGE_INTERVAL") {
            if let Ok(x) = str.parse::<u64>() {
                change_interval = x;
            }
        }

        let metadata = fs::metadata(IMAGE_PATH.to_string());
        if let Ok(metadata) = metadata {
            if let Ok(mtime) = metadata.modified() {
                if SystemTime::now().duration_since(mtime).unwrap().as_secs() > change_interval {
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
        .await?
        .error_for_status()?;

    let img = response.bytes().await?;

    let path_as_str = IMAGE_PATH.to_string();
    let path = Path::new(&path_as_str);
    if let Some(parent) = path.parent() {
        if !fs::exists(&parent).unwrap() {
            fs::create_dir_all(parent)?;
            println!("created dir: {}", parent.to_str().unwrap());
        }
    }

    let mut file = std::fs::File::create(path_as_str)?;

    file.write_all(&img)?;

    println!("replaced image");

    Ok(())
}
