mod end;

use crate::angle::angle_to_quad;
use crate::antialias::AAPt;
use crate::Pt;
use end::End;

/// Draws an antialiased circular arc.
///
/// If the angles are floating-point numbers they are interpreted as radians.
/// Otherwise the angles are interpreted as degrees.
///
/// See also: [`Draw::antialiased_arc`](crate::Draw::antialiased_arc)
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::conics::antialiased_arc;

/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]);
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// let radius = 190;
/// let center = (200, 200);
/// let start = 0; // 0°
/// let end = 180; // 180°
///
/// antialiased_arc(&mut image, start, end, radius, center, color);
///
/// ```
///
/// Integer numbers for angles are treated as degrees while floating-point numbers
/// are treated as radians.
///
/// This will draw the same image as above using radians (PI = 180°):
///
/// ```
/// # use image::{RgbaImage, Rgba};
/// # use freehand::conics::antialiased_arc;
/// # let bg = Rgba([255, 255, 255, 255]); // white
/// # let color = Rgba([255, 0, 0, 255]);
/// # let mut image = RgbaImage::from_pixel(400, 400, bg);
/// # let radius = 190;
/// # let center = (200, 200);
/// let start = 0.0;
/// let end = std::f64::consts::PI;
/// antialiased_arc(&mut image, start, end, radius, center, color);
/// ```
pub fn antialiased_arc<A, C, T>(
    image: &mut image::RgbaImage,
    start_angle: A,
    end_angle: A,
    radius: T,
    center: C,
    color: image::Rgba<u8>,
) where
    A: crate::Angle,
    C: crate::pt::Point<T>,
    T: Into<f64> + Copy,
{
    AntialiasedArc::new(start_angle, end_angle, radius, center).draw(image, color);
}

/// An antialiased arc.  Implements [`Iterator`] and returns coordinates in order from the starting point.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::conics::AntialiasedArc;

/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]);
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// let radius = 190;
/// let center = (200, 200);
/// let start = 0; // 0°
/// let end = 180; // 180°
///
/// let arc = AntialiasedArc::new(start, end, radius, center);
///
/// arc.draw(&mut image, color);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct AntialiasedArc {
    /// Current local x coordinate (not the same as the final pixel coordinates)
    x: f64,
    /// Current local y coordinate (not the same as the final pixel coordinates)
    y: f64,
    /// Radius
    r: f64,
    /// Radius squared
    r2: f64,
    /// Current quadrant
    quad: u8,
    /// End quadrant
    end_quad: u8,
    /// Whether to increment x (true) or y (false) every iteration.  Only used to make forty-five degree edges look nicer
    fast_x: bool,
    /// Used when start_angle > end_angle and start_quad == end_quad.  This allows it to  loop back around the circle
    revisit: bool,
    /// Where to stop
    end: End,
    /// Center coordinates
    c: Pt<f64>,
}
impl AntialiasedArc {
    /// Creates a new [`AntialiasedArc`].
    ///
    /// If the angles are floating-point numbers they are interpreted as radians.
    /// Otherwise the angles are interpreted as degrees.
    ///
    /// Negative angles are supported as well as angles larger than 360° (or
    /// larger than`2*PI` for radians).  Angles will be normalized into a range
    /// of 0..PI*2.
    ///
    /// # Panics
    ///
    /// Will panic if `radius` is negative.
    ///
    /// An antialiased arc.  Implements [`Iterator`] and returns coordinates in order from the starting point.
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # use freehand::conics::AntialiasedArc;

    /// # let bg = Rgba([255, 255, 255, 255]); // white
    /// # let mut image = RgbaImage::from_pixel(400, 400, bg);
    ///
    /// let radius = 190;
    /// let center = (200, 200);
    /// let start = 0; // 0°
    /// let end = 180; // 180°
    ///
    /// let arc = AntialiasedArc::new(start, end, radius, center);
    /// ```
    pub fn new<A, P, T>(start: A, end: A, radius: T, center: P) -> Self
    where
        A: crate::Angle,
        P: crate::Point<T>,
        T: Into<f64> + Copy,
    {
        let radius = radius.into();
        let center = Pt::new(center.x().into(), center.y().into());

        assert!(
            radius > 0.0,
            "Radius must be larger than 0.  radius={radius:.1}"
        );

        let start = crate::angle::normalize(start.radians());
        let mut end = crate::angle::normalize(end.radians());
        if (start - end).abs() <= std::f64::EPSILON {
            end = crate::angle::normalize(start - crate::TINY);
        }

        Self::arc(start, end, radius, center)
    }

    /// Draw an antialiased arc by iterating over all of its pixels
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # use freehand::conics::AntialiasedArc;

    /// # let bg = Rgba([255, 255, 255, 255]); // white
    /// # let mut image = RgbaImage::from_pixel(400, 400, bg);
    ///
    /// let arc = AntialiasedArc::new(0, 180, 190, (200, 200));
    /// arc.draw(&mut image, Rgba([255, 0, 0, 255]))
    /// ```
    pub fn draw(self, image: &mut image::RgbaImage, color: image::Rgba<u8>) {
        for pt in self {
            pt.draw(image, color);
        }
    }

