fn main() {
    full_quadrant_arc().unwrap();
}

const IMG_SIZE: u32 = 600;
fn guidelines() -> image::RgbaImage {
    let mut image = image::RgbaImage::from_vec(
        IMG_SIZE,
        IMG_SIZE,
        Vec::from([255; (IMG_SIZE * IMG_SIZE) as usize * 4]),
    )
    .unwrap();
    // Draw guide lines
    imageproc::drawing::draw_line_segment_mut(
        &mut image,
        (IMG_SIZE as f32 / 2.0, 0.0),
        (IMG_SIZE as f32 / 2.0, IMG_SIZE as f32),
        image::Rgba([252, 190, 3, 255]),
    );
    imageproc::drawing::draw_line_segment_mut(
        &mut image,
        (0.0, IMG_SIZE as f32 / 2.0),
        (IMG_SIZE as f32, IMG_SIZE as f32 / 2.0),
        image::Rgba([252, 190, 3, 255]),
    );
    imageproc::drawing::draw_line_segment_mut(
        &mut image,
        (0.0, 0.0),
        (IMG_SIZE as f32, IMG_SIZE as f32),
        image::Rgba([255, 242, 206, 255]),
    );
    imageproc::drawing::draw_line_segment_mut(
        &mut image,
        (0.0, IMG_SIZE as f32),
        (IMG_SIZE as f32, 0.0),
        image::Rgba([255, 242, 206, 255]),
    );
    image
}

fn full_quadrant_arc() -> Result<(), image::ImageError> {
    let mut image: image::RgbaImage = guidelines();
    let c: (i32, i32) = (IMG_SIZE as i32 / 2, IMG_SIZE as i32 / 2);
    let r: i32 = 150;

    imageproc::drawing::draw_hollow_circle_mut(&mut image, c, r, image::Rgba([0, 0, 255, 255]));
    let mut x: i32 = 0;
    let mut y: i32 = r;
    let mut d: i32 = 1 - r;
    let mut quad: u8 = 3;

    // p = (x+1)² + (y - ½)² - r²

    loop {
        image.put_pixel(
            (x + c.0) as u32,
            (y + c.1) as u32,
            image::Rgba([255, 0, 0, 255]),
        );

        if quad == 3 {
            if y == 0 {
                x = r;
                y = 0;
                d = 1 - r;
                quad = 0;
            }
            if x < y {
                // octect 7
                x += 1;
                if d <= 0 {
                    d += 2 * x + 1;
                } else {
                    y -= 1;
                    d += 2 * (x - y) + 1;
                }
            } else {
                // octect 8
                y -= 1;
                if d <= 0 {
                    d += 2 * y + 1;
                } else {
                    x += 1;
                    d += 2 * (y - x) + 1;
                }
            }
        } else if quad == 0 {
            // octect 0
            if x == 0 {
                x = 0;
                y = -r;
                d = 1 - r;
                quad = 1;
            }
            if y > -x {
                y -= 1;
                if d <= 0 {
                    d += 2 * -y - 1;
                } else {
                    x -= 1;
                    d += 2 * (-y - x) + 1;
                }
            } else {
                x -= 1;
                if d <= 0 {
                    d += 2 * x - 1;
                } else {
                    y -= 1;
                    d += 2 * (x - -y) + 1;
                }
            }
        } else if quad == 1 {
            if y == 0 {
                x = -r;
                y = 0;
                d = 1 - r;
                quad = 2;
            }
            if x > y {
                x -= 1;
                if d <= 0 {
                    d += 2 * -x - 1;
                } else {
                    y += 1;
                    d += 2 * (-x - -y) - 1;
                }
            } else {
                y += 1;
                if d <= 0 {
                    d += 2 * -y - 1;
                    // d += 2 * -y + 1;
                } else {
                    x -= 1;
                    // d += 2 * y - (2 * x) + 1;
                    d += 2 * (-y - -x) - 1;
                }
            }
        } else if quad == 2 {
            if x == 0 {
                break;
            }
            if -x > y {
                y += 1;
                if d <= 0 {
                    d += 2 * y + 1;
                } else {
                    x += 1;
                    d += 2 * (y - -x) + 1;
                }
            } else {
                x += 1;
                if d <= 0 {
                    d += 2 * -x + 1;
                } else {
                    y += 1;
                    d += 2 * (-x - y) + 1;
                }
            }
        } else {
            println!("invalid quadrant");
            break;
        }
    }

    image.save("full_quad_arc.png")
}
