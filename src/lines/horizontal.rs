// These functions are exported publicly in a different module - keep the module prefix
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::many_single_char_names)]

use crate::ops::blend_at_unchecked;
use crate::pt::Point;
use image::{GenericImage, Rgba, RgbaImage};

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

/// Draws a dashed horizontal line.
///
/// A `width` of 0 will draw a solid horizontal line.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::lines::horizontal_dashed_line;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]);
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
        crate::lines::horizontal_line(image, pt, x2, color);
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

/// Draws a solid horizontal line by blending it into the image with a specified opacity.
///
/// Opacity should be in the range `0..=1`.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::lines::horizontal_line_alpha;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]);
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
    debug_assert!((0.0..=1.0).contains(&opacity));

    if pt.y() < image.height() {
        (pt.x().min(image.width() - 1)..=x2.min(image.width() - 1))
            // This is safe due to the min() calls above
            .for_each(|x| unsafe { blend_at_unchecked(image, x, pt.y(), opacity, color) });
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
/// let color = Rgba([255, 0, 0, 255]);
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
    debug_assert!((0.0..=1.0).contains(&opacity));

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
            blend_at_unchecked(image, x, y, opacity, Rgba([r, g, b, 255]));
        }
        x = if i == width - 1 { x + width + 1 } else { x + 1 };
        i = if i == width - 1 { 0 } else { i + 1 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_pixel_colors, test_pixels_changed};
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
}
