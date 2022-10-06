use image::GenericImage;

pub fn horizontal_line<I: GenericImage>(image: &mut I, y: u32, x0: u32, x1: u32, color: I::Pixel) {
    if y < image.height() {
        (x0.min(image.width() - 1)..=x1.min(image.width() - 1))
            // This is safe due to the min() calls above
            .for_each(|x| unsafe { image.unsafe_put_pixel(x, y, color) });
    }
}

pub fn vertical_line<I: GenericImage>(image: &mut I, x: u32, y0: u32, y1: u32, color: I::Pixel) {
    if x < image.width() {
        (y0.min(image.height() - 1)..=y1.min(image.height() - 1))
            // This is safe due to the min() calls above
            .for_each(|y| unsafe { image.unsafe_put_pixel(x, y, color) });
    }
}

/// Draws a straight diagonal line between two points.
pub fn diagonal_line<I: GenericImage, P: Into<crate::Pt<u32>>>(
    image: &mut I,
    a: P,
    b: P,
    color: I::Pixel,
) {
    let mut a = a.into();
    let mut b = b.into();

    if a.x() > b.x() {
        std::mem::swap(&mut a, &mut b);
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
