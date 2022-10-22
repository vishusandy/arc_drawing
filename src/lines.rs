//! Horizontal, vertical, and diagonal lines with variations for solid, dashed,
//! and alpha blended lines.
//!
//! ```
//! # use image::{RgbaImage, Rgba};
//! use freehand::lines::line;
//! # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
//!
//! line(&mut image, (0, 0), (399, 399), Rgba([255, 0, 0, 255]));
//! ```

mod aa;
mod bres;
mod diagonal;
mod horizontal;
mod straight;
mod thick;
mod vertical;

pub use bres::BresIter;

pub use diagonal::{
    diagonal_dashed_line, diagonal_dashed_line_alpha, diagonal_line, diagonal_line_alpha,
};

pub use horizontal::{
    horizontal_dashed_line, horizontal_dashed_line_alpha, horizontal_line, horizontal_line_alpha,
};

pub use vertical::{
    vertical_dashed_line, vertical_dashed_line_alpha, vertical_line, vertical_line_alpha,
};

pub use straight::{dashed_line, dashed_line_alpha, line, line_alpha, path};

pub use thick::ThickLine;
