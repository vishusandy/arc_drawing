use crate::ops::blend_at;
use crate::{Point, Pt};

/// ```
/// # use image::{RgbaImage, Rgba};
/// use freehand::lines::thick_line;
/// # let mut image = RgbaImage::from_pixel(400, 400, Rgba([255, 255, 255, 255]));
///
/// thick_line(&mut image, (0, 0), (399, 399), 4.5, Rgba([255, 0, 0, 255]));
/// ```
// http://members.chello.at/~easyfilter/bresenham.html
// http://members.chello.at/~easyfilter/canvas.html
pub fn thick_line<P, T>(image: &mut image::RgbaImage, a: P, b: P, wd: f32, color: image::Rgba<u8>)
where
    P: Point<T>,
    T: Into<i32>,
{
    let Pt {
        x: mut x0,
        y: mut y0,
    } = Pt::new(a.x().into(), a.y().into());

    let Pt { x: x1, y: y1 } = Pt::new(b.x().into(), b.y().into());

    let dx = (x1 - x0).abs(); // x difference
    let dy = (y1 - y0).abs(); // y difference

    // amount to added to x
    let sx = match x0 < x1 {
        true => 1,
        false => -1,
    };
    // amount added to y
    let sy = match y0 < y1 {
        true => 1,
        false => -1,
    };

    let mut err = dx - dy;
    let ed = match dx + dy == 0 {
        true => 1.0,
        false => ((dx as f32 * dx as f32) + (dy as f32 * dy as f32)).sqrt(),
    };

    let wd = (wd + 1.0) / 2.0;

    loop {
        {
            let o = (((err - dx + dy).abs() as f32) / ed - wd + 1.0).max(0.0);
            blend_at(image, x0 as u32, y0 as u32, 1.0 - o, color);
        }
        let mut e2 = err;
        let mut x2 = x0;
        if 2 * e2 >= -dx {
            // x step
            e2 += dy;
            let mut y2 = y0;
            while (e2 as f32) < ed * wd && (y1 != y2 || dx > dy) {
                y2 += sy;
                let o = (e2.abs() as f32 / ed - wd + 1.0).max(0.0);
                blend_at(image, x0 as u32, y2 as u32, 1.0 - o, color);
                e2 += dx;
            }
            if x0 == x1 {
                break;
            }
            e2 = err;
            err -= dy;
            x0 += sx;
        }
        if 2 * e2 <= dy {
            // y step
            e2 = dx - e2;
            while (e2 as f32) < (ed * wd) && (x1 != x2 || dx < dy) {
                x2 += sx;
                let o = (e2.abs() as f32 / ed - wd + 1.0).max(0.0);
                blend_at(image, x2 as u32, y0 as u32, 1.0 - o, color);
                e2 += dy;
            }
            if y0 == y1 {
                break;
            }
            err += dx;
            y0 += sy;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn thick_aa_line() -> Result<(), image::ImageError> {
        let mut image = image::RgbaImage::from_pixel(400, 400, image::Rgba([255, 255, 255, 255]));
        super::thick_line(
            &mut image,
            (10, 10),
            (50, 50),
            5.5,
            image::Rgba([255, 0, 0, 255]),
        );
        image.save("images/thick_aa_line.png")
    }
}
