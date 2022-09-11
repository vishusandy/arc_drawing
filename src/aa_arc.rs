use crate::Pt;

struct FpArc {
    x: f64,
    y: f64,
    d: f64,
    r: f64,
    r2: f64,
    c: Pt<f64>,
}

impl FpArc {
    fn full(r: i32, c: Pt<i32>) -> Self {
        let r = r as f64;
        Self {
            x: 0.0,
            y: r as f64,
            d: 1.0 - r,
            r,
            r2: r * r,
            c: c.f64(),
        }
    }
    pub fn new(x: i32, r: i32, c: Pt<i32>) -> Self {
        todo!()
    }
}

impl Iterator for FpArc {
    type Item = Pt<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.y {
            return None;
        }
        let pt = Pt::new(self.x, self.y);
        self.x += 1.0;
        if self.d < 0.0 {
            self.d += 2.0 * self.x + 1.0;
        } else {
            self.y -= 1.0;
            self.d += 2.0 * (self.x - self.y) + 1.0;
        }
        Some(pt.to_real(7, self.c).u32())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fp_arc_iter() -> Result<(), image::ImageError> {
        let mut image = crate::setup(crate::RADIUS);
        for Pt { x, y } in FpArc::full(crate::RADIUS, crate::CENTER.into()) {
            image.put_pixel(x, y, image::Rgba([255, 0, 0, 255]));
        }
        image.save("images/fp_arc_iter.png")
    }
}
