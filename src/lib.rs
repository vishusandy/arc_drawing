#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_precision_loss)]

//! # Overview
//!
//! A crate that provides various drawing functions for use with the [`image`]
//! crate.
//!
//! It is designed to be easy to use flexible, and lightweight - depending only
//! on [`image`].
//!
//! Current features:
//! - line drawing functions
//! - circles, circular arcs, and annuli (filled donut shapes)
//! - rectangles
//!
//! # Usage
//!
//! First add freehand to your `Cargo.toml`'s dependencies:
//!
//! ```toml
//! [dependencies]
//! freehand = "0.1.0"
//! ```
//!
//! Using the [`Draw`] struct is preferred over calling the functions directly
//! as it is slightly more ergonomic to work with.  The `Draw` struct can be
//! created using the [`new()`] function or [`Draw::new()`].  Note: not all
//! drawing functions have `Draw` methods; some of the less common functions
//! must be called directly.
//!
//! ```
//! use image::{RgbaImage, Rgba};
//! let mut image = RgbaImage::new(400, 400);
//!
//! let draw = freehand::new(&mut image);
//!
//! draw.line((0, 0), (399,399), Rgba([255, 0, 0, 255]))
//!     .dashed_line((0, 399), (399, 0), 2, Rgba([255, 0, 0, 255]))
//!     .rectangle((150, 150), 100, 100, Rgba([0, 255, 0, 255]))
//!     .thick_line((200, 50), (350, 200), 3.5, Rgba([255, 0, 0, 255]))
//!     .arc(20, 160, 100, (200, 200), Rgba([0, 0, 255, 255]));
//! ```
//!
//! # Note on angles
//!
//! Angles are treated differently based on their type.  Floating-point types
//! will be in radians, while integer types will be treated as degrees (and
//! silently converted to radians).  The [`to_radians()`] function can be used
//! to convert a floating-point in degrees to a floating-point in radians.
//!
//! The rationale is that it is more common to calculate precise angles using
//! radians (where floating-points would be needed) rather than degrees, which
//! are more commonly used to specify well-known degrees such as 45°, 90°,
//! 180°, etc.
//!
//! [`to_radians()`]: https://doc.rust-lang.org/std/primitive.f64.html#method.to_radians
//!
//! # Note on opacity
//!
//! Drawing functions that take a specified opacity currently only work
//! with [`image::RgbaImage`].
//!
//! It should also be noted that unless specified otherwise they will take an
//! `opacity` parameter that will be used instead of the specified color's
//! alpha channel for blending the colors together.  The alpha channel in the
//! specified color will only be used to calculate the resulting color's alpha
//! channel (not how much of each color is mixed with the existing color).
//!
//! The rationale for this is that it is often more convenient to calculate
//! opacities using `f32` than integers.  Using an explicit `f32` parameter
//! to specify the opacity prevents unnecessary conversions between `u8` and
//! `f32`. As a result, this approach may be more intuitive to blend colors
//! together when the alpha channel values will primarily be the same.
//!
//![`image`]: https://docs.rs/image/latest/image/
//!

mod angle;
mod antialias;
mod pt;

pub(crate) mod draw;
pub(crate) mod translate;

pub mod conics;
pub mod lines;
pub mod ops;
pub mod shapes;

pub use angle::Angle;
pub use draw::{new, Draw};
pub use pt::{Point, Pt};

#[cfg(test)]
mod test;

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use test::img::{circle_guides, guidelines, GREEN, PURPLE, YELLOW};

#[cfg(test)]
use test::{logger, CENTER, IMG_SIZE, LOG_LEVEL, RADIUS, RADIUS_F, SHOW_MARKERS};

/// Range of a single octant in radians
const RADS: f64 = std::f64::consts::PI / 4.0;
/// Radians in a full circle
const PI2: f64 = std::f64::consts::PI * 2.0;
/// Range of a single quadrant in radians
const QUAD: f64 = std::f64::consts::PI / 2.0;
/// Tiny amount to subtract from an angle (in radians) to avoid different angles from appearing the same
const TINY: f64 = std::f64::EPSILON * 10.0; // multiply by a number greater than 2.0 to avoid floating-point inaccuracy

/// A simple helper function that draws `(x, y)` coordinates returned from an iterator.
pub fn draw_iter<I, P, It, T>(image: &mut I, iter: It, color: I::Pixel)
where
    I: image::GenericImage,
    It: Iterator<Item = P>,
    P: crate::pt::Point<T>,
    T: Into<u32> + Copy,
{
    for p in iter {
        let (x, y) = p.tuple();
        let (x, y) = (x.into(), y.into());
        if x < image.width() && y < image.height() {
            image.put_pixel(x, y, color);
        }
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
    ((pt.x().round() + 1.0).powi(2) + (pt.y().round() - 0.5).powi(2) - r.pow(2) as f64).round()
        as i32
}

/// Calculate the slope of a line
fn calc_slope(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
    (y2 as f64 - y1 as f64) / (x2 as f64 - x1 as f64)
}

/// Calculate the y intercept (y value for x=0)
#[allow(dead_code)]
fn calc_intercept(x: i32, y: i32, slope: f64) -> f64 {
    slope * (-x as f64) + y as f64
}
