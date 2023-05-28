pub fn hsv_to_rgb(hue: f64) -> [u8; 3] {
    let hue = (hue * 360f64) as i16;
    let x = ((1 - ((hue / 60) % 2 - 1).abs()) * 255) as u8;
    if hue < 60 {
        [255, x, 0]
    } else if hue < 120 {
        [x, 255, 0]
    } else if hue < 180 {
        [0, 255, x]
    } else if hue < 240 {
        [0, x, 255]
    } else if hue < 300 {
        [x, 0, 255]
    } else {
        [255, 0, x]
    }
}
