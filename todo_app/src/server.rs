use std::{fs, io::Write, path::Path, time::SystemTime};

use crate::conf::{CHANGE_INTERVAL, IMAGE_PATH};

pub(super) fn replace_image_if_needed() {
    tokio::spawn(async {
        // 5 seconds for dev purposes

        let image_path = { IMAGE_PATH.lock().await.clone() };

        let metadata = fs::metadata(&image_path);
        if let Ok(metadata) = metadata {
            if let Ok(mtime) = metadata.modified() {
                let diff = { *CHANGE_INTERVAL.lock().await };

                if SystemTime::now().duration_since(mtime).unwrap().as_secs() > diff {
                    let _ = get_image(&image_path).await;
                }
            }
        } else {
            let _ = get_image(&image_path).await;
        }
    });
}

async fn get_image(path_as_str: &String) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://picsum.photos/1200")
        .await?
        .error_for_status()?;

    let img = response.bytes().await?;

    let path = Path::new(&path_as_str);
    if let Some(parent) = path.parent() {
        if !fs::exists(&parent)? {
            fs::create_dir_all(parent)?;
            println!("created dir: {}", parent.to_str().unwrap());
        }
    }

    let mut file = std::fs::File::create(path_as_str)?;

    file.write_all(&img)?;

    println!("replaced image");

    Ok(())
}
