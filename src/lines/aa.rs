use crate::ops::blend_at;
use crate::Pt;
use image::{Rgba, RgbaImage};

// http://members.chello.at/~easyfilter/bresenham.html
// http://members.chello.at/~easyfilter/canvas.html
fn thick_line_aa(
    image: &mut image::RgbaImage,
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    wd: f32,
    color: image::Rgba<u8>,
) {
    use image::Pixel;
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = match x0 < x1 {
        true => 1,
        false => -1,
    };
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
    let mut i = 0;
    while i < 1000 {
        {
            // let o = ((err - dx + dy).abs() as f32 / (ed as f32) - wd + 1.0).max(0.0);
            let o = (((err - dx + dy).abs() as f32) / (ed as f32) - wd + 1.0).max(0.0);
            println!("i={i} x0={x0} y0={y0} o={:.2}", o);
            // blend_at(image, x0 as u32, y0 as u32, 1.0 - o, color);
            // let c = image::Rgba([color[0], color[1], color[2], (o * 255.0) as u8]);
            image
                .get_pixel_mut(x0 as u32, y0 as u32)
                .blend(&color_alpha(color, 1.0 - o));
        }
        let mut e2 = err;
        let mut x2 = x0;
        if 2 * e2 >= -dx {
            // x step
            e2 += dy;
            let mut y2 = y0;
            while (e2 as f32) < (ed as f32) * wd && (y1 != y2 || dx > dy) {
                y2 += sy;
                let o = (e2.abs() as f32 / ed - wd + 1.0).max(0.0);
                println!("  x step: x={x0} y={y2} o={:.2}", 1.0 - o);
                // blend_at(image, x0 as u32, y2 as u32, 1.0 - o, color);
                image
                    .get_pixel_mut(x0 as u32, y2 as u32)
                    .blend(&color_alpha(color, 1.0 - o));
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
            while (e2 as f32) < (ed as f32 * wd) && (x1 != x2 || dx < dy) {
                x2 += sx;
                let o = (e2.abs() as f32 / (ed as f32) - wd + 1.0).max(0.0);
                println!("  y step: x={x2} y={y0} o={:.2}", 1.0 - o);
                // blend_at(image, x2 as u32, y0 as u32, 1.0 - o, color);
                image
                    .get_pixel_mut(x2 as u32, y0 as u32)
                    .blend(&color_alpha(color, 1.0 - o));
                e2 += dy;
            }
            if y0 == y1 {
                break;
            }
            err += dx;
            y0 += sy;
        }

        i += 1;
    }
}

fn color_alpha(c: image::Rgba<u8>, opacity: f32) -> image::Rgba<u8> {
    image::Rgba([c[0], c[1], c[2], (opacity * 255.0) as u8])
}

#[cfg(test)]
mod tests {
    use crate::Pt;
    #[test]
    fn aa_line() -> Result<(), image::ImageError> {
        let mut image = image::RgbaImage::from_pixel(400, 400, image::Rgba([255, 255, 255, 255]));
        // super::thick_line_aa(
        //     &mut image,
        //     Pt::new(20, 20),
        //     Pt::new(380, 300),
        //     2.0,
        //     image::Rgba([255, 0, 0, 255]),
        // );
        super::thick_line_aa(
            &mut image,
            10,
            10,
            30,
            20,
            3.5,
            image::Rgba([255, 0, 0, 255]),
        );
        image.save("images/aa_line.png")
    }
}
