//! # Overview
//!
//! A lightweight drawing library for use with the [`image`] crate that only
//! depends on the [`image`] crate.  The provided functions operate on mutable
//! images.
//!
//! In some cases structs are also provided where it may be appropriate to build
//! a list of drawable objects and handle rendering separately.
//!
//!
//!
//![`image`]: https://docs.rs/image/latest/image/
//!

#![warn(missing_docs)]

mod angle;
mod antialias;
mod pt;
pub(crate) mod translate;
pub(crate) use angle::Angle;

pub mod conics;
pub mod lines;
pub mod ops;
pub mod shapes;
pub use pt::{Point, Pt};

#[cfg(test)]
mod test;
#[cfg(test)]
pub(crate) use test::img::{guidelines, setup};

// TODO:
// add tests for rectangles

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
const RADS: f64 = std::f64::consts::PI / 4.0;
/// Radians in a full circle
const PI2: f64 = std::f64::consts::PI * 2.0;
/// Radians in a single quadrant
const QUAD: f64 = std::f64::consts::PI / 2.0;
/// Tiny amount to subtract from an angle (in radians) to avoid different angles from appearing the same
const TINY: f64 = std::f64::EPSILON * 3.0;

/// A simple helper function that plots `(x, y)` coordinates returned from an iterator.
pub fn draw_iter<I, P, It, T>(image: &mut I, iter: It, color: I::Pixel)
where
    I: image::GenericImage,
    It: Iterator<Item = P>,
    P: crate::pt::Point<T>,
    T: Into<u32>,
{
    for p in iter {
        image.put_pixel(p.x().into(), p.y().into(), color);
    }
}

/// Determine the offset in a byte array for a specified pixel given an image with a specified width.
///
/// Assumes Rgba<u8>
#[inline]
fn rgba_array_index(img_width: u32, x: u32, y: u32) -> usize {
    (y * img_width + x) as usize * std::mem::size_of::<image::Rgba<u8>>()
}

/// Calculate the error for a point in a circle.  Assumes octant 7.
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
