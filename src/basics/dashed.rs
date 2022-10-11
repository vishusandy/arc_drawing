use crate::pt::Point;
use image::GenericImage;

/// Draws a dashed horizontal line.
///
/// A `width` of 0 will draw a solid horizontal line.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::lines::horizontal_dashed_line;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]); // red
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// /// Horizontal dashed line across the center of the image with a 2px dash
/// horizontal_dashed_line(&mut image, (0, 200), 399, 2, color);
/// ```
pub fn horizontal_dashed_line<I, P>(image: &mut I, pt: P, mut x2: u32, width: u32, color: I::Pixel)
where
    I: GenericImage,
    P: Point<u32>,
{
    if width == 0 {
        crate::basics::straight::horizontal_line(image, pt, x2, color);
        return;
    }

    let (mut x0, y) = pt.tuple();

    if x0 > x2 {
        std::mem::swap(&mut x0, &mut x2);
    }

    if y >= image.height() || x0 >= image.width() {
        return;
    }

    let x1 = x2.min(image.width() - 1);
    let mut x = x0.min(image.width() - 1);
    let mut i = 0;

    while x <= x1 {
        // This is safe due to the min calls above
        unsafe {
            image.unsafe_put_pixel(x, y, color);
        }
        x = if i == width - 1 { x + width + 1 } else { x + 1 };
        i = if i == width - 1 { 0 } else { i + 1 };
    }
}

/// Draws a dashed vertical line.
///
/// A `width` of 0 will draw a solid vertical line.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::lines::vertical_dashed_line;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]); // red
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// /// Vertical dashed line across the center of the image with a 2px dash
/// vertical_dashed_line(&mut image, (200, 0), 399, 2, color);
/// ```
pub fn vertical_dashed_line<I, P>(image: &mut I, pt: P, mut y2: u32, width: u32, color: I::Pixel)
where
    I: GenericImage,
    P: Point<u32>,
{
    if width == 0 {
        crate::basics::straight::vertical_line(image, pt, y2, color);
        return;
    }

    let (x, mut y0) = pt.tuple();

    if y0 > y2 {
        std::mem::swap(&mut y0, &mut y2);
    }

    if x >= image.width() || (y0 >= image.height()) {
        return;
    }

    let y1 = y2.min(image.height() - 1);
    let mut y = y0.min(image.height() - 1);
    let mut i = 0;

    while y <= y1 {
        // This is safe due to the min calls above
        unsafe {
            image.unsafe_put_pixel(x, y, color);
        }
        y = if i == width - 1 { y + width + 1 } else { y + 1 };
        i = if i == width - 1 { 0 } else { i + 1 };
    }
}

/// Draws a dashed diagonal line between two points.
///
/// A `width` of 0 will draw a solid diagonal line.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::lines::diagonal_dashed_line;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]); // red
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// /// Downards dashed diagonal line across the image with a 2px dash
/// diagonal_dashed_line(&mut image, (0, 0), (399, 399), 2, color);
/// /// Upwards dashed diagonal line across the image with a 2px dash
/// diagonal_dashed_line(&mut image, (0, 399), (399, 0), 2, color);
/// ```
pub fn diagonal_dashed_line<I, P>(image: &mut I, mut a: P, mut b: P, width: u32, color: I::Pixel)
where
    I: GenericImage,
    P: Point<u32>,
{
    if width == 0 {
        crate::basics::straight::diagonal_line(image, a, b, color);
        return;
    }

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
    let mut i = 0;

    if y0 < y1 {
        let dist = (x1 - x0).min(y1 - y0);
        while i <= dist {
            // This is safe due to the min calls above
            unsafe {
                image.unsafe_put_pixel(x0 + i, y0 + i, color);
            }
            let i1 = i + 1;
            let iw = i + width + 1;
            i = if i1 % width == 0 { iw } else { i1 };
        }
    } else {
        let dist = (x1 - x0).min(y0 - y1);
        while i <= dist {
            // This is safe due to the min calls above
            unsafe {
                image.unsafe_put_pixel(x0 + i, y0 - i, color);
            }
            let i1 = i + 1;
            let iw = i + width + 1;
            i = if i1 % width == 0 { iw } else { i1 };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_pixels_changed;

    mod horizontal_dashed_line {
        use super::*;

        test_pixels_changed!(
            horizontal_dashed_line_0px,
            horizontal_dashed_line((0, 0), 10, 0),
            6,
            &*vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)]
        );
        test_pixels_changed!(
            horizontal_dashed_line_1px,
            horizontal_dashed_line((0, 0), 10, 1),
            6,
            &*vec![(0, 0), (2, 0), (4, 0)]
        );
        test_pixels_changed!(
            horizontal_dashed_line_2px,
            horizontal_dashed_line((0, 0), 10, 2),
            6,
            &*vec![(0, 0), (1, 0), (4, 0), (5, 0)]
        );
        test_pixels_changed!(
            horizontal_dashed_line_5px,
            horizontal_dashed_line((0, 0), 10, 5),
            6,
            &*vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]
        );
        test_pixels_changed!(
            horizontal_dashed_line_bounds,
            horizontal_dashed_line((20, 20), 10, 2),
            6,
            &*vec![]
        );
    }

    mod vertical_dashed_line {
        use super::*;

        test_pixels_changed!(
            vertical_dashed_line_0px_width,
            vertical_dashed_line((0, 0), 10, 0),
            6,
            &*vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5)]
        );
        test_pixels_changed!(
            vertical_dashed_line_1px,
            vertical_dashed_line((0, 0), 10, 1),
            6,
            &*vec![(0, 0), (0, 2), (0, 4)]
        );
        test_pixels_changed!(
            vertical_dashed_line_2px,
            vertical_dashed_line((0, 0), 10, 2),
            6,
            &*vec![(0, 0), (0, 1), (0, 4), (0, 5)]
        );
        test_pixels_changed!(
            vertical_dashed_line_5px,
            vertical_dashed_line((0, 0), 10, 5),
            6,
            &*vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)]
        );
        test_pixels_changed!(
            vertical_dashed_line_bounds,
            vertical_dashed_line((20, 20), 10, 2),
            6,
            &*vec![]
        );
    }

    mod diagonal_dashed_line {
        use super::*;

        test_pixels_changed!(
            diagonal_dashed_line_0px_width,
            diagonal_dashed_line((0, 0), (10, 10), 0),
            6,
            &*vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]
        );
        test_pixels_changed!(
            diagonal_dashed_line_1px,
            diagonal_dashed_line((0, 0), (10, 10), 1),
            6,
            &*vec![(0, 0), (2, 2), (4, 4)]
        );
        test_pixels_changed!(
            diagonal_dashed_line_2px,
            diagonal_dashed_line((0, 0), (10, 10), 2),
            6,
            &*vec![(0, 0), (1, 1), (4, 4), (5, 5)]
        );
        test_pixels_changed!(
            diagonal_dashed_line_5px,
            diagonal_dashed_line((0, 0), (10, 10), 5),
            6,
            &*vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)]
        );
        test_pixels_changed!(
            diagonal_dashed_line_bounds,
            diagonal_dashed_line((10, 10), (10, 10), 2),
            6,
            &*vec![]
        );
    }
}
