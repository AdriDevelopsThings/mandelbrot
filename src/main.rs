use std::{env, path::{PathBuf, Path}};

use auto_delete::start_autodelete_runs;
use futures::future::join_all;
use server::start_server;
use tokio::fs::create_dir;

use crate::render::get_tiles_path;

mod auto_delete;
mod color;
mod complex;
mod mandelbrot;
mod render;
mod server;

#[tokio::main]
async fn main() {
    let tiles_path = get_tiles_path();
    let tiles_path = Path::new(&tiles_path);
    if !tiles_path.exists() {
        create_dir(tiles_path).await.expect("Error while creating tiles directory");
    }

    // Pregenerate some zoom levels
    tokio::spawn(async {
        let max_pregenerate_zoom_level = env::var("MAX_PREGENERATE_ZOOM_LEVEL")
            .unwrap_or_else(|_| "4".to_string())
            .parse::<u32>()
            .expect("Invalid MAX_PREGENERATE_ZOOM_LEVEL")
            + 1;
        let generate_zooms: std::ops::Range<u32> = 0..max_pregenerate_zoom_level;
        for zoom in generate_zooms {
            let tiles: u64 = u64::pow(2, zoom);
            println!("Pregenerating tiles for zoom level {zoom}...");
            let mut workers = Vec::new();
            for x in 0..tiles {
                let future = tokio::spawn(async move {
                    for y in 0..tiles {
                        let path = PathBuf::from(render::get_tile_path(zoom, x, y));
                        if !path.exists() {
                            render::render(x, y, zoom);
                        }
                    }
                });
                workers.push(future);
            }
            join_all(workers).await;
        }
        println!("Finished pregenerating tiles");
    });
    println!("Starting server...");
    tokio::spawn(start_autodelete_runs());
    start_server().await;
}
