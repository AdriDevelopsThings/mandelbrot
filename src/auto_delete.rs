use std::{
    env,
    time::{Duration, SystemTime},
};

use tokio::fs::{metadata, read_dir, remove_file};

use crate::render::get_tiles_path;

async fn run_autodelete() {
    let max_auto_delete_protected_zoom_level = env::var("MAX_AUTO_DELETE_PROTECTED_ZOOM_LEVEL")
        .unwrap_or_else(|_| "3".to_string())
        .parse::<u32>()
        .unwrap()
        + 1;
    let protected_zoom_levels = (0..max_auto_delete_protected_zoom_level)
        .map(|z| z.to_string())
        .collect::<Vec<String>>();

    let tiles_path = get_tiles_path();
    let mut zooms = read_dir(tiles_path).await.unwrap();
    let mut n = 0;
    while let Some(zoom) = zooms.next_entry().await.unwrap() {
        if protected_zoom_levels.contains(&zoom.file_name().into_string().unwrap()) {
            continue;
        }
        let mut xs = read_dir(zoom.path()).await.unwrap();
        while let Some(x) = xs.next_entry().await.unwrap() {
            let mut ys = read_dir(x.path()).await.unwrap();
            while let Some(y) = ys.next_entry().await.unwrap() {
                if !y.file_name().into_string().unwrap().ends_with(".png") {
                    continue;
                }
                let meta = metadata(y.path()).await.unwrap();
                if let Ok(time) = meta.accessed() {
                    if SystemTime::now().duration_since(time).unwrap()
                        > Duration::from_secs(60 * 60 * 12)
                    {
                        n += 1;
                        remove_file(y.path()).await.unwrap();
                    }
                }
            }
        }
    }
    if n > 0 {
        println!("Removed {n} files automatically.")
    }
}

pub async fn start_autodelete_runs() {
    let disable_auto_delete = env::var("DISABLE_AUTO_DELETE").unwrap_or_else(|_| "false".to_string()).to_lowercase() == "true";
    if disable_auto_delete {
        return
    }
    loop {
        run_autodelete().await;
        tokio::time::sleep(Duration::from_secs(120)).await;
    }
}
