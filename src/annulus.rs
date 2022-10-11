mod edges;
mod pos;

pub(crate) mod translate;
use crate::pt::Point;
use crate::{angle, Pt};
use edges::Edge;
use pos::Pos;

/// Draws a partial annulus (filled donut).
///
/// If the angles are floating-point numbers they are interpreted as radians.
/// Otherwise the angles are interpreted as degrees.
pub fn annulus<A, C, I>(
    image: &mut I,
    start_angle: A,
    end_angle: A,
    inner_radius: i32,
    outer_radius: i32,
    center: C,
    color: I::Pixel,
) where
    A: crate::Angle,
    C: Point<i32>,
    I: image::GenericImage,
{
    Annulus::new(
        start_angle,
        end_angle,
        inner_radius,
        outer_radius,
        center.pt(),
    )
    .draw(image, color);
}

/// Represents an annulus (part of a filled donut shape) from a start angle to an end angle.
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
    /// Creates a new [`Annulus`].
    ///
    /// A floating-point angle will represent an angle in radians.  Integer types
    /// will represent an angle in degrees.
    ///
    /// Negative angles are supported as well as angles larger than 360° (or
    /// larger than`2*PI` for radians).  Angles will be normalized into a range
    /// of 0..PI*2.
    ///
    /// # Panic
    ///
    /// Will panic if either of the radii are negative.
    pub fn new<A, P>(
        start_angle: A,
        end_angle: A,
        mut inner_radius: i32,
        mut outer_radius: i32,
        center: P,
    ) -> Self
    where
        A: crate::Angle,
        P: crate::pt::Point<i32>,
    {
        let start_angle = crate::angle::normalize(start_angle.radians());
        let mut end_angle = crate::angle::normalize(end_angle.radians());
        if (start_angle - end_angle).abs() <= std::f64::EPSILON {
            end_angle = crate::angle::normalize(end_angle - crate::TINY);
        }
        Self::check_radii(&mut inner_radius, &mut outer_radius);

        let end_oct = angle::angle_to_octant(end_angle);
        let start_oct = angle::angle_to_octant(start_angle);

        let cur_end = if start_oct == end_oct && start_angle > end_angle {
            angle::octant_end_angle(start_oct)
        } else {
            end_angle
        };

        let mut a = Self::annulus(
            start_angle,
            cur_end,
            inner_radius,
            outer_radius,
            center.pt(),
        );
        a.end = Edge::blank(end_angle);
        a
    }

    #[allow(clippy::self_named_constructors)]
    /// An internal function used to create a new [`Annulus`].  The `new()` function
    /// should be used externally, which will also normalize angles and check the radii.
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

    /// Verify radii are not negative and swap if `inner < outer`.
    fn check_radii(inner: &mut i32, outer: &mut i32) {
        if inner.is_negative() | outer.is_negative() {
            panic!("Radii must not be negative");
        }
        if inner > outer {
            std::mem::swap(inner, outer);
        }
    }

    /// Switch to the next octant
    fn next_octant(&mut self) -> bool {
        if self.x > self.inr.ex && self.x > self.otr.ex {
            self.oct = self.oct % 8 + 1; // Increment octant.  Wraps around to 1 if oct == 8
            let start = angle::octant_start_angle(self.oct);
            *self = Self::annulus(start, self.end.angle, self.inr.r, self.otr.r, self.c);
            true
        } else {
            false
        }
    }

    /// Checks to see if the end has been reached
    fn end(&self) -> bool {
        match self.oct == self.end.oct && self.x > self.inr.ex && self.x > self.otr.ex {
            true => self.cur_start.angle <= self.end.angle,
            false => false,
        }
    }

    /// Retrieve the next points in the annulus.
    ///
    /// Returns a local x coordinate and two y coordinates (in iterator
    /// coordinates not image coordinates).
    fn step(&mut self) -> (i32, i32, i32) {
        let x = self.x;
        self.x += 1;

        match (self.inr.matching_y(x), self.otr.matching_y(x)) {
            (Some(inr), Some(otr)) => {
                self.inr.inc();
                self.otr.inc();
                (x, inr, otr)
            }
            (None, None) => (
                x,
                edges::calc_line(self.cur_start.slope(), self.cur_start.int(), x),
                edges::calc_line(self.cur_end.slope(), self.cur_end.int(), x),
            ),
            (inr, otr) => {
                let (slope, int) = match x <= self.inr.ex && x <= self.otr.ex {
                    true => self.cur_start.line(),
                    false => self.cur_end.line(),
                };

                let inr = inr.unwrap_or_else(|| {
                    self.otr.inc();
                    edges::calc_line(slope, int, x)
                });

                let otr = otr.unwrap_or_else(|| {
                    self.inr.inc();
                    edges::calc_line(slope, int, x)
                });

                (x, inr, otr)
            }
        }
    }

    /// Draw a line from the given iterator coordinates onto an image.
    fn put_line<I: image::GenericImage>(
        &self,
        x: i32,
        yi: i32,
        yo: i32,
        image: &mut I,
        color: I::Pixel,
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

    /// Draw the annulus
    pub fn draw<I>(mut self, image: &mut I, color: I::Pixel)
    where
        I: image::GenericImage,
    {
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
    use crate::test::color_in_image;
    use crate::RADS;

    #[test]
    fn annulus() -> Result<(), image::ImageError> {
        crate::logger(crate::LOG_LEVEL);
        let mut image = crate::setup(crate::RADIUS);

        let ri = crate::RADIUS - 40;
        let ro = crate::RADIUS;
        let start = RADS * 6.0;
        let end = RADS * 8.0;
        let center = Pt::new(200, 200);

        imageproc::drawing::draw_hollow_circle_mut(
            &mut image,
            crate::CENTER,
            ri,
            image::Rgba([0, 0, 255, 255]),
        );

        let an: Annulus = Annulus::new(start, end, ri, ro, center);
        an.draw(&mut image, image::Rgba([255, 0, 0, 255]));

        image.save("images/annulus.png")
    }

    #[test]
    fn annulus_overwrite_circles() {
        let mut image = crate::test::img::blank((400, 400));
        let error = image::Rgba([0, 0, 255, 255]);
        let color = image::Rgba([255, 0, 0, 255]);
        let ri = 140;
        let ro = 190;

        imageproc::drawing::draw_hollow_circle_mut(&mut image, (200, 200), ri, error);
        imageproc::drawing::draw_hollow_circle_mut(&mut image, (200, 200), ro, error);

        super::annulus(&mut image, 0.0, 8.0 * RADS, ri, ro, (200, 200), color);

        if let Some((x, y)) = color_in_image(&image, error) {
            let _ = image.save("images/tests/failed_annulus_overwrite_circles.png");
            panic!("{:?} found in image at ({}, {})", error.0, x, y);
        }
    }
}
