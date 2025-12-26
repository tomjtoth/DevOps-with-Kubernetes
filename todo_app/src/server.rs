use std::{fs, io::Write, path::Path, time::SystemTime};

use crate::conf::{CHANGE_INTERVAL, IMAGE_PATH};

pub(super) fn replace_image_if_needed() {
    tokio::spawn(async {
        println!("within tokio::spawned block");

        let metadata = fs::metadata(&*IMAGE_PATH);
        if let Ok(metadata) = metadata {
            println!("within tokio::metadata OK");
            if let Ok(mtime) = metadata.modified() {
                let diff = { *CHANGE_INTERVAL };

                if SystemTime::now().duration_since(mtime).unwrap().as_secs() > diff {
                    let _ = get_image().await;
                }
            }
        } else {
            println!("within tokio::metadata errored");
            let _ = get_image().await;
        }
    });
}

async fn get_image() -> Result<(), Box<dyn std::error::Error>> {
    println!("get_image: reqwest::get");
    let response = reqwest::get("https://picsum.photos/1200")
        .await?
        .error_for_status()?;

    println!("get_image: response.bytes()");
    let img = response.bytes().await?;

    let path = Path::new(&*IMAGE_PATH);
    if let Some(parent) = path.parent() {
        println!("get_image: fs::exists(&parent)?");
        if !fs::exists(&parent)? {
            fs::create_dir_all(parent)?;
            println!("created dir: {}", parent.to_str().unwrap());
        }
    }

    println!("get_image: fs::File::create(path_as_str)?");
    let mut file = fs::File::create(&*IMAGE_PATH)?;

    println!("get_image: before file.write_all(&img)?;");
    file.write_all(&img)?;

    println!("replaced image");

    Ok(())
}
