pub(crate) mod blend;
use crate::{blend_at_unchecked, Pt};
use image::{GenericImage, Rgba, RgbaImage};

pub fn horizontal_line<I: GenericImage>(image: &mut I, y: u32, x0: u32, x1: u32, color: I::Pixel) {
    if y < image.height() {
        (x0.min(image.width() - 1)..=x1.min(image.width() - 1))
            .for_each(|x| unsafe { image.unsafe_put_pixel(x, y, color) });
    }
}

pub fn vertical_line<I: GenericImage>(image: &mut I, x: u32, y0: u32, y1: u32, color: I::Pixel) {
    if x < image.width() {
        (y0.min(image.height() - 1)..=y1.min(image.height() - 1))
            .for_each(|y| unsafe { image.unsafe_put_pixel(x, y, color) });
    }
}

/// Draws a straight diagonal line between two points without bounds checking.
///
/// # Safety
///
/// Both x and y values must be within the image boundaries
pub unsafe fn diagonal_line_unchecked<I: GenericImage>(
    image: &mut I,
    mut x0: u32,
    mut y0: u32,
    mut x1: u32,
    mut y1: u32,
    color: I::Pixel,
) {
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }
    if y0 < y1 {
        let dist = (x1 - x0).min(y1 - y0);
        (0..=dist).for_each(|i| image.unsafe_put_pixel(x0 + i, y0 + i, color));
    } else {
        let dist = (x1 - x0).min(y0 - y1);
        (0..=dist).for_each(|i| image.unsafe_put_pixel(x0 + i, y0 - i, color));
    }
}

/// Draws a straight diagonal line between two points
pub fn diagonal_line<I: GenericImage>(
    image: &mut I,
    mut x0: u32,
    mut y0: u32,
    mut x1: u32,
    mut y1: u32,
    color: I::Pixel,
) {
    unsafe {
        diagonal_line_unchecked(
            image,
            x0.min(image.width() - 1),
            y0.min(image.height() - 1),
            x1.min(image.width() - 1),
            y1.min(image.height() - 1),
            color,
        );
    }
}

pub fn horizontal_dashed_line<I: GenericImage>(
    image: &mut I,
    y: u32,
    mut x0: u32,
    mut x1: u32,
    width: u32,
    color: I::Pixel,
) {
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
    }
    if (width == 0) || (y >= image.height() || (x0 >= image.width())) {
        return;
    }
    let mut x = x0.min(image.width() - 1);
    let mut i = 0;
    while x < x1.min(image.width() - 1) {
        unsafe {
            image.unsafe_put_pixel(x, y, color);
        }
        x = if i == width - 1 { x + width + 1 } else { x + 1 };
        i = if i == width - 1 { 0 } else { i + 1 };
    }
}

pub fn vertical_dashed_line<I: GenericImage>(
    image: &mut I,
    x: u32,
    mut y0: u32,
    mut y1: u32,
    width: u32,
    color: I::Pixel,
) {
    if y0 > y1 {
        std::mem::swap(&mut y0, &mut y1);
    }
    if (width == 0) || (x >= image.width() || (y0 >= image.height())) {
        return;
    }
    let mut y = y0.min(image.height() - 1);
    let mut i = 0;
    while y < y1.min(image.height() - 1) {
        unsafe {
            image.unsafe_put_pixel(x, y, color);
        }
        y = if i == width - 1 { y + width + 1 } else { y + 1 };
        i = if i == width - 1 { 0 } else { i + 1 };
    }
}

pub fn horizontal_dashed_line_alpha(
    image: &mut RgbaImage,
    y: u32,
    mut x0: u32,
    mut x1: u32,
    width: u32,
    opacity: f32,
    color: Rgba<u8>,
) {
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
    }
    if (width == 0) || (y >= image.height() || (x0 >= image.width())) {
        return;
    }
    let mut x = x0.min(image.width() - 1);
    let mut i = 0;
    while x < x1.min(image.width() - 1) {
        let (r, g, b) = (color[0], color[1], color[2]);
        unsafe {
            blend_at_unchecked(image, x, y, Rgba([r, g, b, 255]), opacity as f32);
        }
        x = if i == width - 1 { x + width + 1 } else { x + 1 };
        i = if i == width - 1 { 0 } else { i + 1 };
    }
}

pub fn vertical_dashed_line_alpha(
    image: &mut RgbaImage,
    x: u32,
    mut y0: u32,
    mut y1: u32,
    width: u32,
    opacity: f32,
    color: Rgba<u8>,
) {
    if y0 > y1 {
        std::mem::swap(&mut y0, &mut y1);
    }
    if (width == 0) || (x >= image.width() || (y0 >= image.height())) {
        return;
    }
    let mut y = y0.min(image.height() - 1);
    let mut i = 0;
    while y < y1.min(image.height() - 1) {
        let (r, g, b) = (color[0], color[1], color[2]);
        unsafe {
            blend_at_unchecked(image, x, y, Rgba([r, g, b, 255]), opacity as f32);
        }
        y = if i == width - 1 { y + width + 1 } else { y + 1 };
        i = if i == width - 1 { 0 } else { i + 1 };
    }
}

pub fn rectangle_filled<I: GenericImage>(
    image: &mut I,
    pt: Pt<u32>,
    height: u32,
    width: u32,
    color: I::Pixel,
) {
    let x0 = pt.x();
    let x1 = pt.x() + width - 1;
    for y in pt.y()..pt.y() + height {
        horizontal_line(image, y, x0, x1, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // #[ignore = "Image drawing test - requires manual validation; also this test won't change often"]
    fn basic_drawing() -> Result<(), image::ImageError> {
        crate::logger(log::LevelFilter::Debug);
        let height = 400;
        let width = 400;
        let mut image = image::RgbaImage::from_pixel(width, height, Rgba([255, 255, 255, 255]));
        vertical_line(&mut image, width / 2, 0, height - 1, Rgba([0, 0, 255, 255]));
        horizontal_line(&mut image, height / 2, 0, width - 1, Rgba([0, 255, 0, 255]));
        rectangle_filled(
            &mut image,
            Pt::new(300, 300),
            150,
            150,
            Rgba([255, 0, 0, 255]),
        );
        horizontal_dashed_line(&mut image, 20, 0, width * 2, 10, Rgba([0, 255, 0, 255]));
        vertical_dashed_line(&mut image, 20, 0, width - 1, 1, Rgba([0, 0, 255, 255]));

        diagonal_line(&mut image, 200, 200, 400, 0, Rgba([255, 98, 0, 255]));
        diagonal_line(&mut image, 200, 200, 0, 0, Rgba([255, 98, 0, 255]));
        diagonal_line(&mut image, 200, 200, 400, 400, Rgba([255, 98, 0, 255]));
        diagonal_line(&mut image, 200, 200, 0, 500, Rgba([255, 98, 0, 255]));

        image.save("images/basic_drawing.png")
    }
}
