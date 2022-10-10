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
    opac: f32,
    color: image::Rgba<u8>,
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

/// Blend a specified color into an existing image coordinate.  This ignores `color`'s
/// alpha value and instead uses `opac` which is a floating point number from 0.0 to 1.0.
pub fn blend_at(
    image: &mut image::RgbaImage,
    x: u32,
    y: u32,
    opac: f32,
    color: image::Rgba<u8>,
) -> bool {
    if x < image.width() && y < image.height() && opac >= 0.0 && opac <= 1.0 {
        // this is safe because of the bounds checks
        unsafe {
            blend_at_unchecked(image, x, y, opac, color);
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

#[inline]
fn rgb_float(c: &[u8], o: f32) -> [f32; 4] {
    [
        c[0] as f32 / 255.0,
        c[1] as f32 / 255.0,
        c[2] as f32 / 255.0,
        o,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_blend() {
        let color = image::Rgba([255, 0, 0, 255]);
        let mut image = image::RgbaImage::from_pixel(1, 1, image::Rgba([255, 255, 255, 255]));
        assert!(blend_at(&mut image, 0, 0, 0.5, color));
        assert_eq!(*image.get_pixel(0, 0), image::Rgba([255, 127, 127, 255]));
        assert!(!blend_at(&mut image, 2, 0, 0.5, color));
        assert!(!blend_at(&mut image, 0, 2, 0.5, color));
        assert!(!blend_at(&mut image, 2, 2, 0.5, color));
        assert!(!blend_at(&mut image, 0, 0, 1.1, color));
        assert!(!blend_at(&mut image, 0, 0, -1.1, color));
    }
}
