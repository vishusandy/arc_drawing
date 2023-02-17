use crate::ops::blend_at_unchecked;

use crate::Pt;

/// An antialiased point.  Contains two coordinates and their corresponding opacities.
// allow because no unsafe methods are used when deserializing
#[allow(clippy::unsafe_derive_deserialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug)]
pub struct AAPt<T>
where
    T: PartialOrd,
{
    /// First point
    pub a: Pt<T>,
    /// Second point
    pub b: Pt<T>,
    /// Point `a`'s opacity (distance to a) - range 0..=1.0
    pub ob: f64,
    /// Point `b`'s opacity (distance to b) range 0..=1.0
    pub oa: f64,
}

impl<T> AAPt<T>
where
    T: PartialOrd,
{
    /// Create a new antialiased point.
    pub(crate) fn new(a: Pt<T>, b: Pt<T>, ob: f64) -> Self {
        Self {
            a,
            b,
            oa: 1.0 - ob,
            ob,
        }
    }

    /// Used to adjust the opacity of the first pixel.
    pub(crate) fn mult_opac_a(self, i: f64) -> Self {
        Self {
            a: self.a,
            b: self.b,
            oa: self.oa * i,
            ob: self.ob,
        }
    }
}

impl AAPt<i32> {
    /// Draw an antialiased point by blending the two pixels into an image.
    pub(crate) fn draw(&self, image: &mut image::RgbaImage, color: image::Rgba<u8>) {
        let (width, height) = image.dimensions();
        let a = self.a.u32();
        let b = self.b.u32();

        if (a.x < width) & (a.y < height) {
            // This is safe because the coordinates have already been checked against the image bounds
            // Invalid opacity values are safe, but may produce weird blending
            unsafe {
                blend_at_unchecked(image, a.x(), a.y(), self.oa as f32, color);
            }
        }

        if (b.x < width) & (b.y < height) {
            // This is safe because the coordinates have already been checked against the image bounds
            // Invalid opacity values are safe, but may produce weird blending
            unsafe {
                blend_at_unchecked(image, b.x(), b.y(), self.ob as f32, color);
            }
        }
    }
}
