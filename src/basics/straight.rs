use crate::pt::Point;
use image::GenericImage;

/// Draws a straight horizontal line.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::lines::horizontal_line;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]); // red
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
/// let color = Rgba([255, 0, 0, 255]); // red
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
/// let color = Rgba([255, 0, 0, 255]); // red
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_pixels_changed;

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
