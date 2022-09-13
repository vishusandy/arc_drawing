mod aa_oct;

mod aa_quad {
    use crate::Pt;

    struct Iter {
        fast: f64,
        r2: f64,
        stop: f64,
        oct: u8, // needed to test for correct end point
        c: Pt<u32>,
    }
    impl Iter {
        fn start(r: i32, c: Pt<i32>) -> Self {
            let r = r as f64;
            Self {
                fast: 0.0,
                r2: r * r,
                stop: (r / std::f64::consts::SQRT_2).round(),
                oct: 7,
                c: c.u32(),
            }
        }

        fn coords(&self) -> (Pt<u32>, Pt<u32>, u8) {
            use std::ops::Rem;
            let slow = (self.r2 - self.fast * self.fast).sqrt();
            let o = (slow.fract() * 255.0).round().rem(255.0) as u8;
            let a = slow.floor() as u32;
            let b = a as u32 + 1;
            let f = self.fast as u32;
            match self.oct {
                7 => (Pt::new(f, a) + self.c, Pt::new(f, b) + self.c, o),
                _ => todo!("Need to add other octants"),
            }
        }
    }

    impl Iterator for Iter {
        type Item = (Pt<u32>, Pt<u32>, u8);
        fn next(&mut self) -> Option<Self::Item> {
            if self.fast >= self.stop {
                return None;
            }
            let rst = self.coords();
            self.fast += 1.0;
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

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn aa_iter() -> Result<(), image::ImageError> {
            let mut image = crate::guidelines();
            for (a, b, o) in Iter::start(crate::RADIUS, crate::CENTER.into()) {
                plot_aa(&mut image, a, b, o, image::Rgba([255, 0, 0, 255]));
            }
            image.save("images/aa_iter.png")
        }
    }
}
