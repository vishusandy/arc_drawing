// These functions are exported publicly in a different module - keep the module prefix
#![allow(clippy::module_name_repetitions)]

use crate::ops::blend_at_unchecked;
use crate::pt::Point;
use image::{GenericImage, Rgba, RgbaImage};

/// A straight diagonal line.
///
/// Only points within the image are drawn.
/// ```
/// use freehand::lines::diagonal_line;
/// # use image::{RgbaImage, Rgba};
/// # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
///
/// /// Downwards diagonal line across the image
/// diagonal_line(&mut image, (0, 0), (399, 399), Rgba([255, 0, 0, 255]));
/// /// Upwards diagonal line across the image
/// diagonal_line(&mut image, (0, 399), (399, 0), Rgba([255, 0, 0, 255]));
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

/// A dashed diagonal line.
///
/// A `width` of 0 will draw a solid diagonal line.
///
/// Only points within the image are drawn.
///
/// ```
/// use freehand::lines::diagonal_dashed_line;
/// # use image::{RgbaImage, Rgba};
/// # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
///
/// /// Downards dashed diagonal line across the image with a 2px dash
/// diagonal_dashed_line(&mut image, (0, 0), (399, 399), 2, Rgba([255, 0, 0, 255]));
/// /// Upwards dashed diagonal line across the image with a 2px dash
/// diagonal_dashed_line(&mut image, (0, 399), (399, 0), 2, Rgba([255, 0, 0, 255]));
/// ```
pub fn diagonal_dashed_line<I, P>(image: &mut I, mut a: P, mut b: P, width: u32, color: I::Pixel)
where
    I: GenericImage,
    P: Point<u32>,
{
    if width == 0 {
        crate::lines::diagonal_line(image, a, b, color);
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

/// A diagonal line with opacity.
///
/// Opacity should be in the range `0..=1`.
///
/// Only points within the image are drawn.
///
/// ```
/// use freehand::lines::diagonal_line_alpha;
/// # use image::{RgbaImage, Rgba};
/// # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
///
/// /// Downwards diagonal line across the image with 50% opacity
/// diagonal_line_alpha(&mut image, (0, 0), (399, 399), 0.5, Rgba([255, 0, 0, 255]));
/// /// Upwards diagonal line across the image with 50% opacity
/// diagonal_line_alpha(&mut image, (0, 399), (399, 0), 0.5, Rgba([255, 0, 0, 255]));
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
    debug_assert!((0.0..=1.0).contains(&opacity));

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

/// A dashed diagonal line with opacity.
///
/// Opacity should be in the range `0..=1`.
///
/// A `width` of 0 will draw a solid diagonal line.
///
/// Only points within the image are drawn.
///
/// ```
/// use freehand::lines::diagonal_dashed_line_alpha;
/// # use image::{RgbaImage, Rgba};
/// # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
///
/// /// Downards dashed diagonal line across the image with a 2px dash and 50% opacity
/// diagonal_dashed_line_alpha(&mut image, (0, 0), (399, 399), 2, 0.5, Rgba([255, 0, 0, 255]));
/// /// Upwards dashed diagonal line across the image with a 2px dash and 50% opacity
/// diagonal_dashed_line_alpha(&mut image, (0, 399), (399, 0), 2, 0.5, Rgba([255, 0, 0, 255]));
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
    debug_assert!((0.0..=1.0).contains(&opacity));

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
    use crate::{test_pixel_colors, test_pixels_changed};

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
