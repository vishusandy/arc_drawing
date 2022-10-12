use crate::pt::{Point, Pt};
use image::GenericImage;

/// Draws a straight horizontal line.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::lines::horizontal_line;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]);
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// /// Horizontal line across the center of the image
/// horizontal_line(&mut image, (0, 200), 399, color);
/// ```
pub fn horizontal_line<I, P>(image: &mut I, pt: P, x2: u32, color: I::Pixel)
where
    I: GenericImage,
    P: Point<u32>,
{
    if pt.y() < image.height() {
        (pt.x().min(image.width() - 1)..=x2.min(image.width() - 1))
            // This is safe due to the min() calls above
            .for_each(|x| unsafe { image.unsafe_put_pixel(x, pt.y(), color) });
    }
}

/// Draws a straight vertical line.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::lines::vertical_line;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]);
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// /// Vertical line across the center of the image
/// vertical_line(&mut image, (200, 0), 399, color);
/// ```
pub fn vertical_line<I, P>(image: &mut I, pt: P, y2: u32, color: I::Pixel)
where
    I: GenericImage,
    P: Point<u32>,
{
    if pt.x() < image.width() {
        (pt.y().min(image.height() - 1)..=y2.min(image.height() - 1))
            // This is safe due to the min() calls above
            .for_each(|y| unsafe { image.unsafe_put_pixel(pt.x(), y, color) });
    }
}

/// Draws a straight diagonal line between two points.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::lines::diagonal_line;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]);
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// /// Downwards diagonal line across the image
/// diagonal_line(&mut image, (0, 0), (399, 399), color);
/// /// Upwards diagonal line across the image
/// diagonal_line(&mut image, (0, 399), (399, 0), color);
/// ```
pub fn diagonal_line<I, P>(image: &mut I, mut a: P, mut b: P, color: I::Pixel)
where
    I: GenericImage,
    P: Point<u32>,
{
    if a.x() > b.x() {
        std::mem::swap(&mut a, &mut b);
    }

    if a.x() >= image.width() || a.y().min(b.y()) >= image.height() {
        return;
    }

    let x0 = a.x().min(image.width() - 1);
    let y0 = a.y().min(image.height() - 1);
    let x1 = b.x().min(image.width() - 1);
    let y1 = b.y().min(image.height() - 1);

    if y0 < y1 {
        let dist = (x1 - x0).min(y1 - y0);
        // This is safe due to the min calls above
        (0..=dist).for_each(|i| unsafe { image.unsafe_put_pixel(x0 + i, y0 + i, color) });
    } else {
        let dist = (x1 - x0).min(y0 - y1);
        // This is safe due to the min calls above
        (0..=dist).for_each(|i| unsafe { image.unsafe_put_pixel(x0 + i, y0 - i, color) });
    }
}

