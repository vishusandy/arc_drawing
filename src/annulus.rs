mod translate;
use crate::{angle, Pos, Pt};

fn calc_line(slope: f64, int: i32, x: i32) -> i32 {
    (x as f64 * slope).round() as i32 + int
}

fn calc_slope(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
    (y2 as f64 - y1 as f64) / (x2 as f64 - x1 as f64)
}

#[derive(Clone, Debug)]
struct Edge {
    angle: f64,
    oct: u8,
    slope: f64,
    int: i32, // intercept
}
impl Edge {
    fn blank(angle: f64) -> Self {
        Self {
            angle,
            oct: angle::angle_to_octant(angle),
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
    fn slope(&self) -> f64 {
        self.slope
    }
    fn int(&self) -> i32 {
        self.int
    }
}

#[derive(Clone, Debug)]
pub struct Annulus {
    end: Edge,
    cur_start: Edge,
    cur_end: Edge,
    oct: u8,
    inr: Pos, // inner arc
    otr: Pos, // outer arc
    x: i32,
    c: Pt<i32>,
}
impl Annulus {
    pub fn new<T>(start_angle: T, end_angle: T, mut ri: i32, mut ro: i32, c: Pt<i32>) -> Self
    where
        T: crate::Angle,
    {
        let start_angle = crate::angle::normalize(start_angle.radians());
        let mut end_angle = crate::angle::normalize(end_angle.radians());
        if (start_angle - end_angle).abs() <= std::f64::EPSILON {
            end_angle = crate::angle::normalize(end_angle - crate::TINY);
        }
        Self::check_radii(&mut ri, &mut ro);

        let end_oct = angle::angle_to_octant(end_angle);
        let start_oct = angle::angle_to_octant(start_angle);

        let cur_end = if start_oct == end_oct && start_angle > end_angle {
            angle::octant_end_angle(start_oct)
        } else {
            end_angle
        };

        let mut a = Self::annulus(start_angle, cur_end, ri, ro, c);
        a.end = Edge::blank(end_angle);
        a
    }

    fn annulus(start_angle: f64, end_angle: f64, ri: i32, ro: i32, c: Pt<i32>) -> Self {
        let end_oct = angle::angle_to_octant(end_angle);
        let start_oct = angle::angle_to_octant(start_angle);

        let end = Edge::blank(end_angle);

        let mut cur_start = Edge::blank(start_angle);
        let ea = match start_oct == end_oct {
            true => end_angle,
            false => angle::octant_end_angle(start_oct),
        };
        let mut cur_end = Edge::blank(ea);

        let inr = Pos::new(cur_start.angle, cur_end.angle, cur_start.oct, ri, c);
        let otr = Pos::new(cur_start.angle, cur_end.angle, cur_start.oct, ro, c);

        cur_start.set_slope(inr.x, inr.y, otr.x, otr.y);
        cur_end.set_slope(inr.ex, inr.ey, otr.ex, otr.ey);

        Self {
            end,
            x: inr.x.min(otr.x),
            inr,
            otr,
            oct: start_oct,
            cur_start,
            cur_end,
            c,
        }
    }

    pub fn inner_end(&self) -> Pt<i32> {
        Pt::new(self.inr.ex, self.inr.ey)
    }

    pub fn outer_end(&self) -> Pt<i32> {
        Pt::new(self.otr.ex, self.otr.ey)
    }

    pub fn inner_start(&self) -> Pt<i32> {
        Pt::new(self.inr.x, self.inr.y)
    }

    pub fn outer_start(&self) -> Pt<i32> {
        Pt::new(self.otr.x, self.otr.y)
    }

    fn check_radii(a: &mut i32, b: &mut i32) {
        if a > b {
            std::mem::swap(a, b);
        }
    }

    fn next_octant(&mut self) -> bool {
        if self.x >= self.inr.ex && self.x >= self.otr.ex {
            self.oct = self.oct % 8 + 1; // Increment octant.  Wraps around to 1 if oct == 8
            let start = angle::octant_start_angle(self.oct);
            *self = Self::annulus(start, self.end.angle, self.inr.r, self.otr.r, self.c);
            true
        } else {
            false
        }
    }

    fn end(&self) -> bool {
        match self.oct == self.end.oct && self.x >= self.inr.ex && self.x >= self.otr.ex {
            true => self.cur_start.angle <= self.end.angle,
            false => false,
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
            (None, None) => (
                x,
                calc_line(self.cur_start.slope(), self.cur_start.int(), x),
                calc_line(self.cur_end.slope(), self.cur_end.int(), x),
            ),
            (inr, otr) => {
                let (slope, int) = match x <= self.inr.ex && x <= self.otr.ex {
                    true => self.cur_start.line(),
                    false => self.cur_end.line(),
                };

                let inr = inr.unwrap_or_else(|| {
                    self.otr.inc();
                    calc_line(slope, int, x)
                });

                let otr = otr.unwrap_or_else(|| {
                    self.inr.inc();
                    calc_line(slope, int, x)
                });

                (x, inr, otr)
            }
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
        let width = image.width();
        let height = image.height();
        for y in yo.min(yi)..=yo.max(yi) {
            let Pt { x, y } = translate::iter_to_real(x, y, self.oct, self.c).u32();
            if x < width && y < height {
                image.put_pixel(x, y, color)
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
            let (x, y1, y2) = self.step();
            let (x, y1, y2) = (x, y1.max(x), y2.max(x));
            self.put_line(x, y1, y2, image, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RADS;
    #[test]
    fn annulus() -> Result<(), image::ImageError> {
        crate::logger(log::LevelFilter::Debug);
        let mut image = crate::setup(crate::RADIUS);

        let ri = crate::RADIUS - 40;
        let ro = crate::RADIUS;
        let start = RADS * 1.5;
        let end = RADS * 6.8;
        let center = Pt::new(300, 300);

        imageproc::drawing::draw_hollow_circle_mut(
            &mut image,
            crate::CENTER.into(),
            ri,
            image::Rgba([0, 0, 255, 255]),
        );

        let mut an: Annulus = Annulus::new(start, end, ri, ro, center);
        let oct = an.cur_start.oct;
        log::info!("Annulus: {:#?}", an);

        let is = an.inner_start().iter_to_real(oct, crate::CENTER.into());
        let os = an.outer_start().iter_to_real(oct, crate::CENTER.into());

        an.draw(&mut image, image::Rgba([255, 0, 0, 255]));
        // debug!("{:#?}", an);

        if log::log_enabled!(log::Level::Trace) {
            image.put_pixel(
                is.x() as u32 + 1,
                is.y() as u32,
                image::Rgba([0, 255, 0, 255]),
            );
            image.put_pixel(
                os.x() as u32 + 1,
                os.y() as u32,
                image::Rgba([0, 255, 0, 255]),
            );
            let ie = an.inner_end().iter_to_real(oct, crate::CENTER.into());
            let oe = an.outer_end().iter_to_real(oct, crate::CENTER.into());
            image.put_pixel(
                ie.x() as u32 - 1,
                ie.y() as u32,
                image::Rgba([0, 255, 0, 255]),
            );
            image.put_pixel(
                oe.x() as u32 - 1,
                oe.y() as u32,
                image::Rgba([0, 255, 0, 255]),
            );
        }

        image.save("images/annulus.png")
    }
}
