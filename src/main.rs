use std::path::PathBuf;

use server::start_server;
use auto_delete::start_autodelete_runs;

mod color;
mod complex;
mod mandelbrot;
mod render;
mod server;
mod auto_delete;

const GENERATE_ZOOMS: std::ops::Range<u32> = 0..4;

#[tokio::main]
async fn main() {
    // Pregenerate some zoom levels
    for zoom in GENERATE_ZOOMS {
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
    println!("Starting server...");
    tokio::spawn(start_autodelete_runs());
    start_server().await;
}
