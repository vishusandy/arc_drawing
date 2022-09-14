mod aa_oct;

pub(crate) mod aa_quad {
    use crate::Pt;
    use log::debug;

    #[derive(Clone, Debug)]
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

    #[derive(Copy, Clone, Debug)]
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

        fn r#match(&self, p: Pt<f64>) -> bool {
            match self {
                Self::X(x) => *x <= p.x,
                Self::Y(y) => *y >= p.y,
            }
        }

        fn match_x(&self, p: f64) -> bool {
            match self {
                Self::X(x) => *x <= p,
                _ => false,
            }
        }

        fn match_y(&self, p: f64) -> bool {
            match self {
                Self::Y(y) => *y >= p,
                _ => false,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct AAArc {
        /// Current local x coordinate (not the same as the final pixel coordinates)
        x: f64,
        /// Current local y coordinate (not the same as the final pixel coordinates)
        y: f64,
        /// Radius
        r: f64,
        /// Radius squared
        r2: f64,
        /// Current quadrant
        quad: u8,
        /// End quadrant
        end_quad: u8,
        /// Whether to increment x (true) or y (false) every iteration.  Only used to make forty-five degree edges look nicer
        fast_x: bool,
        /// Used when start_angle > end_angle and start_quad == end_quad.  This allows it to  loop back around the circle
        skip: bool,
        /// Where to stop
        end: End,
        /// Center coordinates
        c: Pt<f64>,
    }
    impl AAArc {
        pub fn new<T>(start: T, end: T, r: f64, c: Pt<f64>) -> Self
        where
            T: crate::Angle,
        {
            let start = start.radians() % std::f64::consts::PI * 2.0;
            let end = end.radians() % std::f64::consts::PI * 2.0;
            Self::arc(start, end, r, c)
        }

        pub fn arc<T>(start_angle: T, end_angle: T, r: f64, c: Pt<f64>) -> Self
        where
            T: crate::Angle + std::fmt::Display,
        {
            let start_angle = start_angle.radians();
            let end_angle = end_angle.radians();
            let quad = angle_to_quad(start_angle);
            let end_quad = angle_to_quad(end_angle);
            debug!("start_quad={} end_quad={}", quad, end_quad);
            let mut start = Pt::from_radian(start_angle, r, c.into()).quad_to_iter(quad, c);
            let end = Pt::from_radian(end_angle, r, c.into()).quad_to_iter(end_quad, c);
            let inc_x = if start.x() < start.y() {
                start.x = start.x;
                start.y = start.y;
                true
            } else {
                start.y = start.y.ceil();
                false
            };

            Self {
                x: start.x,
                y: start.y,
                r: r,
                r2: r * r,
                quad,
                end_quad,
                skip: start_angle > end_angle && quad == end_quad,
                fast_x: inc_x,
                end: End::new(end),
                c,
            }
        }

        pub fn draw(self, image: &mut image::RgbaImage, color: image::Rgba<u8>) {
            draw(image, self, color);
        }

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
                skip: false,
                fast_x: true,
                end: End::Y(0.0),
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

        fn step_x(&mut self) -> Option<AAPt<u32>> {
            if self.end_quad == self.quad && self.end.match_x(self.x) {
                return None;
            }
            let (x, y) = (self.x, self.y);
            let (ya, yb, da) = Self::calc_fract(self.y);
            let rst = AAPt::new(
                Pt::new(x, ya).iter_to_quad(self.quad, self.c).u32(),
                Pt::new(x, yb).iter_to_quad(self.quad, self.c).u32(),
                da,
            );
            self.x += 1.0;
            self.y = self.calc_slow(self.x);
            Some(rst)
        }

        fn step_y(&mut self) -> Option<AAPt<u32>> {
            if self.end_quad == self.quad && self.end.match_y(self.y) {
                return None;
            }
            let (x, y) = (self.x, self.y);
            let (xa, xb, da) = Self::calc_fract(self.x);
            let rst = AAPt::new(
                Pt::new(xa, y).iter_to_quad(self.quad, self.c).u32(),
                Pt::new(xb, y).iter_to_quad(self.quad, self.c).u32(),
                da,
            );
            self.y -= 1.0;
            self.x = self.calc_slow(self.y);
            Some(rst)
        }

        fn step(&mut self) -> Option<AAPt<u32>> {
            let (x, y) = (self.x, self.y);
            if self.x <= self.y {
                self.step_x()
            } else {
                if self.fast_x {
                    // This is to handle the forty-five degree edge case
                    self.fast_x = false;
                    self.y = self.y.ceil();
                    debug!("switching x={} y={}", self.x, self.y);
                    self.step_y().map(|o| o.reduce_opac_b(0.5))
                } else {
                    self.step_y()
                }
            }
        }

        fn reset(&mut self) {
            self.x = 0.0;
            self.y = self.r;
            self.fast_x = true;
            self.quad = self.quad % 4 + 1;
        }

        fn next_quad(&mut self) -> bool {
            if self.y < 0.0 {
                self.reset();
                debug!("Q={}", self.quad);
                true
            } else {
                false
            }
        }

        fn end(&mut self) -> bool {
            let last = self.quad == self.end_quad && self.y <= 0.0;
            if self.skip && last {
                self.skip = false;
                self.reset();
                false
            } else {
                last
            }
        }
    }

    impl Iterator for AAArc {
        type Item = AAPt<u32>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.end() {
                return None;
            }
            if self.next_quad() {
                return self.next();
            }
            log::trace!("x={:.2} y={:.2}", self.x, self.y);
            self.step()
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

    fn angle_to_quad(angle: f64) -> u8 {
        use crate::RADS;
        match angle {
            a if a < RADS * 2.0 => 1,
            a if a < RADS * 4.0 => 2,
            a if a < RADS * 6.0 => 3,
            a if a < RADS * 8.0 => 4,
            _ => panic!("invalid angle"),
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
            let arc = AAArc::start(crate::RADIUS, crate::CENTER.into());
            arc.draw(&mut image, color);
            image.save("images/aa_draw.png")
        }

        #[test]
        fn aa_partial_iter() -> Result<(), image::ImageError> {
            use crate::RADS;
            crate::logger(log::LevelFilter::Debug);
            let mut image = crate::guidelines();
            let start = RADS * 5.8;
            let end = RADS * 7.4;
            let r = crate::RADIUS as f64;
            let c = (crate::CENTER.0 as f64, crate::CENTER.1 as f64);
            let arc = AAArc::arc(start, end, r, c.into());
            debug!("ARC: {:#?}", arc);
            let color = image::Rgba([255, 0, 0, 255]);
            draw(&mut image, arc, color);
            image.save("images/aa_partial.png")
        }

        #[test]
        fn aa_partial_draw() -> Result<(), image::ImageError> {
            use crate::RADS;
            crate::logger(log::LevelFilter::Debug);
            let mut image = crate::guidelines();
            let start = RADS * 1.5;
            let end = RADS * 0.5;
            let r = crate::RADIUS as f64;
            let c = (crate::CENTER.0 as f64, crate::CENTER.1 as f64);
            debug!("FFD={:.2}", r / std::f64::consts::SQRT_2);
            let arc = AAArc::arc(start, end, r, c.into());
            debug!("ARC: {:#?}", arc);
            let color = image::Rgba([255, 0, 0, 255]);
            arc.draw(&mut image, color);
            image.save("images/aa_partial_draw.png")
        }
    }
}
