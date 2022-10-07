use crate::blend_at_unchecked;
use crate::pt::Point;
use image::{Rgba, RgbaImage};

pub fn horizontal_line_alpha<P>(
    image: &mut RgbaImage,
    pt: P,
    x1: u32,
    opacity: f32,
    color: Rgba<u8>,
) where
    P: Point<u32>,
{
    if pt.y() < image.height() {
        (pt.x().min(image.width() - 1)..=x1.min(image.width() - 1))
            // This is safe due to the min() calls above
            .for_each(|x| unsafe { blend_at_unchecked(image, x, pt.y(), opacity, color) });
    }
}

pub fn vertical_line_alpha<P>(image: &mut RgbaImage, pt: P, y1: u32, opacity: f32, color: Rgba<u8>)
where
    P: Point<u32>,
{
    if pt.x() < image.width() {
        (pt.y().min(image.height() - 1)..=y1.min(image.height() - 1))
            // This is safe due to the min() calls above
            .for_each(|y| unsafe { blend_at_unchecked(image, pt.x(), y, opacity, color) });
    }
}

pub fn diagonal_line_alpha<P>(
    image: &mut RgbaImage,
    mut a: P,
    mut b: P,
    opacity: f32,
    color: Rgba<u8>,
) where
    P: Point<u32>,
{
    if a.x() > b.x() {
        std::mem::swap(&mut a, &mut b);
    }

    if a.x() >= image.width() || a.y().min(b.y()) >= image.height() {
        return;
    }

    let x0 = a.x().min(image.width() - 1);
    let y0 = a.y().min(image.height() - 1);
    let x1 = b.x().min(image.width() - 1);
    let y1 = b.y().min(image.height() - 1);

    if y0 < y1 {
        let dist = (x1 - x0).min(y1 - y0);
        // This is safe due to the min calls above
        (0..=dist)
            .for_each(|i| unsafe { blend_at_unchecked(image, x0 + i, y0 + i, opacity, color) });
    } else {
        let dist = (x1 - x0).min(y0 - y1);
        // This is safe due to the min calls above
        (0..=dist)
            .for_each(|i| unsafe { blend_at_unchecked(image, x0 + i, y0 + i, opacity, color) });
    }
}

pub fn vertical_dashed_line_alpha<P>(
    image: &mut RgbaImage,
    pt: P,
    mut y1: u32,
    width: u32,
    opacity: f32,
    color: Rgba<u8>,
) where
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
    while y <= y1 {
        let (r, g, b) = (color[0], color[1], color[2]);
        // This is safe due to the min calls above
        unsafe {
            blend_at_unchecked(image, x, y, opacity as f32, Rgba([r, g, b, 255]));
        }
        y = if i == width - 1 { y + width + 1 } else { y + 1 };
        i = if i == width - 1 { 0 } else { i + 1 };
    }
}

pub fn horizontal_dashed_line_alpha<P>(
    image: &mut RgbaImage,
    pt: P,
    mut x1: u32,
    width: u32,
    opacity: f32,
    color: Rgba<u8>,
) where
    P: Point<u32>,
{
    let (mut x0, y) = pt.tuple();
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
    }

    if (width == 0) || (y >= image.height() || (x0 >= image.width())) {
        return;
    }

    let x1 = x1.min(image.width() - 1);
    let mut x = x0.min(image.width() - 1);
    let mut i = 0;

    while x <= x1 {
        let (r, g, b) = (color[0], color[1], color[2]);
        // This is safe due to the min calls above
        unsafe {
            blend_at_unchecked(image, x, y, opacity as f32, Rgba([r, g, b, 255]));
        }
        x = if i == width - 1 { x + width + 1 } else { x + 1 };
        i = if i == width - 1 { 0 } else { i + 1 };
    }
}

