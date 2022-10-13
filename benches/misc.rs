pub fn warmup_arc(mut image: image::RgbaImage, r: i32, c: (i32, i32)) -> image::RgbaImage {
    let mut x = 0;
    let mut y = r;
    let mut d = 1 - r;
    while x < y {
        image.put_pixel(
            (x + c.0) as u32,
            (y + c.1) as u32,
            image::Rgba([255, 0, 192, 255]),
        );
        x += 1;
        if d <= 0 {
            d += 2 * x + 1;
        } else {
            y -= 1;
            d += 2 * (x - y) + 1;
        }
    }
    image
}
