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
    fn reduce_opac_b(self, i: f64) -> Self {
        Self {
            a: self.a,
            b: self.b,
            da: self.da,
            db: self.db * i,
        }
    }
}

impl AAPt<u32> {
    fn draw(&self, image: &mut image::RgbaImage, color: image::Rgba<u8>) {
        use image::Pixel;
        let (width, height) = image.dimensions();
        if self.a.x < width && self.a.y < height {
            let c2 = alpha(opac(self.db), color);
            image.get_pixel_mut(self.a.x(), self.a.y()).blend(&c2);
        }
        if self.b.x < width && self.b.y < height {
            let c1 = alpha(opac(self.da), color);
            image.get_pixel_mut(self.b.x(), self.b.y()).blend(&c1);
        }
    }
}

fn opac(d: f64) -> u8 {
    use std::ops::Rem;
    (d * 255.0).round().rem(256.0) as u8
}

fn alpha(a: u8, c: image::Rgba<u8>) -> image::Rgba<u8> {
    image::Rgba([c[0], c[1], c[2], a])
}

