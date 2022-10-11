use super::end::End;
use super::AAPt;
use crate::angle::angle_to_quad;
use crate::Pt;

/// Draws an antialiased circular arc.
///
/// If the angles are floating-point numbers they are interpreted as radians.
/// Otherwise the angles are interpreted as degrees.
pub fn antialiased_arc<A, C, I>(
    image: &mut image::RgbaImage,
    start_angle: A,
    end_angle: A,
    radius: f64,
    center: C,
    color: image::Rgba<u8>,
) where
    A: crate::Angle,
    C: crate::pt::Point<f64>,
{
    AntialiasedArc::new(start_angle, end_angle, radius, center.pt()).draw(image, color);
}

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
    /// # Panic
    ///
    /// Will panic if `radius` is negative.
    pub fn new<A, P, T>(start: A, end: A, radius: T, center: P) -> Self
    where
        A: crate::Angle,
        P: crate::Point<T>,
        T: Into<f64>,
    {
        let radius = radius.into();
        let center = Pt::new(center.x().into(), center.y().into());

        if radius < 0.0 {
            panic!("Cannot specify a negative radius");
        }

        let start = crate::angle::normalize(start.radians());
        let mut end = crate::angle::normalize(end.radians());
        if (start - end).abs() <= std::f64::EPSILON {
            end = crate::angle::normalize(start - crate::TINY);
        }

        Self::arc(start, end, radius, center)
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

    /// Draw an antialiased arc by iterating over all of its pixels
    pub fn draw(self, image: &mut image::RgbaImage, color: image::Rgba<u8>) {
        for pt in self {
            pt.draw(image, color);
        }
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

/// Draw an arc by plotting each iteration on an image
#[cfg(test)]
fn draw(image: &mut image::RgbaImage, iter: AntialiasedArc, color: image::Rgba<u8>) {
    iter.draw(image, color);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aa_partial_iter() -> Result<(), image::ImageError> {
        use crate::RADS;
        crate::logger(crate::LOG_LEVEL);
        let mut image = crate::guidelines();

        let start = RADS * 5.8;
        let end = RADS * 7.4;
        let color = image::Rgba([255, 0, 0, 255]);

        let arc = AntialiasedArc::new(start, end, crate::RADIUS_F, crate::CENTER_F);
        draw(&mut image, arc, color);

        image.save("images/aa_partial.png")
    }

    #[test]
    fn aa_partial_draw() -> Result<(), image::ImageError> {
        use crate::RADS;
        crate::logger(crate::LOG_LEVEL);
        let mut image = crate::test::img::blank(Pt::new(crate::IMG_SIZE, crate::IMG_SIZE));

        let start = RADS * 0.0;
        let end = RADS * 8.0;
        let r = crate::RADIUS_F;
        let c = Pt::new(200.0, 200.0);
        let color = image::Rgba([255, 0, 0, 255]);

        let arc = AntialiasedArc::new(start, end, r, c);
        arc.draw(&mut image, color);

        image.save("images/aa_partial_draw.png")
    }
}
