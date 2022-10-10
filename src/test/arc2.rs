use crate::{translate, Pt, RADS};

#[derive(Clone, Debug)]
struct Edge {
    angle: f64,
    oct: u8,
    x: i32,
}

impl Edge {
    fn new(angle: f64, oct: u8, r: i32, c: Pt<i32>) -> Self {
        Edge {
            angle,
            oct,
            x: Pt::from_radian(angle, r, c)
                .real_to_iter(oct, c.into())
                .i32()
                .x(),
        }
    }
}

#[derive(Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
    d: i32,
    oct: u8,
    ex: Option<i32>,
}

impl Pos {
    fn blank(oct: u8, r: i32) -> Self {
        Self {
            x: 0,
            y: r,
            d: 1 - r,
            oct,
            ex: None,
        }
    }

    fn at(oct: u8, angle: f64, r: i32, c: Pt<i32>) -> Self {
        let pt = Pt::from_radian(angle, r, c).real_to_iter(oct, c.into());
        Self {
            x: pt.x().round() as i32,
            y: pt.y().round() as i32,
            d: crate::calc_error(pt, r),
            oct,
            ex: None,
        }
    }

    fn with_ex(mut self, ex: i32) -> Self {
        self.ex = Some(ex);
        self
    }

    fn set_ex(&mut self, ex: i32) {
        self.ex = Some(ex);
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
}

#[derive(Clone, Debug)]
pub struct Arc {
    pos: Pos,
    start: Edge,
    end: Edge,
    c: Pt<i32>,
    r: i32,
    revisit: bool,
}

impl Arc {
    pub fn new<A, T, C>(start_angle: A, end_angle: A, r: T, c: C) -> Self
    where
        A: crate::Angle,
        T: Into<i32>,
        C: crate::pt::Point<T>,
    {
        let start = crate::angle::normalize(start_angle.radians());
        let end = crate::angle::normalize(end_angle.radians());

        let mut arc = Self::blank(start, end, r, c);

        let revisit = arc.start.angle > arc.end.angle;

        if arc.start.oct % 2 == 1 {
            arc.pos = Pos::at(arc.start.oct, arc.start.angle, arc.r, arc.c);
            // arc.pos.set_ex(arc.end.x);
        } else if arc.start.oct == arc.end.oct && arc.start.angle < arc.end.angle {
            #[cfg(test)]
            log::debug!("swap");
            // std::mem::swap(&mut arc.start, &mut arc.end);
            arc.pos.set_ex(arc.end.x);
        }

        arc.revisit = revisit;
        #[cfg(test)]
        arc.octant_range();
        arc
    }

    fn blank<T, C>(start_angle: f64, end_angle: f64, r: T, c: C) -> Self
    where
        T: Into<i32>,
        C: crate::pt::Point<T>,
    {
        let c = Pt::new(c.x().into(), c.y().into());
        let r = r.into();
        let start_oct = crate::angle::angle_to_octant(start_angle);
        let end_oct = crate::angle::angle_to_octant(end_angle);
        let start = Edge::new(start_angle, start_oct, r, c);
        let end = Edge::new(end_angle, end_oct, r, c);

        Self {
            pos: Pos::blank(start_oct, r).with_ex(start.x),
            start,
            end,
            c,
            r,
            revisit: start_oct == end_oct && start_angle < end_angle,
        }
    }

    // fn restart(&mut self) {
    //     self.pos.oct = self.pos.oct % 8 + 1;
    //     match self.pos.oct == self.end.oct {
    //         true => self.pos.ex = Some(self.end.x),
    //         false => self.pos.ex = None,
    //     }
    //     if self.pos.oct % 2 == 0 {
    //         self.pos.x = self.end.x;
    //     } else {
    //     }
    // }

    fn pos(&self, oct: u8) -> Pos {
        if oct != self.end.oct {
            Pos::blank(oct, self.r)
        } else if oct % 2 == 1 {
            Pos::blank(oct, self.r).with_ex(self.end.x)
        } else {
            Pos::at(oct, self.end.angle, self.r, self.c)
        }
    }

    fn end(&self) -> bool {
        if self.revisit || self.pos.oct != self.end.oct {
            return false;
        }

        match self.pos.oct % 2 == 0 {
            true => self.pos.x > self.pos.y,
            false => self.pos.x > self.end.x,
        }
    }

    fn next_octant(&self) -> bool {
        if self.pos.x >= self.pos.y {
            return true;
        }

        match self.pos.ex {
            None => false,
            Some(ex) => self.pos.x > ex,
        }
    }

    #[cfg(test)]
    fn octant_range(&self) {
        let ffd = self.r as f64 / std::f64::consts::SQRT_2;
        log::debug!(
            "Oct {} range: {}..{}",
            self.pos.oct,
            self.pos.x,
            self.pos.ex.unwrap_or(ffd.round() as i32)
        );
    }

    pub fn draw<I>(mut self, image: &mut I, color: I::Pixel)
    where
        I: image::GenericImage,
    {
        let mut i = 0;
        while i < 2000 {
            #[cfg(test)]
            if self.pos.oct == self.start.oct || self.pos.oct == self.end.oct {
                log::trace!(
                    "o={} x={} y={} ex={:?}",
                    self.pos.oct,
                    self.pos.x,
                    self.pos.y,
                    self.pos.ex
                );
            }

            if self.end() {
                break;
            }

            if self.next_octant() {
                self.pos = self.pos(self.pos.oct % 8 + 1);
                self.revisit = false;
                #[cfg(test)]
                {
                    log::debug!("oct={}: {:?}", self.pos.oct, self.pos);
                    self.octant_range();
                }

                continue;
            }

            let pt = self.pt();

            #[cfg(test)]
            if self.pos.oct == self.start.oct || self.pos.oct == self.end.oct {
                log::trace!("  pt={:?}", pt);
            }

            image.put_pixel(pt.x() as u32, pt.y() as u32, color);
            self.pos.inc();
            i += 1;
        }
        #[cfg(test)]
        if i == 2000 {
            log::debug!("\nEXITING INFINITE LOOP\n");
        }
    }

    fn pt(&self) -> Pt<i32> {
        let pt = Pt::new(self.pos.x, self.pos.y);
        translate::iter_to_real(pt.x(), pt.y(), self.pos.oct, self.c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc2_draw() -> Result<(), image::ImageError> {
        crate::logger(log::LevelFilter::Debug);

        let r = 190;
        let c = (200, 200);
        let start = RADS * 1.2;
        let end = RADS * 1.8;

        let mut image = crate::setup(r);
        let arc = Arc::new(start, end, r, c);
        let dbg_arc = arc.clone();
        arc.draw(&mut image, image::Rgba([255, 0, 0, 255]));
        {
            log::debug!("{:#?}", dbg_arc);
            let start = Pt::from_radian(dbg_arc.start.angle, dbg_arc.r, dbg_arc.c).u32();
            let start_iter = start.i32().real_to_iter(dbg_arc.start.oct, dbg_arc.c);
            let end = Pt::from_radian(dbg_arc.end.angle, dbg_arc.r, dbg_arc.c).u32();
            let end_iter = end.i32().real_to_iter(dbg_arc.end.oct, dbg_arc.c);
            log::debug!("start: \t{:?}\n\t {:?}", start, start_iter);
            log::debug!("end: \t{:?}\n\t {:?}", end, end_iter);
            image.put_pixel(start.x(), start.y(), image::Rgba([0, 255, 0, 255]));
            image.put_pixel(end.x(), end.y(), image::Rgba([0, 0, 0, 255]));
        }

        image.save("images/arc2.png")
    }
}
