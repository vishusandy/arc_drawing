#![allow(dead_code)]
#![allow(unused_variables)]
mod aa;
mod angle;
mod annulus;
mod arc;
mod bres;
mod fp;
mod pt;

// STATUS
//  arc::Arc is the full implementation of partial circular arc drawing, however it could use some
//  performance improvements and optimizations :\

pub use aa::arc::AAArc;
pub use annulus::Annulus;
pub use arc::Arc;
pub use bres::octs::{Oct1, Oct2};
pub use bres::{draw_bres_circle, full_arc_oct, full_circle};
pub use fp::{arc_integer, arc_midpoint};

pub(crate) use angle::Angle;
pub use pt::Pt;

pub const IMG_SIZE: u32 = 600;
pub const RADIUS: i32 = 240;
pub const CENTER: (i32, i32) = (300, 300);
pub const RADIUS_F: f64 = RADIUS as f64;
pub const CENTER_F: (f64, f64) = (CENTER.0 as f64, CENTER.1 as f64);
const SHOW_MARKERS: bool = false;

const OR: f64 = std::f64::consts::PI / 4.0;
pub const RADS: f64 = std::f64::consts::PI / 4.0; // range of a single octant

fn logger(level: log::LevelFilter) {
    let _ = env_logger::Builder::new()
        .filter_level(level)
        .format_module_path(false)
        .format_target(false)
        .format_timestamp(None)
        .format_level(false)
        .try_init();
}

pub fn blank() -> image::RgbaImage {
    image::RgbaImage::from_pixel(IMG_SIZE, IMG_SIZE, image::Rgba([255, 255, 255, 255]))
}

pub fn draw_iter<T: Iterator<Item = (i32, i32)>>(
    image: &mut image::RgbaImage,
    iter: T,
    color: image::Rgba<u8>,
) {
    // let iter = Oct1::new(r, c);
    for (x, y) in iter {
        image.put_pixel(x as u32, y as u32, color);
    }
}

pub fn setup(r: i32) -> image::RgbaImage {
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

fn guidelines() -> image::RgbaImage {
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

fn draw_markers(image: &mut image::RgbaImage, r: i32, c: (i32, i32)) {
    let rads = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    for o in 0..8 {
        let oa = o as f64 * OR;
        for a in rads {
            let rad = a * OR + oa;
            plot_marker(image, r, rad, c, image::Rgba([0, 255, 0, 255]));
        }
    }
}

fn plot_marker(
    image: &mut image::RgbaImage,
    r: i32,
    angle: f64,
    c: (i32, i32),
    color: image::Rgba<u8>,
) {
    let pt::Pt { x, y }: pt::Pt<i32> = pt::Pt::from_radian(angle, r, c).into();
    image.put_pixel(x as u32, y as u32, color);
}
