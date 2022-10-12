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

pub fn line<I, P>(image: &mut I, a: P, b: P, color: I::Pixel)
where
    I: GenericImage,
    P: Point<i32>,
{
    let mut a = a.pt();
    let mut b = b.pt();

    // Ensure the line moves from left to right
    if a.x() > b.x() {
        std::mem::swap(&mut a, &mut b);
    }

    let (width, height) = image.dimensions();
    if !crate::size_check_u32_to_i32(width, height) {
        panic!("line() cannot handle images this large");
    }
    let (width, height) = (width as i32, height as i32);

    // Bounds check
    if a.x() >= width || b.x() < 0 || a.y().min(b.y()) >= height || a.y().max(b.y()) < 0 {
        return;
    }

    // Vertical lines should be handled by `vertical_line()`
    if a.x() == b.x() {
        vertical_line(image, a.min_u32(), b.y().max(0) as u32, color);
        return;
    }

    // Horizontal lines should be handled by `horizontal_line()`
    if a.y() == b.y() {
        horizontal_line(image, a.min_u32(), b.x().max(0) as u32, color);
        return;
    }
}

#[derive(Clone, Debug)]
// https://github.com/ssloy/tinyrenderer/wiki/Lesson-1:-Bresenham%E2%80%99s-Line-Drawing-Algorithm#timings-fifth-and-final-attempt
struct BresIter {
    /// Current position
    pt: Pt<i32>,
    /// Where to end
    end: Pt<i32>,
    d: i32,
    /// Amount added to decision parameter every step
    dx: i32,
    /// Amount subtracted from decision parameter on y steps
    dy: i32,
    /// Amount added to y on y steps
    y_step: i32,
    /// If steep the x, y coordinates are transposed
    steep: bool,
}

impl BresIter {
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

        let dx = (b.y() - a.y()).abs() * 2;
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
            dx,
            dy: b.x() - a.x(),
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
    pub fn e(&self) -> i32 {
        self.dx
    }

    /// Error amount the decision parameter changes when y changes
    pub fn dx(&self) -> i32 {
        self.dy
    }

    pub fn y_step(&self) -> i32 {
        self.y_step
    }
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

        self.d += self.dx;
        if self.d >= 0 {
            self.pt.add_y(self.y_step);
            self.d -= self.dy * 2;
        }

        self.pt.add_x(1);

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
    fn bres_iter() -> Result<(), image::ImageError> {
        crate::logger(crate::LOG_LEVEL);
        let height = 400;
        let width = 400;

        let mut image =
            image::RgbaImage::from_pixel(width, height, image::Rgba([255, 255, 255, 255]));

        let iter = BresIter::new((0, 0), (399, 20));
        log::debug!("{:?}", iter);

        for Pt { x, y } in iter {
            if (0..width as i32).contains(&x) && (0..height as i32).contains(&y) {
                image.put_pixel(x as u32, y as u32, image::Rgba([255, 0, 0, 255]));
            }
        }

        image.save("images/bres_iter.png")
    }

    #[test]
    fn line_down() -> Result<(), image::ImageError> {
        crate::logger(crate::LOG_LEVEL);
        let height = 400;
        let width = 400;

        let mut image =
            image::RgbaImage::from_pixel(width, height, image::Rgba([255, 255, 255, 255]));

        super::line(
            &mut image,
            (0, 100),
            (5, 300),
            image::Rgba([255, 0, 0, 255]),
        );

        image.save("images/line_down.png")
    }

    #[test]
    fn line_up() -> Result<(), image::ImageError> {
        crate::logger(crate::LOG_LEVEL);
        let height = 400;
        let width = 400;

        let mut image =
            image::RgbaImage::from_pixel(width, height, image::Rgba([255, 255, 255, 255]));

        super::line(
            &mut image,
            (0, 300),
            (399, 100),
            image::Rgba([255, 0, 0, 255]),
        );

        image.save("images/line_up.png")
    }

    #[test]
    fn line_horizontal() -> Result<(), image::ImageError> {
        crate::logger(crate::LOG_LEVEL);
        let height = 400;
        let width = 400;

        let mut image =
            image::RgbaImage::from_pixel(width, height, image::Rgba([255, 255, 255, 255]));

        super::line(
            &mut image,
            (0, 200),
            (399, 200),
            image::Rgba([255, 0, 0, 255]),
        );

        image.save("images/line_horizontal.png")
    }

    #[test]
    fn line_vertical() -> Result<(), image::ImageError> {
        crate::logger(crate::LOG_LEVEL);
        let height = 400;
        let width = 400;

        let mut image =
            image::RgbaImage::from_pixel(width, height, image::Rgba([255, 255, 255, 255]));

        super::line(
            &mut image,
            (200, 0),
            (200, 399),
            image::Rgba([255, 0, 0, 255]),
        );

        image.save("images/line_vertical.png")
    }

    mod vertical_line {
        use super::*;

        test_pixels_changed!(
            vertical_line,
            vertical_line((0, 0), 10),
            3,
            &*vec![(0, 0), (0, 1), (0, 2)]
        );
        test_pixels_changed!(
            vertical_line_bounds,
            vertical_line((10, 10), 100),
            3,
            &*vec![]
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
