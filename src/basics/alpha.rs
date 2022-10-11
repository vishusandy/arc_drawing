use crate::ops::blend_at_unchecked;
use crate::pt::Point;
use image::{Rgba, RgbaImage};

/// Draws a solid horizontal line by blending it into the image with a specified opacity.
///
/// Opacity should be in the range `0..=1`.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::lines::horizontal_line_alpha;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]); // red
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// /// Horizontal line across the center of the image with 50% opacity
/// horizontal_line_alpha(&mut image, (0, 200), 399, 0.5, color);
/// ```
pub fn horizontal_line_alpha<P>(
    image: &mut RgbaImage,
    pt: P,
    x2: u32,
    opacity: f32,
    color: Rgba<u8>,
) where
    P: Point<u32>,
{
    if pt.y() < image.height() {
        (pt.x().min(image.width() - 1)..=x2.min(image.width() - 1))
            // This is safe due to the min() calls above
            .for_each(|x| unsafe { blend_at_unchecked(image, x, pt.y(), opacity, color) });
    }
}

/// Draws a solid vertical line by blending it into the image with a specified opacity.
///
/// Opacity should be in the range `0..=1`.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::lines::vertical_line_alpha;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]); // red
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// /// Vertical line across the center of the image with 50% opacity
/// vertical_line_alpha(&mut image, (200, 0), 399, 0.5, color);
/// ```
pub fn vertical_line_alpha<P>(image: &mut RgbaImage, pt: P, y2: u32, opacity: f32, color: Rgba<u8>)
where
    P: Point<u32>,
{
    if pt.x() < image.width() {
        (pt.y().min(image.height() - 1)..=y2.min(image.height() - 1))
            // This is safe due to the min() calls above
            .for_each(|y| unsafe { blend_at_unchecked(image, pt.x(), y, opacity, color) });
    }
}

/// Draws a solid diagonal line between two points by blending it into the image
/// with a specified opacity.
///
/// Opacity should be in the range `0..=1`.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::lines::diagonal_line_alpha;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]); // red
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// /// Downwards diagonal line across the image with 50% opacity
/// diagonal_line_alpha(&mut image, (0, 0), (399, 399), 0.5, color);
/// /// Upwards diagonal line across the image with 50% opacity
/// diagonal_line_alpha(&mut image, (0, 399), (399, 0), 0.5, color);
/// ```
pub fn diagonal_line_alpha<P>(
    image: &mut RgbaImage,
    mut a: P,
    mut b: P,
    opacity: f32,
    color: Rgba<u8>,
) where
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
        (0..=dist)
            .for_each(|i| unsafe { blend_at_unchecked(image, x0 + i, y0 + i, opacity, color) });
    } else {
        let dist = (x1 - x0).min(y0 - y1);
        // This is safe due to the min calls above
        (0..=dist)
            .for_each(|i| unsafe { blend_at_unchecked(image, x0 + i, y0 - i, opacity, color) });
    }
}

/// Draws a dashed vertical line by blending it into the image with a specified opacity.
///
/// Opacity should be in the range `0..=1`.
///
/// A `width` of 0 will draw a solid vertical line.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::lines::vertical_dashed_line_alpha;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]); // red
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// /// Vertical dashed line across the center of the image with a 2px dash and 50% opacity
/// vertical_dashed_line_alpha(&mut image, (200, 0), 399, 2, 0.5, color);
/// ```
pub fn vertical_dashed_line_alpha<P>(
    image: &mut RgbaImage,
    pt: P,
    mut y2: u32,
    width: u32,
    opacity: f32,
    color: Rgba<u8>,
) where
    P: Point<u32>,
{
    if width == 0 {
        vertical_line_alpha(image, pt, y2, opacity, color);
        return;
    }

    let (x, mut y1) = pt.tuple();

    if y1 > y2 {
        std::mem::swap(&mut y1, &mut y2);
    }

    if x >= image.width() || (y1 >= image.height()) {
        return;
    }

    let y2 = y2.min(image.height() - 1);
    let mut y = y1.min(image.height() - 1);
    let mut i = 0;
    while y <= y2 {
        let (r, g, b) = (color[0], color[1], color[2]);
        // This is safe due to the min calls above
        unsafe {
            blend_at_unchecked(image, x, y, opacity as f32, Rgba([r, g, b, 255]));
        }
        y = if i == width - 1 { y + width + 1 } else { y + 1 };
        i = if i == width - 1 { 0 } else { i + 1 };
    }
}

/// Draws a dashed horizontal line by blending it into the image with a specified opacity.
///
/// Opacity should be in the range `0..=1`.
///
/// A `width` of 0 will draw a solid horizontal line.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::lines::horizontal_dashed_line_alpha;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]); // red
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// /// Horizontal dashed line across the center of the image with a 2px dash and 50% opacity
/// horizontal_dashed_line_alpha(&mut image, (0, 200), 399, 2, 0.5, color);
/// ```
pub fn horizontal_dashed_line_alpha<P>(
    image: &mut RgbaImage,
    pt: P,
    mut x2: u32,
    width: u32,
    opacity: f32,
    color: Rgba<u8>,
) where
    P: Point<u32>,
{
    if width == 0 {
        horizontal_line_alpha(image, pt, x2, opacity, color);
        return;
    }

    let (mut x1, y) = pt.tuple();
    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
    }

    if y >= image.height() || (x1 >= image.width()) {
        horizontal_line_alpha(image, pt, x2, opacity, color);
        return;
    }

    let x2 = x2.min(image.width() - 1);
    let mut x = x1.min(image.width() - 1);
    let mut i = 0;

    while x <= x2 {
        let (r, g, b) = (color[0], color[1], color[2]);
        // This is safe due to the min calls above
        unsafe {
            blend_at_unchecked(image, x, y, opacity as f32, Rgba([r, g, b, 255]));
        }
        x = if i == width - 1 { x + width + 1 } else { x + 1 };
        i = if i == width - 1 { 0 } else { i + 1 };
    }
}

