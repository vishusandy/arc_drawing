//! # Overview
//!
//! A lightweight drawing library for use with the [`image`] crate that only depends on the [`image`] crate.
//!
//! The following types of drawing functions are provided:
//! - [Lines]
//! - [Conics/circles]
//! - [Shapes]
//!
//![`image`]: https://docs.rs/image/latest/image/
//! [Lines]: lines/index.html
//! [Conics/circles]: conics/index.html
//! [Shapes]: shapes/index.html
//!
//! # Examples
//!
//! #### Lines
//!
//! Solid line between two points:
//! ```
//! # use image::{RgbaImage, Rgba};
//! use freehand::lines::line;
//! # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
//!
//! line(&mut image, (0, 0), (399, 399), Rgba([255, 0, 0, 255]));
//! ```
//!
//! Dashed line between two points:
//!
//! ```
//! # use image::{RgbaImage, Rgba};
//! use freehand::lines::dashed_line;
//! # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
//!
//! let dash: u8 = 2;
//! dashed_line(&mut image, (0, 0), (399, 399), dash, Rgba([255, 0, 0, 255]));
//! ```
//!
//! #### Arcs
//!
//! Draw an arc over the top half of the image:
//!
//! ```
//! use freehand::conics::arc;
//! # use image::{RgbaImage, Rgba};
//! # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
//!
//! /// Note: integers are treated as degrees while floating-point
//! ///  numbers are treated as radians
//! let start = 0; // 0°
//! let end = 180; // 180°
//!
//! let radius = 190;
//! let center = (200, 200);
//!
//! arc(&mut image, start, end, radius, center, Rgba([255, 0, 0, 255]));
//! ```
//!
//! #### Antialiased arcs
//!
//! Draw an antialiased arc over the top half of an image:
//!
//! ```
//! use freehand::conics::antialiased_arc;
//! # use image::{RgbaImage, Rgba};
//! # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
//! # let start = 0; // 0°
//! # let end = 180; // 180°
//! # let radius = 190;
//! # let center = (200, 200);
//!
//! antialiased_arc(&mut image, start, end, radius, center, Rgba([255, 0, 0, 255]));
//! ```
//!
//! #### Annuli (filled donut)
//!
//! Draw a 50px wide annulus over the top half of the image:
//!
//! ```
//! use freehand::conics::annulus;
//! # use image::{RgbaImage, Rgba};
//! # let bg = Rgba([255, 255, 255, 255]); // white
//! # let color = Rgba([255, 0, 0, 255]);
//! # let mut image = RgbaImage::from_pixel(400, 400, bg);
//!
//! let start = 0.0; // equivalent to 0° in radians
//! let end = std::f64::consts::PI; // equivalent to 180° in radians
//!
//! let inner_radius = 150;
//! let outer_radius = 190;
//! # let center = (200, 200);
//!
//! annulus(&mut image, start, end, inner_radius, outer_radius, center, color);
//! ```
//!
//!

#![warn(missing_docs)]

mod aa;
mod angle;
mod annulus;
mod arc;
mod basics;
mod pt;
#[cfg(test)]
mod test;

/// Horizontal, vertical, and diagonal lines with variations for solid, dashed,
/// and alpha blended lines.
///
/// ```
/// # use image::{RgbaImage, Rgba};
/// use freehand::lines::line;
/// # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
///
/// line(&mut image, (0, 0), (399, 399), Rgba([255, 0, 0, 255]));
/// ```
pub mod lines {
    pub use crate::basics::alpha::{
        diagonal_dashed_line_alpha, diagonal_line_alpha, horizontal_dashed_line_alpha,
        horizontal_line_alpha, vertical_dashed_line_alpha, vertical_line_alpha,
    };
    pub use crate::basics::dashed::{
        diagonal_dashed_line, horizontal_dashed_line, vertical_dashed_line,
    };
    pub use crate::basics::straight::{
        dashed_line, diagonal_line, horizontal_line, line, vertical_line, BresIter,
    };
}

/// Conic/circular functions.  Arcs, antialiased arcs, and annuli (filled-donut shapes).
///
/// # Examples
///
/// #### Arcs
///
/// Draw an arc over the top half of the image:
///
/// ```
/// use freehand::conics::arc;
/// # use image::{RgbaImage, Rgba};
/// # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
///
/// /// Note: integers are treated as degrees while floating-point
/// ///  numbers are treated as radians
/// let start = 0; // 0°
/// let end = 180; // 180°
///
/// let radius = 190;
/// let center = (200, 200);
///
/// arc(&mut image, start, end, radius, center, Rgba([255, 0, 0, 255]));
/// ```
///
/// #### Antialiased arcs
///
/// Draw an antialiased arc over the top half of an image:
///
/// ```
/// use freehand::conics::antialiased_arc;
/// # use image::{RgbaImage, Rgba};
/// # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
/// # let start = 0; // 0°
/// # let end = 180; // 180°
/// # let radius = 190;
/// # let center = (200, 200);
///
/// antialiased_arc(&mut image, start, end, radius, center, Rgba([255, 0, 0, 255]));
/// ```
///
/// #### Annuli (filled donut)
///
/// Draw a 50px wide annulus over the top half of the image:
///
/// ```
/// use freehand::conics::annulus;
/// # use image::{RgbaImage, Rgba};
/// # let bg = Rgba([255, 255, 255, 255]); // white
/// # let color = Rgba([255, 0, 0, 255]);
/// # let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// let start = 0.0; // equivalent to 0° in radians
/// let end = std::f64::consts::PI; // equivalent to 180° in radians
///
/// let inner_radius = 150;
/// let outer_radius = 190;
/// # let center = (200, 200);
///
/// annulus(&mut image, start, end, inner_radius, outer_radius, center, color);
/// ```
///
pub mod conics {
    pub use crate::aa::cir_arc::{antialiased_arc, AntialiasedArc};
    pub use crate::annulus::{annulus, Annulus};
    pub use crate::arc::{arc, Arc};
}

/// Functions for drawing basic shapes
pub mod shapes {
    pub use crate::basics::shapes::{
        rectangle, rectangle_alpha, rectangle_filled, rectangle_filled_alpha,
    };
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
