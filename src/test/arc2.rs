// use crate::{Pos, Pt};

// #[derive(Clone, Debug)]
// struct Arc {
//     pos: Pos,
//     oct: u8,
//     end_oct: u8,
//     end_angle: f64,
//     c: Pt<i32>,
// }

// impl Arc {}

#[cfg(test)]
mod tests {
    #[test]
    fn quad_test() -> Result<(), image::ImageError> {
        let mut image = crate::setup(crate::RADIUS);

        let r = crate::RADIUS;
        let radius = r as f64;
        let mid = (radius / std::f64::consts::SQRT_2).round() as usize;
        let c: crate::Pt<i32> = crate::CENTER.into();

        let mut x = 0;
        let mut y = r;
        let mut d = 1 - r;
        for _ in 0..mid {
            image.put_pixel(
                (x + c.x()) as u32,
                (y + c.y()) as u32,
                image::Rgba([255, 0, 0, 255]),
            );
            x += 1;
            if d > 0 {
                y -= 1;
                d += 2 * (x - y) + 1;
            } else {
                d += 2 * x + 1;
            }
        }
        // x -= 1;
        y -= 1;
        x += 1;
        while y != 0 {
            image.put_pixel(
                (x + c.x()) as u32,
                (y + c.y()) as u32,
                image::Rgba([255, 0, 0, 255]),
            );
            y -= 1;
            if d > 0 {
                x += 1;
                d += 2 * (y - x) + 1;
            } else {
                d += 2 * y + 1;
            }
        }

        image.save("images/quad_test.png")
    }
}
