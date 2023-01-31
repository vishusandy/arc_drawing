use crate::conics;

/// Draws a full circle.
///
/// ```
/// use freehand::conics::circle;
/// # use image::{RgbaImage, Rgba};
/// # let mut image = RgbaImage::new(400, 400);
///
/// circle(&mut image, 380, (200, 200), Rgba([255, 0, 0, 255]));
/// ```
///
/// Uses [`conics::Arc`] to calculate a single octant and draw those pixels
/// in all octants.
pub fn circle<C, I, T>(image: &mut I, radius: T, center: C, color: I::Pixel)
where
    C: crate::pt::Point<T>,
    I: image::GenericImage,
    T: Into<i32> + Copy,
{
    let mut octant = conics::Arc::octant(1, radius, center);

    loop {
        if octant.stop() {
            if octant.end() {
                break;
            }
            octant.restart();
            continue;
        }

        // draw the pixel in all 8 octants
        for i in 1..=8 {
            let pt: Result<crate::Pt<u32>, &'static str> = octant.coords_oct(i).try_into();
            if let Ok(pt) = pt {
                if pt.x() < image.width() && pt.y() < image.height() {
                    image.put_pixel(pt.x(), pt.y(), color);
                }
            }
        }

        octant.inc();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn circle() -> Result<(), image::ImageError> {
        crate::logger(crate::LOG_LEVEL);

        let r = 190;
        let c = (200, 200);

        let mut image = crate::circle_guides(r);

        super::circle(&mut image, r, c, image::Rgba([255, 0, 0, 255]));

        image.save("images/circle.png")
    }
}
