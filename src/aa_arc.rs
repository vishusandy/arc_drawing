use crate::Pt;

struct FpArc {
    x: f64,
    y: f64,
    d: f64,
    r: f64,
    r2: f64,
    c: Pt<f64>,
    oct: u8,
}

impl FpArc {
    fn full(r: i32, c: Pt<i32>, oct: u8) -> Self {
        let r = r as f64;
        Self {
            x: 0.0,
            y: r as f64,
            d: 0.0,
            r,
            r2: r * r,
            c: c.f64(),
            oct,
        }
    }
    pub fn new(x: i32, r: i32, c: Pt<i32>) -> Self {
        todo!()
    }
}

impl Iterator for FpArc {
    type Item = (Pt<u32>, Pt<u32>, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.y {
            return None;
        }

        use std::ops::Rem;
        let o = (self.y.fract() * 255.0).round().rem(255.0) as u8;
        let a = Pt::new(self.x, self.y.floor());
        let b = Pt::new(self.x, self.y.floor() + 1.0);
        // let o = 0;
        println!("x={:.0} y={:.0} o={}", self.x, self.y, o);

        self.x += 1.0;
        self.y = (self.r2 - self.x * self.x).sqrt();

        Some((
            a.to_real(self.oct, self.c).u32(),
            b.to_real(self.oct, self.c).u32(),
            o,
        ))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fp_arc_iter() -> Result<(), image::ImageError> {
        use image::Pixel;
        let c: image::Rgba<u8> = image::Rgba([255, 0, 0, 255]);
        // let mut image = crate::setup(crate::RADIUS);
        let mut image = crate::guidelines();
        for (a, b, o) in FpArc::full(crate::RADIUS, crate::CENTER.into(), 7) {
            let c1 = image::Rgba([c[0], c[1], c[2], 255 - o]);
            let c2 = image::Rgba([c[0], c[1], c[2], o]);
            image.get_pixel_mut(a.x(), a.y()).blend(&c1);
            image.get_pixel_mut(b.x(), b.y()).blend(&c2);
        }
        image.save("images/fp_arc_iter.png")
    }
}
