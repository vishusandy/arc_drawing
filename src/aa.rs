pub(crate) mod cir_arc;
mod ellipse_arc;
use crate::basics::blend::blend_at_unchecked;

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

impl AAPt<i32> {
    fn draw(&self, image: &mut image::RgbaImage, color: image::Rgba<u8>) {
        let (width, height) = image.dimensions();
        let a = self.a.u32(); // i32 to u32 - negatives wrap around to become large numbers
        let b = self.b.u32();

        if (a.x < width) & (a.y < height) {
            // This is safe because the coordinates have already been checked against the width and height
            unsafe {
                blend_at_unchecked(image, a.x(), a.y(), color, self.db as f32);
            }
        }
        if (b.x < width) & (b.y < height) {
            // This is safe because the coordinates have already been checked against the width and height
            unsafe {
                blend_at_unchecked(image, b.x(), b.y(), color, self.da as f32);
            }
        }
    }
}
