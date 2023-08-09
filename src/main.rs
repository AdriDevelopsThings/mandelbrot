use std::{path::PathBuf, env};

use server::start_server;
use auto_delete::start_autodelete_runs;

mod color;
mod complex;
mod mandelbrot;
mod render;
mod server;
mod auto_delete;

#[tokio::main]
async fn main() {
    // Pregenerate some zoom levels
    tokio::spawn(async {
        let max_pregenerate_zoom_level = env::var("MAX_PREGENERATE_ZOOM_LEVEL").unwrap_or_else(|_| "4".to_string()).parse().expect("Invalid MAX_PREGENERATE_ZOOM_LEVEL");
        let generate_zooms: std::ops::Range<u32> = 0..max_pregenerate_zoom_level;
        for zoom in generate_zooms {
            let tiles: u64 = u64::pow(2, zoom);
            println!("Pregenerating tiles for zoom level {zoom}...");
            for x in 0..tiles {
                for y in 0..tiles {
                    let path = PathBuf::from(render::get_tile_path(zoom, x, y));
                    if !path.exists() {
                        render::render(x, y, zoom);
                    }
                }
            }
        }
    });
    println!("Starting server...");
    tokio::spawn(start_autodelete_runs());
    start_server().await;
}
