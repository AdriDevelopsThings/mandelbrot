use std::time::{Duration, SystemTime};

use tokio::fs::{read_dir, metadata, remove_file};

use crate::render::get_tiles_path;

const PROTECTED_ZOOM_LEVELS: [&str; 4] = ["0", "1", "2", "3"];

async fn run_autodelete() {
    let tiles_path = get_tiles_path();
    let mut zooms = read_dir(tiles_path).await.unwrap();
    let mut n = 0;
    while let Some(zoom) = zooms.next_entry().await.unwrap() {
        if PROTECTED_ZOOM_LEVELS.contains(&zoom.file_name().into_string().unwrap().as_str()) {
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
                    if SystemTime::now().duration_since(time).unwrap() > Duration::from_secs(60 * 60 * 12) {
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
    loop {
        run_autodelete().await;
        tokio::time::sleep(Duration::from_secs(120)).await;
    }
}