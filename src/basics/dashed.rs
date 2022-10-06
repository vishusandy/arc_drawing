use image::GenericImage;

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

    let x1 = x1.min(image.width() - 1);
    let mut x = x0.min(image.width() - 1);
    let mut i = 0;

    while x < x1 {
        // This is safe due to the min calls above
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

    let y1 = y1.min(image.height() - 1);
    let mut y = y0.min(image.height() - 1);
    let mut i = 0;

    while y < y1 {
        // This is safe due to the min calls above
        unsafe {
            image.unsafe_put_pixel(x, y, color);
        }
        y = if i == width - 1 { y + width + 1 } else { y + 1 };
        i = if i == width - 1 { 0 } else { i + 1 };
    }
}

pub fn diagonal_dashed_line<I: GenericImage>(
    image: &mut I,
    mut x0: u32,
    mut y0: u32,
    mut x1: u32,
    mut y1: u32,
    width: u32,
    color: I::Pixel,
) {
    if width == 0 {
        crate::diagonal_line(image, x0, y0, x1, y1, color);
        return;
    }

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let x0 = x0.min(image.width() - 1);
    let y0 = y0.min(image.height() - 1);
    let x1 = x1.min(image.width() - 1);
    let y1 = y1.min(image.height() - 1);
    let mut i = 0;

    if y0 < y1 {
        let dist = (x1 - x0).min(y1 - y0);
        while i <= dist {
            // This is safe due to the min calls above
            unsafe {
                image.unsafe_put_pixel(x0 + i, y0 + i, color);
            }
            let i1 = i + 1;
            let iw = i + width + 1;
            i = if i1 % width == 0 { iw } else { i1 };
        }
    } else {
        let dist = (x1 - x0).min(y0 - y1);
        while i <= dist {
            // This is safe due to the min calls above
            unsafe {
                image.unsafe_put_pixel(x0 + i, y0 - i, color);
            }
            let i1 = i + 1;
            let iw = i + width + 1;
            i = if i1 % width == 0 { iw } else { i1 };
        }
    }
}
