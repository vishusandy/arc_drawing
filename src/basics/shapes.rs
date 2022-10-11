use image::GenericImage;

/// Draws a filled rectangle.  The specified point represents the upper left cordner
/// of the rectangle, and will be drawn using the given `height` and `width`.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::shapes::rectangle_filled;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]); // red
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// /// Draws a filled rectangle in the image.
/// /// A 10px white border will be left around the image borders.
/// rectangle_filled(&mut image, (10, 10), 380, 380, color);
/// ```
pub fn rectangle_filled<I: GenericImage, P: crate::pt::Point<u32>>(
    image: &mut I,
    pt: P,
    height: u32,
    width: u32,
    color: I::Pixel,
) {
    let x0 = pt.x();
    let x1 = pt.x() + width - 1;
    for y in pt.y()..pt.y() + height {
        crate::lines::horizontal_line(image, crate::Pt::new(x0, y), x1, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_no_color, test_pixels_changed};

    mod rectangle_filled {
        use super::*;

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
}
