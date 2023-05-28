use std::{
    env,
    fs::create_dir_all,
    path::{Path, MAIN_SEPARATOR},
};

use image::{ImageBuffer, Rgb};

use crate::mandelbrot::get_mandelbrot_color;

pub fn get_tiles_path() -> String {
    env::var("TILES_PATH").unwrap_or_else(|_| String::from("tiles"))
}

fn get_pixels_per_tile() -> u32 {
    env::var("PIXELS_PER_TILE").unwrap_or_else(|_| String::from("255")).parse()
        .expect("Error while parsing 'PIXELS_PER_TILE' environment variable. Environment variable must be a positive integer")
}

fn convert_cord(tile_cord: u32, tile_limit: u32) -> f64 {
    ((tile_cord as f64 / tile_limit as f64) * 4_f64) - 2_f64
}

pub fn render(x: u32, y: u32, zoom: u32) {
    let tiles: u32 = u32::pow(2, zoom);
    let width = 4f64 / tiles as f64;

    let x_path_raw = format!(
        "{}{MAIN_SEPARATOR}{zoom}{MAIN_SEPARATOR}{x}",
        get_tiles_path()
    );
    let x_path = Path::new(&x_path_raw);

    if !x_path.exists() {
        create_dir_all(x_path).unwrap();
    }

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

    let tile_path = format!("{x_path_raw}{MAIN_SEPARATOR}{y}.png");
    image.save(tile_path).unwrap();
}

pub fn get_tile_path(zoom: u32, x: u32, y: u32) -> String {
    format!(
        "{}{MAIN_SEPARATOR}{zoom}{MAIN_SEPARATOR}{x}{MAIN_SEPARATOR}{y}.png",
        get_tiles_path()
    )
}
