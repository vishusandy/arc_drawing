//! # Overview
//!
//! A lightweight drawing library for use with the [`image`] crate.
//!
//! Only depends on the [`image`] crate.
//!
//! # Drawing Functions
//!
//! The following drawing functions are provided:
//! - Vertical, horizontal, and diagonal lines with the following variations:
//!     - Solid
//!     - Alpha blended solid lines
//!     - Dashed
//!     - Alpha blended dashed lines
//! - Conics/circles:
//!     - Solid arcs
//!     - Antialiased arcs
//!     - [Annuli] - filled donut shape
//!
//![`image`]: https://docs.rs/image/latest/image/
//! [Annuli]: https://en.wikipedia.org/wiki/Annulus_(mathematics)

mod aa;
mod angle;
mod annulus;
mod arc;
mod basics;
mod pt;
#[cfg(test)]
mod test;

/// Horizontal, vertical, and diagonal lines with support for solid, dashed, and
/// alpha blended lines.
pub mod lines {
    pub use crate::basics::alpha::{
        diagonal_dashed_line_alpha, diagonal_line_alpha, horizontal_dashed_line_alpha,
        horizontal_line_alpha, vertical_dashed_line_alpha, vertical_line_alpha,
    };
    pub use crate::basics::dashed::{
        diagonal_dashed_line, horizontal_dashed_line, vertical_dashed_line,
    };
    pub use crate::basics::straight::{diagonal_line, horizontal_line, vertical_line};
}

/// Conic/circular functions.  Arcs, antialiased arcs, and annuli (filled-donut shapes).
pub mod conics {
    pub use crate::aa::cir_arc::{antialiased_arc, AntialiasedArc};
    pub use crate::annulus::{annulus, Annulus};
    pub use crate::arc::{arc, Arc};
}

/// Functions for drawing shapes
pub mod shapes {
    pub use crate::basics::shapes::rectangle_filled;
}

/// Helper functions for image operations
pub mod ops {
    pub use crate::basics::blend::{blend_at, blend_at_unchecked};
}

pub use pt::{Point, Pt};

pub(crate) use angle::Angle;
pub(crate) use annulus::translate;
#[cfg(test)]
pub(crate) use test::img::{guidelines, setup};

// TODO:
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
