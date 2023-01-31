// These functions are exported publicly in a different module - keep the module prefix
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::many_single_char_names)]

use crate::ops::blend_at_unchecked;
use crate::pt::Point;
use image::{GenericImage, Rgba, RgbaImage};

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

/// Draws a dashed vertical line.
///
/// A `width` of 0 will draw a solid vertical line.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::lines::vertical_dashed_line;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]);
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
        crate::lines::vertical_line(image, pt, y2, color);
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

/// Draws a solid vertical line by blending it into the image with a specified opacity.
///
/// Opacity should be in the range `0..=1`.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::lines::vertical_line_alpha;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]);
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// /// Vertical line across the center of the image with 50% opacity
/// vertical_line_alpha(&mut image, (200, 0), 399, 0.5, color);
/// ```
pub fn vertical_line_alpha<P>(image: &mut RgbaImage, pt: P, y2: u32, opacity: f32, color: Rgba<u8>)
where
    P: Point<u32>,
{
    debug_assert!((0.0..=1.0).contains(&opacity));

    if pt.x() < image.width() {
        (pt.y().min(image.height() - 1)..=y2.min(image.height() - 1))
            // This is safe due to the min() calls above
            .for_each(|y| unsafe { blend_at_unchecked(image, pt.x(), y, opacity, color) });
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
/// let color = Rgba([255, 0, 0, 255]);
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
    debug_assert!((0.0..=1.0).contains(&opacity));

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
            blend_at_unchecked(image, x, y, opacity, Rgba([r, g, b, 255]));
        }
        y = if i == width - 1 { y + width + 1 } else { y + 1 };
        i = if i == width - 1 { 0 } else { i + 1 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_pixel_colors, test_pixels_changed};

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
            vertical_line_alpha_right,
            vertical_line_alpha((5, 0), 20, 0.5),
            6,
            image::Rgba([255, 0, 0, 255]),
            &*vec![(5, 0), (5, 1), (5, 2), (5, 3), (5, 4), (5, 5)],
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
}
