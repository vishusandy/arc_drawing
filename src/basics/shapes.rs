use image::GenericImage;

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
