use crate::{angle, translate, Pt};

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
        let start = Edge::new(start_angle, start_oct);
        let end = Edge::new(end_angle, end_oct);

        Self {
            pos: Pos::start(start_oct, r),
            start,
            end,
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

#[derive(Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
    d: i32,
    oct: u8,
    ex: Option<i32>,
}

impl Pos {
    fn new(oct: u8, bounds: Bounds, r: i32, c: Pt<i32>) -> Self {
        let (Pt { x, y }, d) = match bounds.start {
            None => (Pt::new(0, r), 1 - r),
            Some(a) => {
                let pt = Pt::from_radian(a, r, c).real_to_iter(oct, c.into());
                (pt.i32(), crate::calc_error(pt, r))
            }
        };

        let ex = bounds.end.map(|a| {
            Pt::from_radian(a, r, c)
                .real_to_iter(oct, c.into())
                .x()
                .round() as i32
        });

        Self { x, y, d, oct, ex }
    }

    fn stop(&self) -> bool {
        self.x >= self.ex.map(|ex| ex + 1).unwrap_or(self.y)
    }

    fn start(oct: u8, r: i32) -> Self {
        Self {
            x: 0,
            y: r,
            d: 1 - r,
            oct,
            ex: None,
        }
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
struct Edge {
    angle: f64,
    oct: u8,
}

impl Edge {
    fn new(angle: f64, oct: u8) -> Self {
        Edge { angle, oct }
    }
}

#[derive(Clone, Debug, Default)]
struct Bounds {
    start: Option<f64>,
    end: Option<f64>,
}

impl Bounds {
    fn new(start: Option<f64>, end: Option<f64>) -> Self {
        Self { start, end }
    }

    fn start_bounds(start_edge: &Edge, end_edge: &Edge, revisit: bool) -> Self {
        let start = Some(start_edge.angle);

        let end = match start_edge.oct == end_edge.oct && !revisit {
            true => Some(end_edge.angle),
            false => None,
        };

        match start_edge.oct % 2 == 0 {
            true => Bounds::new(end, start),
            false => Bounds::new(start, end),
        }
    }

    #[allow(clippy::self_named_constructors)]
    fn bounds(oct: u8, start_edge: &Edge, end_edge: &Edge, revisit: bool) -> Self {
        if oct != end_edge.oct {
            return Bounds::default();
        }

        let start = match oct == start_edge.oct && start_edge.oct != end_edge.oct {
            true => Some(start_edge.angle),
            false => None,
        };

        let end = match oct == end_edge.oct && !revisit {
            true => Some(end_edge.angle),
            false => None,
        };

        match oct % 2 == 0 {
            true => Bounds::new(end, start),
            false => Bounds::new(start, end),
        }
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
