
use crate::Pt;
use image::GenericImage;

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
        crate::horizontal_line(image, y, x0, x1, color);
    }
}
