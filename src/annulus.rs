use crate::pt::Pt;
const RADS: f64 = std::f64::consts::PI / 4.0; // range of a single octant
mod translate;

struct Edge {
    angle: f64,
    oct: u8,
    slope: f64,
}

struct Pos {
    x: i32,
    y: i32,
    d: i32,  // decision parameter
    ex: i32, // ending x coordinate
    r: i32,
}
impl Pos {
    fn new(start: f64, end: f64, oct: u8, r: i32, c: Pt<i32>) -> Self {
        todo!()
    }
    fn inc(&mut self) {
        self.x += 1;
        if self.d > 0 {
            self.y -= 1;
            self.d += 2 * (self.x - self.y) + 1;
        } else {
            self.d += 2 * self.x + 1;
        }
    }
    fn next_octant(&mut self) {}
}

struct Annulus {
    // ang: Angles,
    // oct: Octs,
    start: Edge,
    end: Edge,
    oct: u8,
    inr: Pos, // inner arc
    otr: Pos, // outer arc
    x: i32,
    c: Pt<i32>,
}
impl Annulus {
    fn test_blank(ri: i32, ro: i32, c: Pt<i32>) -> Self {
        let start = Edge {
            angle: 0.0,
            oct: 7,
            slope: 0.0,
        };
        let end = Edge {
            angle: 0.0,
            oct: 7,
            slope: 0.0,
        };
        let oct = 7;
        let inr = Pos {
            x: 0,
            y: ri,
            d: 1 - ri,
            ex: 0,
            r: ri,
        };
        let otr = Pos {
            x: 0,
            y: ro,
            d: 1 - ro,
            ex: 0,
            r: ro,
        };
        let x = 0;
        Self {
            start,
            end,
            oct,
            inr,
            otr,
            x,
            c,
        }
    }
    fn new(start_angle: f64, end_angle: f64, r: i32, c: Pt<i32>) -> Self {
        todo!()
    }
    fn next_octant(&mut self) -> bool {
        // self.inr.next_octant();
        // self.otr.next_octant();
        false
    }

    fn end(&self) -> bool {
        if self.x > self.inr.y && self.x > self.otr.y {
            true
        } else {
            false
        }
    }

    fn put_line(
        &self,
        x: i32,
        yi: i32,
        yo: i32,
        image: &mut image::RgbaImage,
        color: image::Rgba<u8>,
    ) {
        for y in yo.min(yi)..=yo.max(yi) {
            let Pt { x, y } = translate::iter_to_real(x, y, self.oct, self.c);
            image.put_pixel(x as u32, y as u32, color);
        }
    }

    pub fn draw(&mut self, image: &mut image::RgbaImage, color: image::Rgba<u8>) {
        loop {
            if self.end() {
                return;
            }
            if self.next_octant() {
                continue;
            }
            self.put_line(self.x, self.inr.y.max(self.x), self.otr.y, image, color);
            println!("x={} yi={} yo{}", self.x, self.inr.y, self.otr.y);
            self.inr.inc();
            self.otr.inc();

            self.x += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partial_arc() -> Result<(), image::ImageError> {
        let mut image = crate::setup(crate::RADIUS);
        let mut an = Annulus::test_blank(crate::RADIUS - 20, crate::RADIUS, crate::CENTER.into());
        an.draw(&mut image, image::Rgba([255, 0, 0, 255]));
        image.save("images/annulus.png")
    }
}
