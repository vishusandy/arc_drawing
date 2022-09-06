pub(crate) mod octs;

pub fn draw_bres_circle(
    image: &mut image::RgbaImage,
    r: i32,
    c: (i32, i32),
    color: image::Rgba<u8>,
) {
    crate::draw_iter(image, octs::Oct1::full(r, c), color);
    crate::draw_iter(image, octs::Oct2::full(r, c), color);
    crate::draw_iter(image, octs::Oct3::full(r, c), color);
    crate::draw_iter(image, octs::Oct4::full(r, c), color);
    crate::draw_iter(image, octs::Oct5::full(r, c), color);
    crate::draw_iter(image, octs::Oct6::full(r, c), color);
    crate::draw_iter(image, octs::Oct7::full(r, c), color);
    crate::draw_iter(image, octs::Oct8::full(r, c), color);
}

pub fn full_circle(image: &mut image::RgbaImage, r: i32, c: (i32, i32), color: image::Rgba<u8>) {
    for i in 0..8 {
        full_arc_oct(image, r, c, i, color);
    }
}

pub fn full_arc_oct(
    image: &mut image::RgbaImage,
    r: i32,
    c: (i32, i32),
    oct: u8,
    color: image::Rgba<u8>,
) {
    let mut x: i32 = 0;
    let mut y: i32 = r;
    let mut d: i32 = 1 - r;
    let f: fn((i32, i32)) -> (i32, i32) = match oct {
        0 => octs::bres_to::o1,
        1 => octs::bres_to::o2,
        2 => octs::bres_to::o3,
        3 => octs::bres_to::o4,
        4 => octs::bres_to::o5,
        5 => octs::bres_to::o6,
        6 => octs::bres_to::o7,
        7 => octs::bres_to::o8,
        _ => panic!("invalid octant specified"),
    };

    while y >= x {
        let (px, py) = f((x, y));
        image.put_pixel((px + c.0) as u32, (py + c.1) as u32, color);
        x += 1;
        if d > 0 {
            y -= 1;
            d += 2 * (x - y) + 1;
        } else {
            d += 2 * x + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_full_arc() -> Result<(), image::ImageError> {
        let mut image = crate::setup(crate::RADIUS);
        full_arc_oct(
            &mut image,
            crate::RADIUS,
            crate::CENTER,
            7,
            image::Rgba([255, 0, 0, 255]),
        );
        image.save("bres_full_arc.png")
    }
}