/// Draws a straight line between two points.  Ignores points that are outside of the image bounds.
///
/// Uses the Bresenham line drawing algorithm.
///
/// ```
/// # use image::{RgbaImage, Rgba};
/// use freehand::lines::line;
/// # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
///
/// line(&mut image, (0, 0), (399, 399), Rgba([255, 0, 0, 255]));
/// ```
pub fn line<I, P>(image: &mut I, a: P, b: P, color: I::Pixel)
where
    I: GenericImage,
    P: Point<i32>,
{
    let width = image.width().min((std::i32::MAX) as u32) as i32;
    let height = image.height().min((std::i32::MAX) as u32) as i32;

    for Pt { x, y } in BresIter::new(a, b) {
        if (0..width).contains(&x) && (0..height).contains(&y) {
            // Avoid double checking bounds with unsafe_put_pixel()
            // This is safe because the bounds have already been checked
            unsafe {
                image.unsafe_put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

/// Draws a dashed straight line between two points.
/// Points that are outside of the image bounds are ignored.
///
/// If the width is 0 then a solid line is drawn between the two points.
///
/// Uses the Bresenham line drawing algorithm.
///
/// ```
/// # use image::{RgbaImage, Rgba};
/// use freehand::lines::dashed_line;
/// # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
///
/// let dash: u8 = 2;
/// dashed_line(&mut image, (0, 0), (399, 399), dash, Rgba([255, 0, 0, 255]));
/// ```
pub fn dashed_line<I, P, W>(image: &mut I, a: P, b: P, dash_width: W, color: I::Pixel)
where
    I: GenericImage,
    P: Point<i32>,
    W: Into<u16>,
{
    let dash_width = dash_width.into() as usize;
    let w = dash_width as usize * 2;

    if dash_width == 0 {
        line(image, a, b, color);
        return;
    }

    let width = image.width().min((std::i32::MAX) as u32) as i32;
    let height = image.height().min((std::i32::MAX) as u32) as i32;

    for (i, Pt { x, y }) in BresIter::new(a, b).enumerate() {
        if (0..width).contains(&x) && (0..height).contains(&y) && i % w < dash_width {
            // Avoid double checking bounds with unsafe_put_pixel()
            // This is safe because the bounds have already been checked
            unsafe {
                image.unsafe_put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

#[derive(Clone, Debug)]
/// An iterator between two points on a line.
///
/// ```
/// use freehand::lines::BresIter;
///
/// for freehand::Pt {x, y} in BresIter::new((0, 0), (399, 399)) {
///     // ...
/// }
/// ```
// https://github.com/ssloy/tinyrenderer/wiki/Lesson-1:-Bresenham%E2%80%99s-Line-Drawing-Algorithm#timings-fifth-and-final-attempt
pub struct BresIter {
    /// Current position
    pt: Pt<i32>,
    /// Where to end
    end: Pt<i32>,
    d: i32,
    /// Amount added to decision parameter every step
    dy: i32,
    /// Amount subtracted from decision parameter on y steps
    dx: i32,
    /// Amount added to y on y steps
    y_step: i32,
    /// If steep the x, y coordinates are transposed
    steep: bool,
}

impl BresIter {
    /// Creates an iterator between two points on a line.
    ///
    /// Uses the Bresenham line drawing algorithm.
    ///
    /// ```
    /// use freehand::lines::BresIter;
    ///
    /// for freehand::Pt {x, y} in BresIter::new((0, 0), (399, 399)) {
    ///     // ...
    /// }
    /// ```
    pub fn new<P>(a: P, b: P) -> Self
    where
        P: Point<i32>,
    {
        let (mut a, mut b) = (a.pt(), b.pt());

        let steep = (a.x() - b.x()).abs() < (a.y() - b.y()).abs();
        if steep {
            a.transpose();
            b.transpose();
        }

        if a.x() > b.x() {
            std::mem::swap(&mut a, &mut b);
        }

        let d = 0;

        let y_step = match a.y() > b.y() {
            true => -1,
            false if a.y() == b.y() => 0,
            false => 1,
        };

        Self {
            pt: a,
            end: b,
            d,
            dy: (b.y() - a.y()).abs() * 2,
            dx: b.x() - a.x(),
            y_step,
            steep,
        }
    }

    /// Returns the current position in the line
    pub fn pt(&self) -> &Pt<i32> {
        &self.pt
    }

    /// Returns the end point of the line
    pub fn end(&self) -> &Pt<i32> {
        &self.end
    }

    /// Returns the decision parameter that decides whether to change y or not
    pub fn d(&self) -> i32 {
        self.d
    }

    /// Error amount added to the decision parameter every step
    pub fn dy(&self) -> i32 {
        self.dy
    }

    /// Error amount subtracted from the decision parameter when y changes
    pub fn dx(&self) -> i32 {
        self.dx
    }

    /// Amount added to y on y steps
    pub fn y_step(&self) -> i32 {
        self.y_step
    }

    /// If steep is true the x, y coordinates are transposed
    pub fn steep(&self) -> bool {
        self.steep
    }
}

impl Iterator for BresIter {
    type Item = Pt<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pt = self.pt;
        if self.pt.x() > self.end.x() {
            return None;
        }

        self.pt.add_x(1);
        self.d += self.dy;

        if self.d > self.dx {
            self.pt.add_y(self.y_step);
            self.d -= self.dx * 2;
        }

        if self.steep {
            pt.transpose();
        }

        Some(pt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_pixels_changed;

    #[test]
    fn basic_drawing() -> Result<(), image::ImageError> {
        crate::logger(crate::LOG_LEVEL);
        let height = 400;
        let width = 400;

        let mut image =
            image::RgbaImage::from_pixel(width, height, image::Rgba([255, 255, 255, 255]));

        dashed_line(
            &mut image,
            (0, 0),
            (399, 399),
            10u16,
            image::Rgba([255, 0, 0, 255]),
        );
        image.save("images/dashed_bres.png")
    }

    mod line {
        use super::*;

        test_pixels_changed!(line_bounds_neg_x, line((-100, 1), (-10, 1)), 4, &*vec![]);

        test_pixels_changed!(line_bounds_large_x, line((100, 1), (50, 1)), 4, &*vec![]);

        test_pixels_changed!(line_bounds_neg_y, line((1, -100), (1, -50)), 4, &*vec![]);

        test_pixels_changed!(line_bounds_large_y, line((1, 100), (1, 50)), 4, &*vec![]);

        test_pixels_changed!(
            line_hor,
            line((-5, 1), (10, 1)),
            4,
            &*vec![(0, 1), (1, 1), (2, 1), (3, 1)]
        );

        test_pixels_changed!(
            line_vert,
            line((1, 10), (1, -5)),
            4,
            &*vec![(1, 0), (1, 1), (1, 2), (1, 3)]
        );

        test_pixels_changed!(
            line_diag_up,
            line((0, 3), (3, 0)),
            4,
            &*vec![(0, 3), (1, 2), (2, 1), (3, 0)]
        );

        test_pixels_changed!(
            line_diag_down,
            line((0, 3), (3, 0)),
            4,
            &*vec![(3, 0), (2, 1), (1, 2), (0, 3)]
        );

        test_pixels_changed!(
            line_steep_down,
            line((0, 0), (2, 5)),
            6,
            &*vec![(0, 0), (0, 1), (1, 2), (1, 3), (2, 4), (2, 5)]
        );

        test_pixels_changed!(
            line_across,
            line((0, 0), (5, 2)),
            6,
            &*vec![(0, 0), (1, 0), (2, 1), (3, 1), (4, 2), (5, 2)]
        );
    }

    mod dashed_line {
        use super::*;

        test_pixels_changed!(
            dashed_line_steep_down_0px,
            dashed_line((0, 0), (2, 5), 0u8),
            6,
            &*vec![(0, 0), (0, 1), (1, 2), (1, 3), (2, 4), (2, 5)]
        );

        test_pixels_changed!(
            dashed_line_steep_down_1px,
            dashed_line((0, 0), (2, 5), 1u8),
            6,
            &*vec![(0, 0), (1, 2), (2, 4)]
        );

        test_pixels_changed!(
            dashed_line_steep_down_2px,
            dashed_line((0, 0), (2, 5), 2u8),
            6,
            &*vec![(0, 0), (0, 1), (2, 4), (2, 5)]
        );

        test_pixels_changed!(
            dashed_line_steep_down_5px,
            dashed_line((0, 0), (2, 5), 5u8),
            6,
            &*vec![(0, 0), (0, 1), (1, 2), (1, 3), (2, 4)]
        );
    }

    mod horizontal_line {
        use super::*;

        test_pixels_changed!(
            horizontal_line,
            horizontal_line((0, 0), 10),
            3,
            &*vec![(0, 0), (1, 0), (2, 0)]
        );

        test_pixels_changed!(
            horizontal_line_bounds,
            horizontal_line((10, 10), 100),
            3,
            &*vec![]
        );
    }

    mod diagonal_line {
        use super::*;

        test_pixels_changed!(
            diagonal_line,
            diagonal_line((0, 0), (10, 10)),
            3,
            &*vec![(0, 0), (1, 1), (2, 2)]
        );
        test_pixels_changed!(
            diagonal_line_bounds,
            diagonal_line((10, 10), (100, 100)),
            3,
            &*vec![]
        );
    }
}
