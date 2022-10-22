use crate::{Pt, CENTER, IMG_SIZE, RADS, SHOW_MARKERS};
use image::Rgba;

#[allow(dead_code)]
pub(crate) const GREEN: Rgba<u8> = Rgba([0, 191, 16, 255]);
#[allow(dead_code)]
pub(crate) const YELLOW: Rgba<u8> = Rgba([255, 217, 0, 255]);
#[allow(dead_code)]
pub(crate) const PURPLE: Rgba<u8> = Rgba([174, 0, 255, 255]);

pub fn blank<P>(size: P) -> image::RgbaImage
where
    P: crate::pt::Point<u32>,
{
    image::RgbaImage::from_pixel(size.x(), size.y(), image::Rgba([255, 255, 255, 255]))
}

#[allow(dead_code)]
pub fn setup_ellipse(h: i32, v: i32, c: (i32, i32)) -> image::RgbaImage {
    let mut image =
        image::RgbaImage::from_pixel(IMG_SIZE, IMG_SIZE, image::Rgba([255, 255, 255, 255]));
    let center = c;
    imageproc::drawing::draw_hollow_ellipse_mut(
        &mut image,
        center,
        h,
        v,
        image::Rgba([0, 0, 255, 255]),
    );
    image
}

pub fn circle_guides(r: i32) -> image::RgbaImage {
    let mut image = guidelines();
    let center = CENTER;
    imageproc::drawing::draw_hollow_circle_mut(
        &mut image,
        center,
        r,
        image::Rgba([0, 0, 255, 255]),
    );
    if SHOW_MARKERS {
        draw_markers(&mut image, r, center);
    }
    image
}

pub(crate) fn guidelines() -> image::RgbaImage {
    let mut image =
        image::RgbaImage::from_pixel(IMG_SIZE, IMG_SIZE, image::Rgba([255, 255, 255, 255]));
    // Draw guide lines
    imageproc::drawing::draw_line_segment_mut(
        &mut image,
        (IMG_SIZE as f32 / 2.0, 0.0),
        (IMG_SIZE as f32 / 2.0, IMG_SIZE as f32),
        image::Rgba([252, 190, 3, 255]),
    );
    imageproc::drawing::draw_line_segment_mut(
        &mut image,
        (0.0, IMG_SIZE as f32 / 2.0),
        (IMG_SIZE as f32, IMG_SIZE as f32 / 2.0),
        image::Rgba([252, 190, 3, 255]),
    );
    imageproc::drawing::draw_line_segment_mut(
        &mut image,
        (0.0, 0.0),
        (IMG_SIZE as f32, IMG_SIZE as f32),
        image::Rgba([255, 242, 206, 255]),
    );
    imageproc::drawing::draw_line_segment_mut(
        &mut image,
        (0.0, IMG_SIZE as f32),
        (IMG_SIZE as f32, 0.0),
        image::Rgba([255, 242, 206, 255]),
    );
    image
}

#[cfg(test)]
pub(crate) fn draw_markers(image: &mut image::RgbaImage, r: i32, c: (i32, i32)) {
    let rads = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    for o in 0..8 {
        let oa = o as f64 * RADS;
        for a in rads {
            let rad = a * RADS + oa;
            plot_marker(image, r, rad, c, image::Rgba([0, 255, 0, 255]));
        }
    }
}

#[cfg(test)]
fn plot_marker(
    image: &mut image::RgbaImage,
    r: i32,
    angle: f64,
    c: (i32, i32),
    color: image::Rgba<u8>,
) {
    let Pt { x, y }: Pt<i32> = Pt::from_radian(angle, r, c).into();
    image.put_pixel(x as u32, y as u32, color);
}
