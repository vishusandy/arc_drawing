mod bounds;
mod edge;
mod pos;

use crate::{angle, translate, Pt};
use bounds::Bounds;
use edge::Edge;
use pos::Pos;

pub fn arc<A, C, I, T>(
    image: &mut I,
    start_angle: A,
    end_angle: A,
    radius: T,
    center: C,
    color: I::Pixel,
) where
    A: crate::Angle,
    C: crate::pt::Point<T>,
    I: image::GenericImage,
    T: Into<i32>,
{
    Arc::new(start_angle, end_angle, radius, center).draw(image, color);
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
        let start = angle::normalize(start_angle.radians());
        let end = angle::normalize(end_angle.radians());

        let mut arc = Self::blank(start, end, r, c);
        let bounds = Bounds::start_bounds(&arc.start, &arc.end, arc.revisit);

        arc.pos = Pos::new(arc.start.oct, bounds, arc.r, arc.c);
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

        Self {
            pos: Pos::start(start_oct, r),
            start: Edge::new(start_angle, start_oct),
            end: Edge::new(end_angle, end_oct),
            c,
            r,
            revisit: start_oct == end_oct && start_angle > end_angle,
        }
    }

    fn restart(&mut self) {
        let oct = self.pos.oct % 8 + 1;
        let bounds = Bounds::bounds(oct, &self.start, &self.end, self.revisit);
        self.pos = Pos::new(oct, bounds, self.r, self.c);
        self.revisit = false;
    }

    fn end(&self) -> bool {
        self.pos.oct == self.end.oct && !self.revisit
    }

    pub fn draw<I>(mut self, image: &mut I, color: I::Pixel)
    where
        I: image::GenericImage,
    {
        loop {
            if self.pos.stop() {
                if self.end() {
                    break;
                } else {
                    self.restart();
                    continue;
                }
            }

            let pt = self.pt();
            image.put_pixel(pt.x() as u32, pt.y() as u32, color);
            self.pos.inc();
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
    use crate::RADS;

    #[test]
    fn arc3_draw() -> Result<(), image::ImageError> {
        crate::logger(log::LevelFilter::Debug);

        let r = 190;
        let c = (200, 200);
        let start = RADS * 1.8;
        let end = RADS * 0.5;

        let mut image = crate::setup(r);
        let arc = Arc::new(start, end, r, c);
        let dbg_arc = arc.clone();

        arc.draw(&mut image, image::Rgba([255, 0, 0, 255]));

        log::debug!("{:#?}", dbg_arc);

        image.save("images/arc3.png")
    }
}
