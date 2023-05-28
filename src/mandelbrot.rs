use crate::{color::hsv_to_rgb, complex::ComplexNumber};

fn get_mandelbrot_iterations(c: ComplexNumber, max_iterations: u8) -> u8 {
    let mut z = ComplexNumber::null();
    for iteration in 0..max_iterations {
        // z_{n + 1} = z_n ^ 2 + c
        z = z.square().add(&c);
        if z.abs_square() > 42f64 {
            return iteration;
        }
    }
    // c is in the mandelbrot set
    max_iterations
}

pub fn get_mandelbrot_color(x: f64, y: f64, max_iterations: u8) -> [u8; 3] {
    let iterations = get_mandelbrot_iterations(ComplexNumber::new(x, y), max_iterations);
    if iterations == max_iterations {
        return [0, 0, 0];
    }

    hsv_to_rgb((iterations as f64) / (max_iterations as f64))
}
