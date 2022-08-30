pub fn arc_midpoint(mut image: image::RgbaImage, r: i32, c: (i32, i32)) -> image::RgbaImage {
    {
        // octant 1
        let mut y: f64 = 0.0;
        let mut x2 = (r * r) as f64;
        let mut x: f64 = r as f64;
        while x > -y {
            let x2n = x2 + 2.0 * y - 1.0; // next x2 value
            x = x2n.sqrt();
            image.put_pixel(
                (x.round() as i32 + c.0) as u32,
                (y.round() as i32 + c.1 - 1) as u32,
                image::Rgba([255, 0, 0, 255]),
            );
            y -= 1.0;
            x2 = x2n;
        }
        // octant 2 attempt
        // let mut y2 = y * y;
        // while x >= 0.0 {
        //     let y2n = y2 + 2.0 * x - 1.0; // next y2 value
        //     let y = -y2n.sqrt();
        //     image.put_pixel(
        //         (x.round() as i32 + c.0 - 1) as u32,
        //         (y.round() as i32 + c.1) as u32,
        //         image::Rgba([255, 0, 0, 255]),
        //     );
        //     x -= 1.0;
        //     y2 = y2n;
        // }
    }
    // {
    //     // octant 8
    //     let mut x2: f64 = (r * r) as f64;
    //     let mut x: f64 = x2.sqrt();
    //     let mut y: f64 = 0.0;
    //     while x > y {
    //         let xn = x2 - (2.0 * y) - 1.0;
    //         x = x2.sqrt();
    //         image.put_pixel(
    //             (x.round() as i32 + c.0) as u32,
    //             (y.round() as i32 + c.1) as u32,
    //             image::Rgba([255, 0, 0, 255]),
    //         );
    //         y += 1.0;
    //         x2 = xn;
    //     }
    // }

    image
}

pub fn arc_integer(mut image: image::RgbaImage, r: i32, c: (i32, i32)) -> image::RgbaImage {
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
            // d = ((x + 1) * (x + 1)) + (y as f64 - 0.5).powi(2).round() as i32 - (r * r);
        } else {
            y -= 1;
            d += 2 * (x - y) + 1;
            // d = ((x + 1) * (x + 1)) + (y as f64 - 0.5).powi(2).round() as i32 - (r * r);
        }
    }
    image
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_arc_midpoint() -> Result<(), image::ImageError> {
        let image = crate::setup(crate::RADIUS);
        arc_midpoint(image, crate::RADIUS, crate::CENTER).save("arc_midpoint.png")
    }
    #[test]
    fn test_arc_integer() -> Result<(), image::ImageError> {
        let image = crate::setup(crate::RADIUS);
        arc_integer(image, crate::RADIUS, crate::CENTER).save("arc_integer.png")
    }
}
