use crate::pt::Pt;
const RADS: f64 = std::f64::consts::PI / 4.0; // range of a single octant
mod translate;

#[derive(Clone, Debug)]
struct Edge {
    angle: f64,
    oct: u8,
    slope: f64,
    int: i32,
}
impl Edge {
    fn new(angle: f64) -> Self {
        Self {
            angle,
            oct: translate::angle_octant(angle),
            slope: 0.0,
            int: 0,
        }
    }
    fn set_slope(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.slope = calc_slope(x1, y1, x2, y2);
        self.int = (self.slope * (-x1 as f64) + y1 as f64).round() as i32;
    }
    fn line(&self) -> (f64, i32) {
        (self.slope, self.int)
    }
}

fn calc_slope(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
    (y2 as f64 - y1 as f64) / (x2 as f64 - x1 as f64)
}

#[derive(Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
    d: i32,  // decision parameter
    ex: i32, // ending x coordinate
    ey: i32, // ending y coordinate
    r: i32,
}
impl Pos {
    fn new(start: f64, end: f64, oct: u8, r: i32, c: Pt<i32>) -> Self {
        let start = Pt::from_radian(start, r, c.into()).real_to_iter(oct, c.into());
        let end = Pt::from_radian(end, r, c.into()).real_to_iter(oct, c.into());
        let Pt { x, y } = start.i32();
        let Pt { x: ex, y: ey } = end.i32();
        let d: i32 = ((start.x().round() as f64 + 1.0).powi(2)
            + (start.y().round() as f64 - 0.5).powi(2)
            - r.pow(2) as f64)
            .round() as i32;
        Self { x, y, d, ex, ey, r }
    }

    fn get_y(&self, x: i32) -> Option<i32> {
        if x == self.x {
            Some(self.y)
        } else {
            None
        }
    }

    fn y(&self, x: i32, slope: f64, int: i32) -> i32 {
        if x == self.x {
            self.y
        } else {
            let s = self.slope(x, slope, int);
            println!("y={}", s);
            s
        }
    }

    fn slope(&self, x: i32, slope: f64, int: i32) -> i32 {
        (x as f64 * slope).round() as i32 + int
    }

    fn inc(&mut self) {
        if self.x > self.ex {
            return;
        }
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

#[derive(Clone, Debug)]
pub struct Annulus {
    start: Edge,
    end: Edge,
    oct: u8,
    inr: Pos, // inner arc
    otr: Pos, // outer arc
    x: i32,
    c: Pt<i32>,
}
impl Annulus {
    pub fn new(mut start: f64, mut end: f64, mut ri: i32, mut ro: i32, c: Pt<i32>) -> Self {
        Self::check_angles(&mut start, &mut end);
        Self::check_radii(&mut ri, &mut ro);
        let mut start = Edge::new(start);
        let mut end = Edge::new(end);
        let inr = Pos::new(start.angle, end.angle, start.oct, ri, c);
        let otr = Pos::new(start.angle, end.angle, start.oct, ro, c);
        start.set_slope(inr.x, inr.y, otr.x, otr.y);
        end.set_slope(inr.ex, inr.ey, otr.ex, otr.ey);
        Self {
            x: inr.x.min(otr.x),
            inr,
            otr,
            oct: start.oct,
            start,
            end,
            c,
        }
    }

    fn check_angles(start: &mut f64, end: &mut f64) {
        if start > end {
            std::mem::swap(start, end);
        }
        if *start < 0.0 {
            *start = 0.0;
        }
        if *end >= 8.0 {
            *end = 8.0 - std::f64::EPSILON;
        }
    }

    fn check_radii(a: &mut i32, b: &mut i32) {
        if a > b {
            std::mem::swap(a, b);
        }
    }

    fn next_octant(&mut self) -> bool {
        // self.inr.next_octant();
        // self.otr.next_octant();
        false
    }

    fn end(&self) -> bool {
        if self.x >= self.inr.ex && self.x >= self.otr.ex {
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
        println!(
            "\tDraw: x={} yi={} yo={} drawing y=({}..={})",
            x,
            yi,
            yo,
            yo.min(yi),
            yo.max(yi)
        );
        for y in yo.min(yi)..=yo.max(yi) {
            let Pt { x, y } = translate::iter_to_real(x, y, self.oct, self.c);
            image.put_pixel(x as u32, y as u32, color);
        }
    }

    fn step(&mut self) -> (i32, i32, i32) {
        let x = self.x;
        self.x += 1;
        match (self.inr.get_y(x), self.otr.get_y(x)) {
            (Some(inr), Some(otr)) => {
                self.inr.inc();
                self.otr.inc();
                (x, inr, otr)
            }
            (None, None) => {
                panic!("(None, None) returned");
            }
            (inr, otr) => {
                let (slope, int) = if x <= self.inr.ex && x <= self.otr.ex {
                    println!("Edge = start");
                    println!("step x={}", x);
                    self.start.line()
                } else {
                    println!("Edge = end");
                    self.end.line()
                };

                let inr = inr.unwrap_or_else(|| {
                    self.otr.inc();
                    println!(
                        "\tinr=None -> y={}",
                        ((x) as f64 * slope).round() as i32 + int
                    );
                    ((x) as f64 * slope).round() as i32 + int
                });

                let otr = otr.unwrap_or_else(|| {
                    self.inr.inc();
                    println!(
                        "\totr=None -> y={}",
                        ((x) as f64 * slope).round() as i32 + int
                    );
                    ((x) as f64 * slope).round() as i32 + int
                });

                (x, inr, otr)
            }
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
            // println!(
            //     "\nx={} xi={} xo={} yi={} yo={}",
            //     self.x, self.inr.x, self.otr.x, self.inr.y, self.otr.y
            // );
            let (x, y1, y2) = self.step();
            println!("\tstep => x={} y1={} y2={}", x, y1, y2);
            self.put_line(x, y1, y2, image, color);
            imageproc::drawing::draw_hollow_circle_mut(
                image,
                self.c.into(),
                self.otr.r,
                image::Rgba([0, 0, 255, 255]),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn annulus() -> Result<(), image::ImageError> {
        let mut image = crate::setup(crate::RADIUS);
        // let mut an = Annulus::test_blank(crate::RADIUS - 20, crate::RADIUS, crate::CENTER.into());
        let mut an = Annulus::new(
            RADS * 6.2,
            RADS * 6.8,
            crate::RADIUS - 10,
            crate::RADIUS,
            crate::CENTER.into(),
        );
        println!("Annulus: {:#?}", an);
        an.draw(&mut image, image::Rgba([255, 0, 0, 255]));
        image.save("images/annulus.png")
    }
}