    /// An internal function to create a new [`AntialiasedArc`] without normalizing
    /// angles or checking the radius.
    fn arc<T>(start_angle: T, end_angle: T, r: f64, c: Pt<f64>) -> Self
    where
        T: crate::Angle + std::fmt::Display,
    {
        let start_angle = start_angle.radians();
        let end_angle = end_angle.radians();
        let quad = angle_to_quad(start_angle);
        let end_quad = angle_to_quad(end_angle);
        let mut start = Pt::from_radian(start_angle, r, c).quad_to_iter(quad, c);
        let end = Pt::from_radian(end_angle, r, c).quad_to_iter(end_quad, c);
        let inc_x = if start.x() < start.y() {
            true
        } else {
            start.y = start.y.ceil();
            false
        };

        Self {
            x: start.x,
            y: start.y,
            r,
            r2: r * r,
            quad,
            end_quad,
            revisit: start_angle > end_angle && quad == end_quad,
            fast_x: inc_x,
            end: End::new(end),
            c,
        }
    }

    /// Advance in the x direction
    fn step_x(&mut self) -> Option<AAPt<i32>> {
        if (self.end_quad == self.quad) & self.end.match_x(self.x) {
            return None;
        }
        let x = self.x;
        let (ya, yb, da) = Self::calc_fract(self.y);
        let rst = AAPt::new(
            Pt::new(x, ya).iter_to_quad(self.quad, self.c).i32(),
            Pt::new(x, yb).iter_to_quad(self.quad, self.c).i32(),
            da,
        );
        self.x += 1.0;
        self.y = self.calc_slow(self.x);
        Some(rst)
    }

    /// Advance in the y direction
    fn step_y(&mut self) -> Option<AAPt<i32>> {
        if (self.end_quad == self.quad) & self.end.match_y(self.y) {
            return None;
        }
        let y = self.y;
        let (xa, xb, da) = Self::calc_fract(self.x);
        let rst = AAPt::new(
            Pt::new(xa, y).iter_to_quad(self.quad, self.c).i32(),
            Pt::new(xb, y).iter_to_quad(self.quad, self.c).i32(),
            da,
        );
        self.y -= 1.0;
        self.x = self.calc_slow(self.y);
        Some(rst)
    }

    /// Advance or end iteration
    fn step(&mut self) -> Option<AAPt<i32>> {
        if self.x <= self.y {
            self.step_x()
        } else if self.fast_x {
            // This is to handle the forty-five degree edge case
            self.fast_x = false;
            self.y = self.y.ceil();
            self.step_y().map(|o| o.mult_opac_a(0.5))
        } else {
            self.step_y()
        }
    }

    /// Check if iteration should move to next quadrant
    fn next_quad(&mut self) -> bool {
        if self.y < 0.0 {
            self.reset();
            true
        } else {
            false
        }
    }

    /// Check if iteration should end
    fn end(&mut self) -> bool {
        let last = (self.quad == self.end_quad) & (self.y <= 0.0);
        if self.revisit & last {
            self.revisit = false;
            self.reset();
            false
        } else {
            last
        }
    }

    /// Reset to beginning of next quadrant
    fn reset(&mut self) {
        self.x = 0.0;
        self.y = self.r;
        self.fast_x = true;
        self.quad = self.quad % 4 + 1;
    }

    /// Calculate the slow coordinate from the fast coordinate
    fn calc_slow(&self, fast: f64) -> f64 {
        (self.r2 - fast * fast).sqrt()
    }

    /// Returns the two slow coordinates to antialias and the distance between a and the actual arc (to be used for antialiasing)
    fn calc_fract(slow: f64) -> (f64, f64, f64) {
        let o = slow.fract();
        let a = slow.floor().abs();
        let b = a + 1.0;
        (a, b, o)
    }
}

impl Iterator for AntialiasedArc {
    type Item = AAPt<i32>;

    /// Iterate over points in an arc, returning the two corresponding points and their opacities
    fn next(&mut self) -> Option<Self::Item> {
        if self.end() {
            return None;
        }
        if self.next_quad() {
            return self.next();
        }
        self.step()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arc_aa() -> Result<(), image::ImageError> {
        use crate::RADS;
        crate::logger(crate::LOG_LEVEL);
        let mut image = crate::test::img::blank(Pt::new(crate::IMG_SIZE, crate::IMG_SIZE));

        let start = RADS * 0.8;
        let end = RADS * 7.4;
        let r = crate::RADIUS_F;
        let c = Pt::new(200.0, 200.0);
        let color = image::Rgba([255, 0, 0, 255]);

        let arc = AntialiasedArc::new(start, end, r, c);
        arc.draw(&mut image, color);

        image.save("images/arc_aa.png")
    }
}
