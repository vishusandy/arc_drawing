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
            d: 1.0 - r,
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
    type Item = (Pt<u32>, Pt<u32>, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.y {
            return None;
        }
        let a = Pt::new(self.x, self.y);
        let b = Pt::new(self.x, self.y + 1.0);
        let d: u32;
        self.x += 1.0;
        if self.d < 0.0 {
            self.d += 2.0 * self.x + 1.0;
            d = 0;
        } else {
            self.y -= 1.0;
            self.d += 2.0 * (self.x - self.y) + 1.0;
            d = 0;
        }
        Some((
            a.to_real(self.oct, self.c).u32(),
            b.to_real(self.oct, self.c).u32(),
            d,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fp_arc_iter() -> Result<(), image::ImageError> {
        let mut image = crate::setup(crate::RADIUS);
        for (Pt { x, y }, _, _) in FpArc::full(crate::RADIUS, crate::CENTER.into(), 7) {
            image.put_pixel(x, y, image::Rgba([255, 0, 0, 255]));
        }
        image.save("images/fp_arc_iter.png")
    }
}
