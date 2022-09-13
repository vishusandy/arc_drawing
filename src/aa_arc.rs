mod aa_oct;

mod aa_quad {
    use crate::Pt;

    #[derive(Clone, Debug)]
    struct Iter {
        x: f64,
        y: f64,
        r2: f64,
        // stop: f64,
        quad: u8, // needed to test for correct end point
        c: Pt<u32>,
    }
    impl Iter {
        fn start(r: i32, c: Pt<i32>) -> Self {
            let r = r as f64;
            Self {
                x: 0.0,
                y: r,
                r2: r * r,
                // stop: (r / std::f64::consts::SQRT_2).round(),
                quad: 4,
                c: c.u32(),
            }
        }

        // fn coords(&self) -> (Pt<u32>, Pt<u32>, u8) {
        //     use std::ops::Rem;
        //     let slow = (self.r2 - self.x * self.x).sqrt();
        //     let o = (slow.fract() * 255.0).round().rem(255.0) as u8;
        //     let a = slow.floor() as i32;
        //     let b = a as i32 + 1;
        //     let f = self.x as i32;
        //     match self.quad {
        //         4 => (
        //             (Pt::new(f, a) + self.c).u32(),
        //             (Pt::new(f, b) + self.c).u32(),
        //             o,
        //         ),
        //         1 => (
        //             (Pt::new(a, -f) + self.c).u32(),
        //             (Pt::new(b, -f) + self.c).u32(),
        //             o,
        //         ),
        //         _ => todo!("Need to add other octants"),
        //     }
        // }

        /// Calculate the slow coordinate from the fast coordinate
        fn calc_slow(&self, fast: f64) -> f64 {
            (self.r2 - fast * fast).sqrt()
        }

        /// Returns the two slow coordinates to antialias and the alpha value to use
        fn calc_fract(slow: f64) -> (u32, u32, f64) {
            // use std::ops::Rem;
            // let o = (slow.fract() * 255.0).round().rem(255.0) as u8;
            let o = slow.fract();
            let a = slow.floor().abs() as u32;
            let b = a + 1;
            (a, b, o)
        }

        fn step(&mut self) -> (Pt<u32>, Pt<u32>, f64) {
            let (x, y) = (self.x as u32, self.y as u32);
            let rst;
            if self.x <= self.y {
                let (a, b, o) = Self::calc_fract(self.y);
                println!("Oct7 x={} ya={} yb={} o={:.2}", self.x, a, b, o);
                rst = (Pt::new(x, a) + self.c, Pt::new(x, b) + self.c, o);
                self.x += 1.0;
                self.y = self.calc_slow(self.x);
            } else {
                let (a, b, o) = Self::calc_fract(self.x);
                println!("Oct8 y={} xa={} xb={} o={:.2}", self.y, a, b, o);
                rst = (Pt::new(a, y) + self.c, Pt::new(b, y) + self.c, o);
                self.y -= 1.0;
                self.x = self.calc_slow(self.y);
            }
            println!("    {:?}", rst);
            rst
        }
    }

    impl Iterator for Iter {
        type Item = (Pt<u32>, Pt<u32>, f64);
        fn next(&mut self) -> Option<Self::Item> {
            if self.y <= 0.0 {
                return None;
            }
            let rst = self.step();
            Some(rst)
        }
    }

    fn plot_aa(image: &mut image::RgbaImage, a: Pt<u32>, b: Pt<u32>, o: u8, c: image::Rgba<u8>) {
        use image::Pixel;
        let c1 = image::Rgba([c[0], c[1], c[2], 255 - o]);
        let c2 = image::Rgba([c[0], c[1], c[2], o]);
        image.get_pixel_mut(a.x(), a.y()).blend(&c1);
        image.get_pixel_mut(b.x(), b.y()).blend(&c2);
    }

    fn opac(d: f64) -> u8 {
        use std::ops::Rem;
        (d * 255.0).round().rem(256.0) as u8
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn aa_iter() -> Result<(), image::ImageError> {
            // let mut image = crate::setup(crate::RADIUS);
            let mut image = crate::guidelines();
            for (a, b, d) in Iter::start(crate::RADIUS, crate::CENTER.into()) {
                let o = opac(d);
                plot_aa(&mut image, a, b, o, image::Rgba([255, 0, 0, 255]));
                println!("    plot: o={} a={:?} b={:?}", o, a, b);
            }
            image.save("images/aa_iter.png")
        }
    }
}
