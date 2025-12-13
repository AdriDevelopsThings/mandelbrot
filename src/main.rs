use std::env;

use futures::future::join_all;
#[cfg(feature = "pregenerate_progressbar")]
use indicatif::ProgressBar;
use server::start_server;

use crate::{data::Data, render::get_key};

mod color;
mod complex;
mod data;
mod mandelbrot;
mod render;
mod server;

#[tokio::main]
async fn main() {
    let data = Data::new(env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost".to_string()));

    // Pregenerate some zoom levels
    let p_data = data.clone();
    let pregenerate_future = tokio::spawn(async move {
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
                let c_data = p_data.clone();
                let future = tokio::spawn(async move {
                    for y in 0..tiles {
                        if !c_data.exists(&get_key(zoom, x, y)).await {
                            render::render(&c_data, x, y, zoom, None).await;
                        }
                    }
                });
                workers.push(future);
            }
            if cfg!(feature = "pregenerate_progressbar") {
                #[cfg(feature = "pregenerate_progressbar")]
                {
                    let progress_bar = ProgressBar::new(tiles);
                    for worker in workers {
                        worker.await.unwrap();
                        progress_bar.inc(1);
                    }
                    progress_bar.finish();
                }
            } else {
                join_all(workers).await;
            }
        }
        println!("Finished pregenerating tiles");
    });

    if !env::args().any(|x| x == *"--only-pregenerate") {
        println!("Starting server...");
        start_server(data).await;
    }
    pregenerate_future.await.unwrap();
}
