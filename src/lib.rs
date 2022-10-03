mod aa;
mod angle;
mod annulus;
mod arc;
mod basics;
mod pt;
#[cfg(test)]
mod test;

pub use aa::cir_arc::AAArc;
pub use annulus::Annulus;
pub use arc::Arc;
pub use basics::blend::{blend_at, blend_at_unchecked};
pub use basics::{
    horizontal_dashed_line, horizontal_dashed_line_blend, horizontal_line, rectangle_filled,
    vertical_dashed_line, vertical_dashed_line_blend, vertical_line,
};
pub use pt::Pt;

// STATUS
//  arc::Arc could use a lot of love, or a rewrite

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

pub fn draw_iter<T: Iterator<Item = (i32, i32)>>(
    image: &mut image::RgbaImage,
    iter: T,
    color: image::Rgba<u8>,
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

#[derive(Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
    d: i32,  // decision parameter
    ex: i32, // ending x coordinate
    ey: i32, // ending y coordinate
    r: i32,
}
impl Pos {
    fn new(start: f64, end: f64, oct: u8, r: i32, c: Pt<i32>) -> Self {
        let mut start = Pt::from_radian(start, r, c.into()).real_to_iter(oct, c.into());
        let mut end = Pt::from_radian(end, r, c.into()).real_to_iter(oct, c.into());
        let Pt { mut x, mut y } = start.i32();
        let Pt {
            x: mut ex,
            y: mut ey,
        } = end.i32();
        if oct % 2 == 0 {
            std::mem::swap(&mut start, &mut end);
            std::mem::swap(&mut x, &mut ex);
            std::mem::swap(&mut y, &mut ey);
        }
        let d: i32 = ((start.x().round() as f64 + 1.0).powi(2)
            + (start.y().round() as f64 - 0.5).powi(2)
            - r.pow(2) as f64)
            .round() as i32;
        Self { x, y, d, ex, ey, r }
    }

    /// Get `self.y` when `self.x` is the same as the specified `x`
    fn get_matching_y(&self, x: i32) -> Option<i32> {
        if x == self.x {
            Some(self.y)
        } else {
            None
        }
    }

    fn inc(&mut self) {
        if self.x >= self.ex {
            return;
        }
        self.x += 1;
        if self.d > 0 {
            self.y -= 1;
            self.d += 2 * (self.x - self.y) + 1;
        } else {
            self.d += 2 * self.x + 1;
        }
    }
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