/// Draws a dashed diagonal line between two points by blending it into the image
/// with a specified opacity.
///
/// Opacity should be in the range `0..=1`.
///
/// A `width` of 0 will draw a solid diagonal line.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::lines::diagonal_dashed_line_alpha;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]); // red
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// /// Downards dashed diagonal line across the image with a 2px dash and 50% opacity
/// diagonal_dashed_line_alpha(&mut image, (0, 0), (399, 399), 2, 0.5, color);
/// /// Upwards dashed diagonal line across the image with a 2px dash and 50% opacity
/// diagonal_dashed_line_alpha(&mut image, (0, 399), (399, 0), 2, 0.5, color);
/// ```
pub fn diagonal_dashed_line_alpha<P>(
    image: &mut RgbaImage,
    mut a: P,
    mut b: P,
    width: u32,
    opacity: f32,
    color: Rgba<u8>,
) where
    P: Point<u32>,
{
    if width == 0 {
        diagonal_line_alpha(image, a, b, opacity, color);
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
                blend_at_unchecked(image, x0 + i, y0 + i, opacity, color);
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
                blend_at_unchecked(image, x0 + i, y0 - i, opacity, color);
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
    use crate::test_pixel_colors;

    mod horizontal_line_alpha {
        use super::*;

        test_pixel_colors!(
            horizontal_line_alpha,
            horizontal_line_alpha((0, 0), 10, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );
        test_pixel_colors!(
            horizontal_line_alpha_bounds,
            horizontal_line_alpha((10, 10), 20, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );
    }

    mod vertical_line_alpha {
        use super::*;

        test_pixel_colors!(
            vertical_line_alpha,
            vertical_line_alpha((0, 0), 10, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            vertical_line_alpha_bounds,
            vertical_line_alpha((10, 10), 20, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );
    }

    mod diagonal_line_alpha {
        use super::*;

        test_pixel_colors!(
            diagonal_line_alpha,
            diagonal_line_alpha((0, 0), (10, 10), 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            diagonal_line_alpha_swap,
            diagonal_line_alpha((6, 6), (0, 0), 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            diagonal_line_alpha_upwards,
            diagonal_line_alpha((6, 0), (0, 6), 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(5, 0), (4, 1), (3, 2), (2, 3), (1, 4), (0, 5)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            diagonal_line_alpha_bounds,
            diagonal_line_alpha((20, 20), (10, 10), 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );
    }

    mod horizontal_dashed_line_alpha {
        use super::*;

        test_pixel_colors!(
            horizontal_dashed_line_alpha_0px,
            horizontal_dashed_line_alpha((0, 0), 10, 0, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            horizontal_dashed_line_alpha_1px,
            horizontal_dashed_line_alpha((0, 0), 10, 1, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (2, 0), (4, 0)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            horizontal_dashed_line_alpha_swap_1px,
            horizontal_dashed_line_alpha((10, 0), 0, 1, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (2, 0), (4, 0)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            horizontal_dashed_line_alpha_2px,
            horizontal_dashed_line_alpha((0, 0), 10, 2, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (1, 0), (4, 0), (5, 0)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            horizontal_dashed_line_alpha_5px,
            horizontal_dashed_line_alpha((0, 0), 10, 5, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            horizontal_dashed_line_alpha_bounds,
            horizontal_dashed_line_alpha((10, 10), 20, 1, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );
    }

    mod vertical_dashed_line_alpha {
        use super::*;

        test_pixel_colors!(
            vertical_dashed_line_alpha_0px,
            vertical_dashed_line_alpha((0, 0), 10, 0, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            vertical_dashed_line_alpha_1px,
            vertical_dashed_line_alpha((0, 0), 10, 1, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (0, 2), (0, 4)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            vertical_dashed_line_alpha_swap_1px,
            vertical_dashed_line_alpha((0, 10), 0, 1, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (0, 2), (0, 4)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            vertical_dashed_line_alpha_2px,
            vertical_dashed_line_alpha((0, 0), 10, 2, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (0, 1), (0, 4), (0, 5)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            vertical_dashed_line_alpha_5px,
            vertical_dashed_line_alpha((0, 0), 10, 5, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            vertical_dashed_line_alpha_bounds,
            vertical_dashed_line_alpha((10, 10), 20, 1, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );
    }

    mod diagonal_dashed_line_alpha {
        use super::*;

        test_pixel_colors!(
            diagonal_dashed_line_alpha_0px,
            diagonal_dashed_line_alpha((0, 0), (20, 10), 0, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            diagonal_dashed_line_alpha_1px,
            diagonal_dashed_line_alpha((0, 0), (20, 10), 1, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (2, 2), (4, 4)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            diagonal_dashed_line_alpha_swap_1px,
            diagonal_dashed_line_alpha((6, 0), (0, 6), 1, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 5), (2, 3), (4, 1)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            diagonal_dashed_line_alpha_2px,
            diagonal_dashed_line_alpha((0, 0), (20, 10), 2, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (1, 1), (4, 4), (5, 5)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            diagonal_dashed_line_alpha_5px,
            diagonal_dashed_line_alpha((0, 0), (20, 10), 5, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );

        test_pixel_colors!(
            diagonal_dashed_line_alpha_bounds,
            diagonal_dashed_line_alpha((10, 10), (20, 20), 1, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![],
            &*vec![image::Rgba([255, 127, 127, 255]); 6]
        );
    }
}
