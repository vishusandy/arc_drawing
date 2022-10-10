#[allow(dead_code)]
fn midpoint_algo(
    image: &mut image::RgbaImage,
    r: f64,
    c: freehand::Pt<u32>,
) -> Result<(), image::ImageError> {
    // let mut image = crate::test::img::blank((400, 400));
    let mut x = 0.0f64;
    let mut y = r as f64;
    let mut d = 1.0 - r;

    while x < y {
        image.put_pixel(
            x as u32 + c.x(),
            y as u32 + c.y(),
            image::Rgba([255, 0, 0, 255]),
        );
        x += 1.0;
        if d > 0.0 {
            y -= 1.0;
            d += 2.0 * x - 2.0 * y + 1.0;
        } else {
            d += 2.0 * x + 1.0;
        }
    }

    x = (r / std::f64::consts::SQRT_2).round();
    y = (r / std::f64::consts::SQRT_2).round() + 0.5;
    d = (x - 0.0) * (x - 0.0) + ((y + 0.0) * (y + 0.0)) - r * r;

    while y > 0.0 {
        image.put_pixel(
            x as u32 + c.x(),
            y as u32 + c.y(),
            image::Rgba([255, 0, 0, 255]),
        );
        y -= 1.0;
        // if x * x + y * y - r * r > 0.0 {
        if d > 0.0 {
            x += 1.0;
            d += 2.0 * y - 2.0 * x + 1.0;
        } else {
            d += 2.0 * y - 1.0;
        }
    }

    image.save("images/midpoint.png")
}

pub fn arc_midpoint(mut image: image::RgbaImage, radius: i32, c: (i32, i32)) -> image::RgbaImage {
    let r = radius as f64;
    let mut y: f64;
    let mut x: f64;
    {
        // Quadrant 1
        {
            // Octant 1
            x = r as f64;
            y = 0.0;
            let mut x2 = (r * r) as f64;
            while x > -y {
                // println!("x={:.1} y={:.1}", x, y);
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
        }
        {
            // Octant 2
            x = 0.0;
            y = -r;
            let mut y2 = -(r * r);
            // println!("x={:.1} y={:.1}", x, y);
            while x < -y {
                // println!("x={:.1} y={:.1}", x, y);
                let y2n = y2 + 2.0 * x - 1.0;
                y = -(-y2n).sqrt();
                image.put_pixel(
                    (x.round() as i32 + c.0) as u32,
                    (y.round() as i32 + c.1) as u32,
                    image::Rgba([255, 0, 0, 255]),
                );
                x += 1.0;
                y2 = y2n;
            }
        }
    }
    // Quadrant 2
    {
        {
            // Octant 3
            x = 0.0;
            y = -r as f64;
            let mut y2 = y * y;
            // println!("x={:.1} y={:.1}", x, y);
            while x > y {
                // println!("x={:.1} y={:.1}", x, y);
                let y2n = y2 + 2.0 * x - 1.0;
                y = -(y2n).sqrt();
                image.put_pixel(
                    (x.round() as i32 + c.0 - 1) as u32,
                    (y.round() as i32 + c.1) as u32,
                    image::Rgba([255, 0, 0, 255]),
                );
                x -= 1.0;
                y2 = y2n;
            }
        }
        {
            // Octant 4
            // println!("Starting octant 4");
            x = -r as f64;
            y = 0.0;
            let mut x2 = -(x * x);
            // println!("x={:.1} y={:.1}", x, y);
            while x < y {
                // println!("x={:.1} y={:.1}", x, y);
                let x2n = x2 - 2.0 * y - 1.0;
                x = -(-x2n).sqrt();
                image.put_pixel(
                    (x.round() as i32 + c.0) as u32,
                    (y.round() as i32 + c.1) as u32,
                    image::Rgba([255, 0, 0, 255]),
                );
                y -= 1.0;
                x2 = x2n;
            }
        }
    }
    {
        // Quadrant 3
    }

    {
        // octant 8
        let mut x: f64 = r;
        let mut y: f64 = 0.0;
        let mut x2: f64 = (r * r) as f64;
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
        } else {
            y -= 1;
            d += 2 * (x - y) + 1;
        }
    }
    image
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_arc_midpoint() -> Result<(), image::ImageError> {
//         let r = 240;
//         let c = (300, 300);
//         let image = crate::setup(r);
//         arc_midpoint(image, r, c).save("images/arc_midpoint.png")
//     }
//     #[test]
//     fn test_arc_integer() -> Result<(), image::ImageError> {
//         let r = 240;
//         let c = (300, 300);
//         let image = crate::setup(r);
//         arc_integer(image, r, c).save("images/arc_integer.png")
//     }
// }
