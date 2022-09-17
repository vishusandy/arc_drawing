pub(crate) mod arc;

use crate::Pt;
#[derive(Clone, Debug)]
pub struct AAPt<T> {
    a: Pt<T>,
    b: Pt<T>,
    da: f64, // distance to a (decimal of range: 0..=1.0)
    db: f64,
}
impl<T> AAPt<T> {
    fn new(a: Pt<T>, b: Pt<T>, da: f64) -> Self {
        // debug!("da={} -> db={}", da, 1.0 - da);
        Self {
            a,
            b,
            da,
            db: 1.0 - da,
        }
    }
    fn mult_opac_b(self, i: f64) -> Self {
        Self {
            a: self.a,
            b: self.b,
            da: self.da,
            db: self.db * i,
        }
    }
}

#[inline]
fn rgb_float(c: &[u8], o: f32) -> [f32; 4] {
    [
        c[0] as f32 / 255.0,
        c[1] as f32 / 255.0,
        c[2] as f32 / 255.0,
        o,
    ]
}

impl AAPt<i32> {
    fn draw(&self, image: &mut image::RgbaImage, color: image::Rgba<u8>) {
        let (width, height) = image.dimensions();
        let a = self.a.u32(); // i32 to u32 - negatives wrap around to become large numbers
        let b = self.b.u32();

        if a.x < width && a.y < height {
            // This is safe because the coordinates have already been checked against the width and height
            unsafe {
                blend(image, width, a.x(), a.y(), color, self.db as f32);
            }
        }
        if b.x < width && b.y < height {
            // This is safe because the coordinates have already been checked against the width and height
            unsafe {
                blend(image, width, b.x(), b.y(), color, self.da as f32);
            }
        }
    }
}

#[inline(always)]
/// Blend a specified color into an existing image coordinate.  This ignores `color`'s
/// alpha value and instead uses `opac` which is a floating point number from 0.0 to 1.0.
///
/// A few safety checks are skipped here for performance
///
/// # Safety
/// The x and y coordinates must be less than the width and height, respectively.
/// This is because it uses the `get_unchecked_mut()` method to access the image.
///
/// Also, `opac` must be in the range `(0..=1.0)`.
unsafe fn blend(
    image: &mut image::RgbaImage,
    width: u32,
    x: u32,
    y: u32,
    color: image::Rgba<u8>,
    opac: f32,
) {
    use image::Pixel;
    // https://stackoverflow.com/questions/7438263/alpha-compositing-algorithm-blend-modes#answer-11163848
    // https://docs.rs/image/latest/src/image/color.rs.html#358-370
    let i = vec_idx(width, x, y);
    let bg = unsafe { image.get_unchecked_mut(i..i + 4) };
    let [r1, g1, b1, a1] = mult_alpha(rgba_float(bg));
    let [r2, g2, b2, a2] = mult_alpha(rgb_float(color.channels(), opac));
    let o = 1.0 - opac;
    bg[0] = (r1.mul_add(o, r2) * 255.0).to_int_unchecked(); // ((r2 + r1 * (1.0 - a2)) * 255.0);
    bg[1] = (g1.mul_add(o, g2) * 255.0).to_int_unchecked(); // ((g2 + g1 * (1.0 - a2)) * 255.0);
    bg[2] = (b1.mul_add(o, b2) * 255.0).to_int_unchecked(); // ((b2 + b1 * (1.0 - a2)) * 255.0);
    bg[3] = ((a1 + a2 - a1 * a2) * 255.0).to_int_unchecked();
}

#[inline]
fn rgba_float(c: &[u8]) -> [f32; 4] {
    [
        c[0] as f32 / 255.0,
        c[1] as f32 / 255.0,
        c[2] as f32 / 255.0,
        c[3] as f32 / 255.0,
    ]
}

#[inline]
fn mult_alpha(c: [f32; 4]) -> [f32; 4] {
    [c[0] * c[3], c[1] * c[3], c[2] * c[3], c[3]]
}

#[inline(always)]
fn vec_idx(width: u32, x: u32, y: u32) -> usize {
    (y * width + x) as usize * 4
}

#[allow(dead_code)]
#[inline]
fn opac(d: f64) -> u8 {
    use std::ops::Rem;
    (d * 255.0).round().rem(256.0) as u8
}

#[allow(dead_code)]
#[inline]
fn alpha(a: u8, c: image::Rgba<u8>) -> image::Rgba<u8> {
    image::Rgba([c[0], c[1], c[2], a])
}
