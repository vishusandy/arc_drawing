/// Blend a specified color into an existing image coordinate.  This ignores `color`'s
/// alpha value and instead uses `opac` which is a floating point number from 0.0 to 1.0.
///
/// A few safety checks are skipped here for performance
///
/// # Safety
/// The x and y coordinates must be less than the image width and height, respectively.
///
/// Also, `opac` must be in the range `(0..=1.0)`.
pub unsafe fn blend_at_unchecked(
    image: &mut image::RgbaImage,
    x: u32,
    y: u32,
    color: image::Rgba<u8>,
    opac: f32,
) {
    use image::Pixel;
    // https://stackoverflow.com/questions/7438263/alpha-compositing-algorithm-blend-modes#answer-11163848
    // https://docs.rs/image/latest/src/image/color.rs.html#358-370
    let i = crate::rgba_array_index(image.width(), x, y);
    let bg = image.get_unchecked_mut(i..i + std::mem::size_of::<image::Rgba<u8>>());
    let [r1, g1, b1, a1] = mult_alpha(rgba_float(bg));
    let [r2, g2, b2, a2] = mult_alpha(rgb_float(color.channels(), opac));
    let o = 1.0 - opac;
    bg[0] = (r1.mul_add(o, r2) * 255.0).to_int_unchecked(); // ((r2 + r1 * (1.0 - a2)) * 255.0);
    bg[1] = (g1.mul_add(o, g2) * 255.0).to_int_unchecked(); // ((g2 + g1 * (1.0 - a2)) * 255.0);
    bg[2] = (b1.mul_add(o, b2) * 255.0).to_int_unchecked(); // ((b2 + b1 * (1.0 - a2)) * 255.0);
    bg[3] = ((a1 + a2 - a1 * a2) * 255.0).to_int_unchecked();
}

pub fn blend_at(
    image: &mut image::RgbaImage,
    x: u32,
    y: u32,
    color: image::Rgba<u8>,
    opac: f32,
) -> bool {
    if x < image.width() && y < image.height() && opac >= 0.0 && opac <= 1.0 {
        // this is safe because of the bounds checks
        unsafe {
            blend_at_unchecked(image, x, y, color, opac);
        }
        true
    } else {
        false
    }
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

#[inline]
fn rgb_float(c: &[u8], o: f32) -> [f32; 4] {
    [
        c[0] as f32 / 255.0,
        c[1] as f32 / 255.0,
        c[2] as f32 / 255.0,
        o,
    ]
}
