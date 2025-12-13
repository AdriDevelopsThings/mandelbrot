use std::{env, io::Cursor};

use image::{ImageBuffer, Rgb};

use crate::{data::Data, mandelbrot::get_mandelbrot_color};

fn get_pixels_per_tile() -> u32 {
    env::var("PIXELS_PER_TILE").unwrap_or_else(|_| String::from("255")).parse()
        .expect("Error while parsing 'PIXELS_PER_TILE' environment variable. Environment variable must be a positive integer")
}

pub fn get_key(zoom: u32, x: u64, y: u64) -> String {
    format!("{zoom}_{x}_{y}")
}

fn convert_cord(tile_cord: u64, tile_limit: u64) -> f64 {
    ((tile_cord as f64 / tile_limit as f64) * 4_f64) - 2_f64
}

pub async fn render(data: &Data, x: u64, y: u64, zoom: u32, ex: Option<i64>) -> Vec<u8> {
    let tiles: u64 = u64::pow(2, zoom);
    let width = 4f64 / tiles as f64;

    let start_x = convert_cord(x, tiles);
    let start_y = convert_cord(y, tiles);

    let pixels_per_tile = get_pixels_per_tile();

    let pixel_size = width / pixels_per_tile as f64;

    let image = ImageBuffer::from_fn(pixels_per_tile, pixels_per_tile, |image_x, image_y| {
        let mx = start_x + image_x as f64 * pixel_size;
        let my = start_y + image_y as f64 * pixel_size;

        let color = get_mandelbrot_color(mx, my, 255);
        Rgb(color)
    });

    let mut bytes = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
        .unwrap();
    data.set(&get_key(zoom, x, y), &bytes, ex).await;
    bytes
}
