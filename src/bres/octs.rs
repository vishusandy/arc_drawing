pub(super) mod bres_to {
    pub(in super::super) fn o1<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (c.1, -c.0)
    }
    pub(in super::super) fn o2<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (c.0, -c.1)
    }
    pub(in super::super) fn o3<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (-c.0, -c.1)
    }
    pub(in super::super) fn o4<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (-c.1, -c.0)
    }
    pub(in super::super) fn o5<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (-c.1, c.0)
    }
    pub(in super::super) fn o6<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (-c.0, c.1)
    }
    pub(in super::super) fn o7<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
        c
    }
    pub(in super::super) fn o8<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (c.1, c.0)
    }
}

macro_rules! bres_oct {
    ( $o:ident, $x:ident, $y:ident, ( $ex:expr, $ey:expr ) ) => {
        #[derive(Clone, Debug)]
        pub struct $o {
            x: i32,
            y: i32,
            d: i32,
            c: (i32, i32),
        }
        impl $o {
            pub fn new(radius: i32, center: (i32, i32)) -> Self {
                Self {
                    x: 0,
                    y: radius,
                    d: 1 - radius,
                    c: center,
                }
            }
        }
        impl Iterator for $o {
            type Item = (i32, i32);
            fn next(&mut self) -> Option<Self::Item> {
                if self.x > self.y {
                    return None;
                }
                let $x = self.x;
                let $y = self.y;
                self.x += 1;
                if self.d > 0 {
                    self.y -= 1;
                    self.d += 2 * (self.x - self.y) + 1;
                } else {
                    self.d += 2 * self.x + 1;
                }
                Some(($ex + self.c.0, $ey + self.c.1))
            }
        }
    };
}

bres_oct!(Oct1, x, y, (y, -x));
bres_oct!(Oct2, x, y, (x, -y));
bres_oct!(Oct3, x, y, (-x, -y));
bres_oct!(Oct4, x, y, (-y, -x));
bres_oct!(Oct5, x, y, (-y, x));
bres_oct!(Oct6, x, y, (-x, y));
bres_oct!(Oct7, x, y, (x, y));
bres_oct!(Oct8, x, y, (y, x));

pub fn draw_bres_iter<T: Iterator<Item = (i32, i32)>>(
    image: &mut image::RgbaImage,
    iter: T,
    color: image::Rgba<u8>,
) {
    // let iter = Oct1::new(r, c);
    for (x, y) in iter {
        image.put_pixel(x as u32, y as u32, color);
    }
}

pub fn draw_bres_circle(
    image: &mut image::RgbaImage,
    r: i32,
    c: (i32, i32),
    color: image::Rgba<u8>,
) {
    draw_bres_iter(image, Oct1::new(r, c), color);
    draw_bres_iter(image, Oct2::new(r, c), color);
    draw_bres_iter(image, Oct3::new(r, c), color);
    draw_bres_iter(image, Oct4::new(r, c), color);
    draw_bres_iter(image, Oct5::new(r, c), color);
    draw_bres_iter(image, Oct6::new(r, c), color);
    draw_bres_iter(image, Oct7::new(r, c), color);
    draw_bres_iter(image, Oct8::new(r, c), color);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bres_macro_oct() -> Result<(), image::ImageError> {
        let mut image = crate::setup(crate::RADIUS);
        draw_bres_iter(
            &mut image,
            Oct1::new(crate::RADIUS, crate::CENTER),
            image::Rgba([255, 0, 0, 255]),
        );
        image.save("bres_macro_octant.png")
    }
    #[test]
    fn bres_macro_circle() -> Result<(), image::ImageError> {
        let mut image = crate::setup(crate::RADIUS);
        draw_bres_circle(
            &mut image,
            crate::RADIUS,
            crate::CENTER,
            image::Rgba([255, 0, 0, 255]),
        );
        image.save("bres_iter_circle.png")
    }
    #[test]
    fn pixel_test() -> Result<(), image::ImageError> {
        let mut image = crate::setup(crate::RADIUS);
        let len = 300;
        let rad = std::f64::consts::PI / 4.0;
        for i in 0..=len {
            let i = i as f64 / len as f64;
            let a = rad * i + rad;
            let p = crate::pt::Pt::from_radian(a, crate::CENTER, crate::RADIUS);
            let x = p.x().round() as u32;
            let y = p.y().round() as u32;

            let correct = image::Rgba([184, 120, 184, 255]);
            let incorrect = image::Rgba([255, 0, 0, 255]);
            let overwrite = image::Rgba([180, 180, 180, 255]);

            let color = match *image.get_pixel(x, y) {
                image::Rgba([255, 255, 255, 255]) => {
                    println!("a={:.2}\tx={:.2}\ty={:.2}", a, p.x(), p.y());
                    println!("\tx={}\t\ty={}\n", x, y);
                    incorrect
                }
                image::Rgba([184, 120, 184, 255]) | image::Rgba([180, 180, 180, 255]) => overwrite,
                _ => correct,
            };
            // let color = if *image.get_pixel(x, y) == image::Rgba([255, 255, 255, 255]) {
            //     println!("a={:.2}\tx={:.2}\ty={:.2}", a, p.x(), p.y());
            //     println!("\tx={}\t\ty={}\n", x, y);
            //     image::Rgba([255, 0, 0, 255])
            // } else {
            //     image::Rgba([200, 200, 200, 255])
            // };
            image.put_pixel(x, y, color);
        }
        image.save("pixel_test.png")
    }
}
