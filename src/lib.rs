mod aa;
mod angle;
mod annulus;
mod arc;
mod fp;
mod pt;

// STATUS
//  arc::Arc could use a lot of love, or a rewrite

pub use aa::arc::AAArc;
pub use annulus::Annulus;
pub use arc::Arc;
pub use fp::{arc_integer, arc_midpoint};

pub(crate) use angle::Angle;
pub use pt::Pt;

pub const IMG_SIZE: u32 = 600;
pub const RADIUS: i32 = 240;
pub const CENTER: (i32, i32) = (300, 300);
pub const RADIUS_F: f64 = RADIUS as f64;
pub const CENTER_F: Pt<f64> = Pt::new(CENTER.0 as f64, CENTER.1 as f64);

const PI2: f64 = std::f64::consts::PI * 2.0;
const QUAD: f64 = std::f64::consts::PI / 2.0;
const TINY: f64 = std::f64::EPSILON * 3.0;

pub const RADS: f64 = std::f64::consts::PI / 4.0; // range of a single octant

#[cfg(test)]
const OR: f64 = std::f64::consts::PI / 4.0;
#[cfg(test)]
const SHOW_MARKERS: bool = false;

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
fn vec_idx(width: u32, x: u32, y: u32) -> usize {
    (y * width + x) as usize * 4
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

    fn get_y(&self, x: i32) -> Option<i32> {
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

#[cfg(test)]
pub fn blank() -> image::RgbaImage {
    image::RgbaImage::from_pixel(IMG_SIZE, IMG_SIZE, image::Rgba([255, 255, 255, 255]))
}

#[cfg(test)]
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

#[cfg(test)]
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

#[cfg(test)]
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

#[cfg(test)]
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
