use crate::Pt;
use image::{GenericImage, Rgba, RgbaImage};

pub fn horizontal_line(image: &mut RgbaImage, y: u32, x0: u32, x1: u32, color: Rgba<u8>) {
    if y < image.height() {
        (x0.min(image.width() - 1)..=x1.min(image.width() - 1))
            .for_each(|x| unsafe { image.unsafe_put_pixel(x, y, color) });
    }
}

pub fn vertical_line(image: &mut RgbaImage, x: u32, y0: u32, y1: u32, color: Rgba<u8>) {
    if x < image.width() {
        (y0.min(image.height() - 1)..=y1.min(image.height() - 1))
            .for_each(|y| unsafe { image.unsafe_put_pixel(x, y, color) });
    }
}

pub fn rectangle_filled(
    image: &mut RgbaImage,
    pt: Pt<u32>,
    height: u32,
    width: u32,
    color: Rgba<u8>,
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
    #[ignore = "Image drawing test - requires manual validation; also this test won't change often"]
    fn basic_drawing() -> Result<(), image::ImageError> {
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
        image.save("images/tests/basic_drawing.png")
    }
}
