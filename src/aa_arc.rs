mod aa_oct;

pub(crate) mod aa_quad {
    use crate::Pt;
    use log::debug;

    pub struct AAPt<T> {
        a: Pt<T>,
        b: Pt<T>,
        da: f64, // distance to a (decimal of range: 0..=1.0)
        db: f64,
    }
    impl<T> AAPt<T> {
        fn new(a: Pt<T>, b: Pt<T>, da: f64) -> Self {
            // debug!("da={} -> db={}", da, 1.0 - da);
            Self {
                a,
                b,
                da,
                db: 1.0 - da,
            }
        }
        fn reduce_opac_b(self, i: f64) -> Self {
            Self {
                a: self.a,
                b: self.b,
                da: self.da,
                db: self.db * i,
            }
        }
    }

    impl AAPt<u32> {
        fn draw(&self, image: &mut image::RgbaImage, color: image::Rgba<u8>) {
            use image::Pixel;
            let c1 = alpha(opac(self.da), color);
            let c2 = alpha(opac(self.db), color);
            image.get_pixel_mut(self.a.x(), self.a.y()).blend(&c2);
            image.get_pixel_mut(self.b.x(), self.b.y()).blend(&c1);
        }
    }
    enum End {
        X(f64),
        Y(f64),
    }
    impl End {
        fn new(p: Pt<f64>) -> Self {
            if p.x() <= p.y() {
                Self::X(p.x())
            } else {
                Self::Y(p.y())
            }
        }

        fn match_x(&self, p: f64) -> bool {
            match self {
                Self::X(x) => (x - p).abs() <= std::f64::EPSILON,
                _ => false,
            }
        }

        fn match_y(&self, p: f64) -> bool {
            match self {
                Self::Y(y) => (y - p).abs() <= std::f64::EPSILON,
                _ => false,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct AAArc {
        x: f64,
        y: f64,
        r: f64,
        r2: f64,
        quad: u8,     // current quadrant
        end_quad: u8, // end quadrant
        inc_x: bool,  // whether to increment x (true) or increment y (false)
        // end: End,
        c: Pt<f64>,
    }
    impl AAArc {
        pub fn start(r: i32, c: Pt<i32>) -> Self {
            let r = r as f64;
            Self {
                x: 0.0,
                y: r,
                r,
                r2: r * r,
                // stop: (r / std::f64::consts::SQRT_2).round(),
                quad: 1,
                end_quad: 4,
                inc_x: true,
                c: c.f64(),
            }
        }

        /// Calculate the slow coordinate from the fast coordinate
        fn calc_slow(&self, fast: f64) -> f64 {
            (self.r2 - fast * fast).sqrt()
        }

        /// Returns the two slow coordinates to antialias and the distance between a and the actual arc (to be used for antialiasing)
        fn calc_fract(slow: f64) -> (f64, f64, f64) {
            let o = slow.fract();
            let a = slow.floor().abs();
            let b = a + 1.0;
            (a, b, o)
        }

        fn step_x(&mut self) -> AAPt<u32> {
            let (x, y) = (self.x, self.y);
            let (ya, yb, da) = Self::calc_fract(self.y);
            let rst = AAPt::new(
                Pt::new(x, ya).iter_to_quad(self.quad, self.c).u32(),
                Pt::new(x, yb).iter_to_quad(self.quad, self.c).u32(),
                da,
            );
            self.x += 1.0;
            self.y = self.calc_slow(self.x);
            rst
        }

        fn step_y(&mut self) -> AAPt<u32> {
            let (x, y) = (self.x, self.y);
            let (xa, xb, da) = Self::calc_fract(self.x);
            let rst = AAPt::new(
                Pt::new(xa, y).iter_to_quad(self.quad, self.c).u32(),
                Pt::new(xb, y).iter_to_quad(self.quad, self.c).u32(),
                da,
            );
            self.y -= 1.0;
            self.x = self.calc_slow(self.y);
            rst
        }

        fn step(&mut self) -> AAPt<u32> {
            let (x, y) = (self.x, self.y);
            if self.x <= self.y {
                self.step_x()
            } else {
                if self.inc_x {
                    // This is to handle the forty-five degree edge case
                    self.inc_x = false;
                    self.y = self.y.ceil();
                    debug!("switching");
                    // self.step_y().reduce_opac_a(0.0)
                    self.step_y().reduce_opac_b(0.5)
                } else {
                    self.step_y()
                }
            }
        }

        fn reset(&mut self) {
            self.x = 0.0;
            self.y = self.r;
            self.inc_x = true;
            self.quad = self.quad % 4 + 1;
        }

        fn next_quad(&mut self) -> bool {
            if self.y < 0.0 {
                self.reset();
                true
            } else {
                false
            }
        }

        fn end(&self) -> bool {
            self.quad == self.end_quad && self.y <= 0.0
        }

        pub fn draw(mut self, image: &mut image::RgbaImage, color: image::Rgba<u8>) {
            let ffd = (self.r / std::f64::consts::SQRT_2).round() as u32; //forty-five degree point
            loop {
                for _ in 0..ffd {
                    self.step_x().draw(image, color);
                }
                self.y = self.y.ceil();
                self.step_y().reduce_opac_b(0.5).draw(image, color);

                for _ in 1..ffd {
                    self.step_y().draw(image, color);
                }

                if self.end() {
                    break;
                }
                self.reset();
            }
        }
    }

    impl Iterator for AAArc {
        // type Item = (Pt<u32>, Pt<u32>, f64);
        type Item = AAPt<u32>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.end() {
                return None;
            }
            if self.next_quad() {
                return self.next();
            }
            Some(self.step())
        }
    }

    fn opac(d: f64) -> u8 {
        use std::ops::Rem;
        (d * 255.0).round().rem(256.0) as u8
    }

    fn alpha(a: u8, c: image::Rgba<u8>) -> image::Rgba<u8> {
        image::Rgba([c[0], c[1], c[2], a])
    }

    pub fn draw(image: &mut image::RgbaImage, iter: AAArc, color: image::Rgba<u8>) {
        for pt in iter {
            pt.draw(image, color);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn aa_iter() -> Result<(), image::ImageError> {
            crate::logger(log::LevelFilter::Debug);
            let mut image = crate::guidelines();
            let iter = AAArc::start(crate::RADIUS, crate::CENTER.into());
            let color = image::Rgba([255, 0, 0, 255]);
            draw(&mut image, iter, color);
            image.save("images/aa_iter.png")
        }

        #[test]
        fn aa_draw() -> Result<(), image::ImageError> {
            crate::logger(log::LevelFilter::Debug);
            let mut image = crate::guidelines();
            let color = image::Rgba([255, 0, 0, 255]);
            let iter = AAArc::start(crate::RADIUS, crate::CENTER.into());
            iter.draw(&mut image, color);
            image.save("images/aa_draw.png")
        }
    }
}
