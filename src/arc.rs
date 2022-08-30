pub fn arc_midpoint(mut image: image::RgbaImage, r: i32, c: (i32, i32)) -> image::RgbaImage {
    {
        let mut x2: f64 = (r * r) as f64;
        let mut x: f64 = x2.sqrt();
        let mut y: f64 = 0.0;
        while x > y {
            let xn = x2 - (2.0 * y) - 1.0;
            x = x2.sqrt();
            image.put_pixel(
                (x.round() as i32 + c.0) as u32,
                (y.round() as i32 + c.1) as u32,
                image::Rgba([255, 0, 0, 255]),
            );
            y += 1.0;
            x2 = xn;
        }
    }
    image
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_arc_midpoint() -> Result<(), image::ImageError> {
        let image = crate::setup(crate::R);
        arc_midpoint(image, crate::R, crate::CENTER).save("arc_midpoint.png")
    }
}
