use image::GenericImage;

/// Draws a basic rectangle.
///
/// # Example
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::shapes::rectangle;
///
/// let color = Rgba([255, 0, 0, 255]);
/// let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
///
/// rectangle(&mut image, (10, 10), 380, 380, color);
/// ```
///
/// See also: [`Draw::rectangle`](crate::Draw::rectangle)
///
pub fn rectangle<I, P>(image: &mut I, pt: P, height: u32, width: u32, color: I::Pixel)
where
    I: GenericImage,
    P: crate::pt::Point<u32>,
{
    let x0 = pt.x();
    let x1 = x0 + width - 1;
    let y0 = pt.y();
    let y1 = y0 + height - 1;

    // Top
    crate::lines::horizontal_line(image, crate::Pt::new(x0 + 1, y0), x1, color);
    // Bottom
    crate::lines::horizontal_line(image, crate::Pt::new(x0, y1), x1 - 1, color);
    // Left
    crate::lines::vertical_line(image, crate::Pt::new(x0, y0), y1 - 1, color);
    // Right
    crate::lines::vertical_line(image, crate::Pt::new(x1, y0 + 1), y1, color);
}

/// Draws a basic rectangle with the specified opacity.
///
/// # Example
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::shapes::rectangle_alpha;
///
/// let color = Rgba([255, 0, 0, 255]);
/// let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
///
/// rectangle_alpha(&mut image, (10, 10), 380, 380, 0.5, color);
/// ```
///
/// See also: [`Draw::rectangle_alpha`](crate::Draw::rectangle_alpha)
///
pub fn rectangle_alpha<P>(
    image: &mut image::RgbaImage,
    pt: P,
    height: u32,
    width: u32,
    opacity: f32,
    color: image::Rgba<u8>,
) where
    P: crate::pt::Point<u32>,
{
    let x0 = pt.x();
    let x1 = x0 + width - 1;
    let y0 = pt.y();
    let y1 = y0 + height - 1;

    // Top
    crate::lines::horizontal_line_alpha(image, crate::Pt::new(x0 + 1, y0), x1, opacity, color);
    // Bottom
    crate::lines::horizontal_line_alpha(image, crate::Pt::new(x0, y1), x1 - 1, opacity, color);
    // Left
    crate::lines::vertical_line_alpha(image, crate::Pt::new(x0, y0), y1 - 1, opacity, color);
    // Right
    crate::lines::vertical_line_alpha(image, crate::Pt::new(x1, y0 + 1), y1, opacity, color);
}

/// Draws a filled rectangle.  The specified point represents the upper left cordner
/// of the rectangle, and will be drawn using the given `height` and `width`.
///
/// # Example
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::shapes::rectangle_filled;
///
/// let color = Rgba([255, 0, 0, 255]);
/// let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
///
/// /// A 10px white border will be left around the image borders.
/// rectangle_filled(&mut image, (10, 10), 380, 380, color);
/// ```
///
/// See also: [`Draw::rectangle_filled`](crate::Draw::rectangle_filled)
///
pub fn rectangle_filled<I, P>(image: &mut I, pt: P, height: u32, width: u32, color: I::Pixel)
where
    I: GenericImage,
    P: crate::pt::Point<u32>,
{
    let x0 = pt.x();
    let x1 = pt.x() + width - 1;
    for y in pt.y()..pt.y() + height {
        crate::lines::horizontal_line(image, crate::Pt::new(x0, y), x1, color);
    }
}

/// Draws a filled rectangle with a specified opacity.
///
/// # Example
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::shapes::rectangle_filled_alpha;
///
/// let color = Rgba([255, 0, 0, 255]);
/// let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
///
/// /// A 10px white border will be left around the image borders.
/// rectangle_filled_alpha(&mut image, (10, 10), 380, 380, 0.5, color);
/// ```
///
/// See also: [`Draw::rectangle_filled_alpha`](crate::Draw::rectangle_filled_alpha)
///
pub fn rectangle_filled_alpha<P>(
    image: &mut image::RgbaImage,
    pt: P,
    height: u32,
    width: u32,
    opacity: f32,
    color: image::Rgba<u8>,
) where
    P: crate::pt::Point<u32>,
{
    let x0 = pt.x();
    let x1 = pt.x() + width - 1;
    for y in pt.y()..pt.y() + height {
        crate::lines::horizontal_line_alpha(image, crate::Pt::new(x0, y), x1, opacity, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod rectangle {

        test_pixels_changed!(
            rectangle_solid,
            rectangle((0, 0), 4, 4),
            4,
            &*vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 0),
                (2, 0),
                (1, 3),
                (2, 3),
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3)
            ]
        );
    }

    mod rectangle_alpha {

        test_pixel_colors!(
            rectangle_alpha,
            rectangle_alpha((0, 0), 4, 4, 0.5),
            4,
            image::Rgba([255, 0, 0, 255]),
            &*vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 0),
                (2, 0),
                (1, 3),
                (2, 3),
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3)
            ],
            &*vec![image::Rgba([255, 127, 127, 255]); 12]
        );
    }

    mod rectangle_filled {

        test_no_color!(
            rectangle_filled_full,
            rectangle_filled((0, 0), 10, 10),
            3,
            image::Rgba([255, 0, 0, 255]),
            image::Rgba([255, 255, 255, 255])
        );

        test_pixels_changed!(
            rectangle_filled_with_border,
            rectangle_filled((1, 1), 2, 2),
            4,
            &*vec![(1, 1), (1, 2), (2, 1), (2, 2)]
        );
    }

    mod rectangle_filled_alpha {

        test_only_color!(
            rectangle_filled_alpha_full,
            rectangle_filled_alpha((0, 0), 10, 10, 0.5),
            3,
            image::Rgba([255, 0, 0, 255]),
            image::Rgba([255, 127, 127, 255])
        );

        test_pixels_changed!(
            rectangle_filled_alpha_with_border,
            rectangle_filled_alpha((1, 1), 2, 2, 0.5),
            4,
            &*vec![(1, 1), (1, 2), (2, 1), (2, 2)]
        );
    }
}
