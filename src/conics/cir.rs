///
pub fn circle<C, I, T>(image: &mut I, radius: T, center: C, color: I::Pixel)
where
    C: crate::pt::Point<T>,
    I: image::GenericImage,
    T: Into<i32>,
{
    let mut octant = crate::conics::Arc::octant(1, radius, center);

    loop {
        if octant.stop() {
            if octant.end() {
                break;
            } else {
                octant.restart();
                continue;
            }
        }

        for i in 1..=8 {
            let pt = octant.coords_oct(i);

            image.put_pixel(pt.x() as u32, pt.y() as u32, color);
            image.put_pixel(pt.x() as u32, pt.y() as u32, color);
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
