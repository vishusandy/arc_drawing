mod bounds;
mod edge;
mod pos;

use crate::{angle, translate, Pt};
use bounds::Bounds;
use edge::Edge;
use pos::Pos;

/// Draws an arc from a given start angle to an end angle.
///
/// A floating-point angle will represent an angle in radians.  Integer types
/// will represent an angle in degrees.
///
/// # Examples
///
/// Draws an arc that goes across the top half of the image (0° to 180°):
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::conics::arc;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]); // red
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// let radius = 190;
/// let center = (200, 200);
/// let start = 0; // 0°
/// let end = 180; // 180°
/// arc(&mut image, start, end, radius, center, color);
/// ```
/// Integer numbers for angles are treated as degrees while floating-point numbers
/// are treated as radians.
///
/// This will draw the same image as above using radians (PI = 180°):
///
/// ```
/// # use image::{RgbaImage, Rgba};
/// # use freehand::conics::arc;
/// # let bg = Rgba([255, 255, 255, 255]); // white
/// # let color = Rgba([255, 0, 0, 255]); // red
/// # let mut image = RgbaImage::from_pixel(400, 400, bg);
/// # let radius = 190;
/// # let center = (200, 200);
/// let start = 0.0;
/// let end = std::f64::consts::PI;
/// arc(&mut image, start, end, radius, center, color);
/// ```
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

/// A structure for iterating over points in an arc.
///
/// Does not implement the `Iterator` trait because points for even octants would
/// be returned in reverse order.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::conics::Arc;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]); // red
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// /// An arc that goes across the top half of the image (0° to 180°)
/// let radius = 190;
/// let center = (200, 200);
/// let start = 0; // 0°
/// let end = 180; // 180°
///
/// /// Create the struct
/// let arc = Arc::new(start, end, radius, center);
///
/// /// Draw the struct
/// arc.draw(&mut image, color);
/// ```
#[derive(Clone, Debug)]
pub struct Arc {
    /// Current iteration position.
    pos: Pos,
    /// Angle and octant of the start edge
    start: Edge,
    /// Angle and octant of the end edge
    end: Edge,
    /// Center of the circular arc
    c: Pt<i32>,
    /// Radius of the arc
    r: i32,
    /// Used to determine when to iterate over all octants and back to the original octant.
    /// If `revisit` is true iteration will not immediately end when the octant is finished.
    /// This is set to true for the first octant when `start.oct == end.oct` and `start.angle > end.angle`
    revisit: bool,
}

impl Arc {
    /// Creates a new [`Arc`].
    ///
    /// Floating-point angles will represent an angle in radians.  Integer types
    /// will represent an angle in degrees.
    ///
    /// Negative angles are supported as well as angles larger than 360° (or
    /// larger than`2*PI` for radians).  Angles will be normalized into a range
    /// of 0..PI*2.
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # use freehand::conics::Arc;
    /// # let bg = Rgba([255, 255, 255, 255]); // white
    /// # let mut image = RgbaImage::from_pixel(400, 400, bg);
    ///
    /// let arc = Arc::new(0, 180, 190, (200, 200));
    /// ```
    pub fn new<A, T, C>(start_angle: A, end_angle: A, radius: T, center: C) -> Self
    where
        A: crate::Angle,
        T: Into<i32>,
        C: crate::pt::Point<T>,
    {
        let start = angle::normalize(start_angle.radians());
        let end = angle::normalize(end_angle.radians());

        let mut arc = Self::blank(start, end, radius, center);
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

    /// Draw the specified arc by iterating over its points.
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # use freehand::conics::Arc;
    ///
    /// # let bg = Rgba([255, 255, 255, 255]); // white
    /// # let mut image = RgbaImage::from_pixel(400, 400, bg);
    ///
    /// let arc = Arc::new(0, 180, 190, (200, 200));
    /// arc.draw(&mut image, Rgba([255, 0, 0, 255]));
    ///
    /// ```
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
    fn arc_draw() -> Result<(), image::ImageError> {
        crate::logger(crate::LOG_LEVEL);

        let r = 190;
        let c = (200, 200);
        let start = RADS * 1.8;
        let end = RADS * 0.5;

        let mut image = crate::setup(r);
        let arc = Arc::new(start, end, r, c);

        arc.draw(&mut image, image::Rgba([255, 0, 0, 255]));

        image.save("images/arc3.png")
    }
}