pub fn diagonal_dashed_line_alpha<P>(
    image: &mut RgbaImage,
    mut a: P,
    mut b: P,
    width: u32,
    opacity: f32,
    color: Rgba<u8>,
) where
    P: Point<u32>,
{
    if width == 0 {
        return;
    }

    if a.x() > b.x() {
        std::mem::swap(&mut a, &mut b);
    }

    if a.x() >= image.width() || a.y().min(b.y()) >= image.height() {
        return;
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
                blend_at_unchecked(image, x0 + i, y0 + i, opacity, color);
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
                blend_at_unchecked(image, x0 + i, y0 - i, opacity, color);
            }
            let i1 = i + 1;
            let iw = i + width + 1;
            i = if i1 % width == 0 { iw } else { i1 };
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_pixel_colors;

    test_pixel_colors!(
        horizontal_line_alpha,
        horizontal_line_alpha((0, 0), 10, 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );
    test_pixel_colors!(
        horizontal_line_alpha_bounds,
        horizontal_line_alpha((10, 10), 20, 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );

    test_pixel_colors!(
        vertical_line_alpha,
        vertical_line_alpha((0, 0), 10, 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5)],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );
    test_pixel_colors!(
        vertical_line_alpha_bounds,
        vertical_line_alpha((10, 10), 20, 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );

    test_pixel_colors!(
        diagonal_line_alpha,
        diagonal_line_alpha((0, 0), (10, 10), 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );
    test_pixel_colors!(
        diagonal_line_alpha_bounds,
        diagonal_line_alpha((20, 20), (10, 10), 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );

    test_pixel_colors!(
        horizontal_dashed_line_alpha_1px,
        horizontal_dashed_line_alpha((0, 0), 10, 1, 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![(0, 0), (2, 0), (4, 0)],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );
    test_pixel_colors!(
        horizontal_dashed_line_alpha_2px,
        horizontal_dashed_line_alpha((0, 0), 10, 2, 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![(0, 0), (1, 0), (4, 0), (5, 0)],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );
    test_pixel_colors!(
        horizontal_dashed_line_alpha_5px,
        horizontal_dashed_line_alpha((0, 0), 10, 5, 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );
    test_pixel_colors!(
        horizontal_dashed_line_alpha_bounds,
        horizontal_dashed_line_alpha((10, 10), 20, 1, 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );

    test_pixel_colors!(
        vertical_dashed_line_alpha_1px,
        vertical_dashed_line_alpha((0, 0), 10, 1, 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![(0, 0), (0, 2), (0, 4)],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );
    test_pixel_colors!(
        vertical_dashed_line_alpha_2px,
        vertical_dashed_line_alpha((0, 0), 10, 2, 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![(0, 0), (0, 1), (0, 4), (0, 5)],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );
    test_pixel_colors!(
        vertical_dashed_line_alpha_5px,
        vertical_dashed_line_alpha((0, 0), 10, 5, 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );

    test_pixel_colors!(
        vertical_dashed_line_alpha_bounds,
        vertical_dashed_line_alpha((10, 10), 20, 1, 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );

    test_pixel_colors!(
        diagonal_dashed_line_alpha_1px,
        diagonal_dashed_line_alpha((0, 0), (20, 10), 1, 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![(0, 0), (2, 2), (4, 4)],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );
    test_pixel_colors!(
        diagonal_dashed_line_alpha_2px,
        diagonal_dashed_line_alpha((0, 0), (20, 10), 2, 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![(0, 0), (1, 1), (4, 4), (5, 5)],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );
    test_pixel_colors!(
        diagonal_dashed_line_alpha_5px,
        diagonal_dashed_line_alpha((0, 0), (20, 10), 5, 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );
    test_pixel_colors!(
        diagonal_dashed_line_alpha_bounds,
        diagonal_dashed_line_alpha((10, 10), (20, 20), 1, 0.5),
        6,
        image::Rgba([255, 0, 0, 255]),
        &*vec![],
        &*vec![image::Rgba([255, 127, 127, 255]); 6]
    );
}
