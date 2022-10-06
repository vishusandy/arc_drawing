use crate::pt::Point;
use image::GenericImage;

pub fn horizontal_dashed_line<I, P>(image: &mut I, pt: P, mut x1: u32, width: u32, color: I::Pixel)
where
    I: GenericImage,
    P: Point<u32>,
{
    let (mut x0, y) = pt.tuple();

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
    }

    if (width == 0) | (y >= image.height()) | (x0 >= image.width()) {
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

pub fn vertical_dashed_line<I, P>(image: &mut I, pt: P, mut y1: u32, width: u32, color: I::Pixel)
where
    I: GenericImage,
    P: Point<u32>,
{
    let (x, mut y0) = pt.tuple();

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

pub fn diagonal_dashed_line<I, P>(image: &mut I, mut a: P, mut b: P, width: u32, color: I::Pixel)
where
    I: GenericImage,
    P: Point<u32>,
{
    if width == 0 {
        crate::diagonal_line(image, a, b, color);
        return;
    }

    if a.x() > b.x() {
        std::mem::swap(&mut a, &mut b);
    }

    let x0 = a.x().min(image.width() - 1);
    let y0 = a.y().min(image.height() - 1);
    let x1 = b.x().min(image.width() - 1);
    let y1 = b.y().min(image.height() - 1);
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
