mod aa;
mod angle;
mod annulus;
mod arc;
mod arc3;
mod basics;
mod pt;
#[cfg(test)]
mod test;

pub use aa::cir_arc::{antialiased_arc, AAArc};
pub use annulus::{annulus, Annulus};
pub use arc::{arc, Arc};
pub use arc3::Arc as Arc3;
pub use basics::alpha::{
    diagonal_dashed_line_alpha, diagonal_line_alpha, horizontal_dashed_line_alpha,
    horizontal_line_alpha, vertical_dashed_line_alpha, vertical_line_alpha,
};
pub use basics::blend::{blend_at, blend_at_unchecked};
pub use basics::dashed::{diagonal_dashed_line, horizontal_dashed_line, vertical_dashed_line};
pub use basics::shapes::rectangle_filled;
pub use basics::straight::{diagonal_line, horizontal_line, vertical_line};
pub use pt::Pt;

pub(crate) use annulus::translate;

// TODO:
// add dashed_diagonal_line_alpha
// add vertical/horizontal/diagonal_line_alpha (non-dashed variants)
// add diagonal lines and alpha lines to benchmarks

#[cfg(test)]
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Debug;
#[cfg(test)]
const IMG_SIZE: u32 = 400;
#[cfg(test)]
const RADIUS: i32 = 190;
#[cfg(test)]
const CENTER: (i32, i32) = (200, 200);
#[cfg(test)]
const RADIUS_F: f64 = RADIUS as f64;
#[cfg(test)]
const CENTER_F: Pt<f64> = Pt::new(CENTER.0 as f64, CENTER.1 as f64);
#[cfg(test)]
const SHOW_MARKERS: bool = false;

/// range of a single octant in radians
pub const RADS: f64 = std::f64::consts::PI / 4.0;
/// Radians in a full circle
const PI2: f64 = std::f64::consts::PI * 2.0;
/// Radians in a single quadrant
const QUAD: f64 = std::f64::consts::PI / 2.0;
/// Tiny amount to subtract from an angle (in radians) to avoid different angles from appearing the same
const TINY: f64 = std::f64::EPSILON * 3.0;

use angle::Angle;
#[cfg(test)]
pub(crate) use test::img::{guidelines, setup};

pub fn draw_iter<I: image::GenericImage, T: Iterator<Item = (i32, i32)>>(
    image: &mut I,
    iter: T,
    color: I::Pixel,
) {
    for (x, y) in iter {
        image.put_pixel(x as u32, y as u32, color);
    }
}

#[inline(always)]
/// Determine the offset in a byte array for a specified pixel given an image with a specified width.
///
/// Assumes Rgba<u8>
fn rgba_array_index(img_width: u32, x: u32, y: u32) -> usize {
    (y * img_width + x) as usize * 4
}

/// Assumes octant 7
fn calc_error(pt: Pt<f64>, r: i32) -> i32 {
    ((pt.x().round() as f64 + 1.0).powi(2) + (pt.y().round() as f64 - 0.5).powi(2)
        - r.pow(2) as f64)
        .round() as i32
}

#[cfg(test)]
fn logger(level: log::LevelFilter) {
    let _ = env_logger::Builder::new()
        .filter_level(level)
        .format_module_path(false)
        .format_target(false)
        .format_timestamp(None)
        .format_level(false)
        .try_init();
}
