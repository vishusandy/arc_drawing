use crate::blend_at_unchecked;
use image::{Rgba, RgbaImage};

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

    let y1 = y1.min(image.height() - 1);
    let mut y = y0.min(image.height() - 1);
    let mut i = 0;
    while y < y1 {
        let (r, g, b) = (color[0], color[1], color[2]);
        // This is safe due to the min calls above
        unsafe {
            blend_at_unchecked(image, x, y, Rgba([r, g, b, 255]), opacity as f32);
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

    let x1 = x1.min(image.width() - 1);
    let mut x = x0.min(image.width() - 1);
    let mut i = 0;

    while x < x1 {
        let (r, g, b) = (color[0], color[1], color[2]);
        // This is safe due to the min calls above
        unsafe {
            blend_at_unchecked(image, x, y, Rgba([r, g, b, 255]), opacity as f32);
        }
        x = if i == width - 1 { x + width + 1 } else { x + 1 };
        i = if i == width - 1 { 0 } else { i + 1 };
    }
}
