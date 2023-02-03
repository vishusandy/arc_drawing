use super::bres::LineIter;
use crate::pt::{Point, Pt};
use image::GenericImage;

/// Draws a straight line between two points.  Ignores points that are outside of the image bounds.
///
/// Panics
///
/// Panics if the image's height or width is larger than 2,147,483,647
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
    crate::check_img_i32!(image);

    #[allow(clippy::cast_possible_wrap)]
    let width = image.width() as i32;
    #[allow(clippy::cast_possible_wrap)]
    let height = image.height() as i32;

    for Pt { x, y } in LineIter::new(a, b) {
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
/// Panics
///
/// Panics if the image's height or width is larger than 2,147,483,647
///
/// ```
/// # use image::{RgbaImage, Rgba};
/// use freehand::lines::dashed_line;
/// # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
///
/// dashed_line(&mut image, (0, 0), (399, 399), 2, Rgba([255, 0, 0, 255]));
/// ```
pub fn dashed_line<I, P>(image: &mut I, a: P, b: P, dash_width: u16, color: I::Pixel)
where
    I: GenericImage,
    P: Point<i32>,
{
    crate::check_img_i32!(image);

    let dash_width = dash_width as usize;
    let w = dash_width * 2;

    if dash_width == 0 {
        line(image, a, b, color);
        return;
    }

    // safe because of earlier check on image bounds (check_img_i32)
    #[allow(clippy::cast_possible_wrap)]
    let height = image.height() as i32;
    #[allow(clippy::cast_possible_wrap)]
    let width = image.width() as i32;

    for (i, Pt { x, y }) in LineIter::new(a, b).enumerate() {
        if (0..width).contains(&x) && (0..height).contains(&y) && i % w < dash_width {
            // Avoid double checking bounds with unsafe_put_pixel()
            // This is safe because the bounds have already been checked
            unsafe {
                image.unsafe_put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

/// Draws a straight line between two points using a specified opacity.
/// Ignores points that are outside of the image bounds.
///
///
/// # Panics
///
/// - Panics if opacity is not in the range `0.0..=1.0`.
/// - Panics if the image's height or width is larger than 2,147,483,647
/// ```
/// # use image::{RgbaImage, Rgba};
/// use freehand::lines::line_alpha;
/// # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
///
/// line_alpha(&mut image, (0, 0), (399, 399), 0.5, Rgba([255, 0, 0, 255]));
/// ```
pub fn line_alpha<P>(image: &mut image::RgbaImage, a: P, b: P, opacity: f32, color: image::Rgba<u8>)
where
    P: Point<i32>,
{
    use crate::ops::blend_at_unchecked;

    crate::check_img_i32!(image);
    crate::check_opacity!(opacity);

    #[allow(clippy::cast_possible_wrap)]
    let width = image.width() as i32;
    #[allow(clippy::cast_possible_wrap)]
    let height = image.height() as i32;

    for Pt { x, y } in LineIter::new(a, b) {
        if (0..width).contains(&x) && (0..height).contains(&y) {
            // Avoid double checking bounds
            // This is safe because the bounds have already been checked
            unsafe {
                blend_at_unchecked(image, x as u32, y as u32, opacity, color);
            }
        }
    }
}

/// Draws a dashed straight line between two points.
/// Points that are outside of the image bounds are ignored.
///
/// If the width is 0 then a solid line is drawn between the two points.
///
/// # Panics
///
/// - Panics if opacity is not in the range `0.0..=1.0`
/// - Panics if the image's height or width is larger than 2,147,483,647
/// ```
/// # use image::{RgbaImage, Rgba};
/// use freehand::lines::dashed_line_alpha;
/// # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
///
/// let dash: u8 = 2;
/// let opacity: f32 = 0.5;
/// dashed_line_alpha(&mut image, (0, 0), (399, 399), dash, opacity, Rgba([255, 0, 0, 255]));
/// ```
pub fn dashed_line_alpha<P, W>(
    image: &mut image::RgbaImage,
    a: P,
    b: P,
    dash_width: W,
    opacity: f32,
    color: image::Rgba<u8>,
) where
    P: Point<i32>,
    W: Into<u16>,
{
    use crate::ops::blend_at_unchecked;

    crate::check_img_i32!(image);
    crate::check_opacity!(opacity);

    let dash_width = dash_width.into() as usize;
    let w = dash_width * 2;

    if dash_width == 0 {
        line(image, a, b, color);
        return;
    }

    #[allow(clippy::cast_possible_wrap)]
    let width = image.width() as i32;
    #[allow(clippy::cast_possible_wrap)]
    let height = image.height() as i32;

    for (i, Pt { x, y }) in LineIter::new(a, b).enumerate() {
        if (0..width).contains(&x) && (0..height).contains(&y) && i % w < dash_width {
            // Avoid double checking
            // This is safe because the bounds have already been checked
            unsafe {
                blend_at_unchecked(image, x as u32, y as u32, opacity, color);
            }
        }
    }
}

/// Draws a path using straight solid lines from one point to the next.
/// The start and end points are not connected.
///
/// ```
/// # use image::{RgbaImage, Rgba};
/// use freehand::lines::path;
/// # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
///
/// let lines = [(0, 0), (399, 0), (399, 399), (0, 399)];
/// path(&mut image, lines, Rgba([255, 0, 0, 255]));
/// ```
pub fn path<I, P, It>(image: &mut I, points: It, color: I::Pixel)
where
    I: GenericImage,
    P: Point<i32>,
    It: IntoIterator<Item = P>,
{
    let mut points = points.into_iter();

    let mut a = match points.next() {
        Some(first) => first,
        None => return,
    };

    for b in points {
        line(image, a.pt(), b.pt(), color);
        a = b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_pixels_changed;

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
            dashed_line((0, 0), (2, 5), 0),
            6,
            &*vec![(0, 0), (0, 1), (1, 2), (1, 3), (2, 4), (2, 5)]
        );

        test_pixels_changed!(
            dashed_line_steep_down_1px,
            dashed_line((0, 0), (2, 5), 1),
            6,
            &*vec![(0, 0), (1, 2), (2, 4)]
        );

        test_pixels_changed!(
            dashed_line_steep_down_2px,
            dashed_line((0, 0), (2, 5), 2),
            6,
            &*vec![(0, 0), (0, 1), (2, 4), (2, 5)]
        );

        test_pixels_changed!(
            dashed_line_steep_down_5px,
            dashed_line((0, 0), (2, 5), 5),
            6,
            &*vec![(0, 0), (0, 1), (1, 2), (1, 3), (2, 4)]
        );
    }

    mod path {

        #[test]
        fn path() -> Result<(), image::ImageError> {
            crate::logger(crate::LOG_LEVEL);
            let height = 400;
            let width = 400;

            let mut image =
                image::RgbaImage::from_pixel(width, height, image::Rgba([255, 255, 255, 255]));

            let lines = [(0, 50), (350, 50), (50, 350), (399, 350)];
            super::super::path(&mut image, lines, image::Rgba([255, 0, 0, 255]));
            image.save("images/path.png")
        }
    }
}
