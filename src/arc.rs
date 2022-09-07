use crate::pt::Pt;

/// Range of a single octant.  This is equal to PI / 4.0
const RADS: f64 = std::f64::consts::PI / 4.0;

#[derive(Clone, Debug)]
struct Loc {
    r: i32,
    c: Pt<i32>,
}
impl Loc {
    fn new(r: i32, c: Pt<i32>) -> Self {
        Self { r, c }
    }
}

#[derive(Clone, Debug)]
pub struct Arc {
    loc: Loc,
    start_angle: f64,
    end_angle: f64,
    start_oct: u8,
    end_oct: u8,
    cur_oct: u8,
    x: i32,
    y: i32,
    d: i32,
    ex: i32,
}
impl Arc {
    /// Ensure angles are in the range 0..2*PI and that start >= end
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

    pub fn new(mut start: f64, mut end: f64, radius: i32, center: Pt<i32>) -> Self {
        Self::check_angles(&mut start, &mut end);
        let loc = Loc::new(radius, center);
        let start_oct = translate::angle_octant(start);
        let end_oct = translate::angle_octant(end);
        let (x, y, d) = if start_oct % 2 == 0 {
            if start_oct == end_oct {
                Self::calc_start(end, &loc, start_oct)
            } else {
                (0, loc.r, 1 - loc.r)
            }
        } else {
            Self::calc_start(start, &loc, start_oct)
        };
        let Pt { x: ex, y: ey } = if start_oct % 2 == 0 {
            Self::calc_end_x(start, &loc, start_oct)
        } else {
            Self::calc_end_x(end, &loc, end_oct)
        };
        Self {
            loc,
            start_angle: start,
            end_angle: end,
            start_oct,
            end_oct,
            cur_oct: start_oct,
            x,
            y,
            d,
            ex,
        }
    }

    fn calc_start(start: f64, loc: &Loc, oct: u8) -> (i32, i32, i32) {
        let c = loc.c.f64();
        let pt = Pt::from_radian(start, loc.r, loc.c.into()).real_to_iter(oct, loc.c.into());
        let d: i32 = ((pt.x().round() as f64 + 1.0).powi(2) + (pt.y().round() as f64 - 0.5).powi(2)
            - loc.r.pow(2) as f64)
            .round() as i32;
        let Pt { x, y } = pt.i32();
        (x, y, d)
    }

    fn calc_end_x(end: f64, loc: &Loc, oct: u8) -> Pt<i32> {
        let c = loc.c.f64();
        Pt::from_radian(end, loc.r, loc.c.into())
            .real_to_iter(oct, loc.c.into())
            .i32()
    }

    // Only account for cases where start and end are in different octants.
    // cases where start_oct == end_oct should be handled by the new function
    // `ex` should be calculated in new as well
    fn next_octant(&mut self) {
        self.cur_oct += 1;
        if self.cur_oct == self.end_oct && self.cur_oct % 2 == 0 {
            let a = self.end_angle;
            let (x, y, d) = Self::calc_start(a, &self.loc, self.cur_oct);
            self.x = x;
            self.y = y;
            self.d = d;
            self.ex = std::i32::MAX;
        } else if self.cur_oct == self.end_oct {
            let Pt { x, y: _ } = Self::calc_end_x(self.end_angle, &self.loc, self.end_oct);
            self.ex = x;
            self.restart();
        } else {
            self.restart();
        }
    }

    // Resets values to the beginning of the octant
    fn restart(&mut self) {
        self.x = 0;
        self.y = self.loc.r;
        self.d = 1 - self.loc.r;
    }

    pub fn draw(&mut self, image: &mut image::RgbaImage, color: image::Rgba<u8>) {
        loop {
            if self.x > self.y {
                if self.end_oct == self.cur_oct || self.cur_oct == 8 {
                    return; // end of arc reached
                } else {
                    self.next_octant();
                    continue; // end of octant reached, continue to next octant
                }
            }
            if self.x == self.ex
                && self.cur_oct == self.start_oct
                && self.start_oct % 2 == 0
                && self.cur_oct != self.end_oct
            {
                self.next_octant();
                continue; // arc starts on an odd octant, ensure we only the part of the octant (finish at ex)
            }
            if self.x == self.ex && self.cur_oct == self.end_oct {
                return;
            }

            self.put_pixel(image, color);
            self.x += 1;
            if self.d > 0 {
                self.y -= 1;
                self.d += 2 * (self.x - self.y) + 1;
            } else {
                self.d += 2 * self.x + 1;
            }
        }
    }

    fn put_pixel(&self, image: &mut image::RgbaImage, color: image::Rgba<u8>) {
        let pt = translate::iter_to_real(self.x, self.y, self.cur_oct, self.loc.c);
        image.put_pixel(pt.x as u32, pt.y as u32, color);
    }
}

mod translate {
    use crate::pt::Pt;
    pub(super) fn iter_to_real(x: i32, y: i32, oct: u8, c: Pt<i32>) -> Pt<i32> {
        match oct {
            1 => (y + c.x(), -x + c.y()).into(),
            2 => (x + c.x(), -y + c.y()).into(),
            3 => (-x + c.x(), -y + c.y()).into(),
            4 => (-y + c.x(), -x + c.y()).into(),
            5 => (-y + c.x(), x + c.y()).into(),
            6 => (-x + c.x(), y + c.y()).into(),
            7 => (x + c.x(), y + c.y()).into(),
            8 => (y + c.x(), x + c.y()).into(),
            _ => (x + c.x(), y + c.y()).into(),
        }
    }
    pub(super) fn real_to_iter<T>(mut x: T, mut y: T, oct: u8, c: Pt<T>) -> Pt<T>
    where
        T: Copy + std::ops::Neg<Output = T> + std::ops::SubAssign,
    {
        x -= c.x();
        y -= c.y();
        match oct {
            1 => Pt::new(-y, x),
            2 => Pt::new(x, -y),
            3 => Pt::new(-x, -y),
            4 => Pt::new(-y, -x),
            5 => Pt::new(y, -x),
            6 => Pt::new(-x, y),
            7 => Pt::new(x, y),
            8 => Pt::new(y, x),
            _ => Pt::new(x, y),
        }
    }
    pub(super) fn angle_octant(angle: f64) -> u8 {
        use super::RADS;
        if angle < RADS {
            return 1;
        }
        if angle < RADS * 2.0 {
            return 2;
        }
        if angle < RADS * 3.0 {
            return 3;
        }
        if angle < RADS * 4.0 {
            return 4;
        }
        if angle < RADS * 5.0 {
            return 5;
        }
        if angle < RADS * 6.0 {
            return 6;
        }
        if angle < RADS * 7.0 {
            return 7;
        }
        if angle < RADS * 8.0 {
            return 8;
        }
        panic!("Invalid angle {:.4}", angle);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partial_arc() -> Result<(), image::ImageError> {
        let mut image = crate::setup(crate::RADIUS);
        let mut arc = Arc::new(RADS * 1.1, RADS * 2.5, crate::RADIUS, crate::CENTER.into());
        arc.draw(&mut image, image::Rgba([255, 0, 0, 255]));
        image.save("images/arc_partial.png")
    }
}
