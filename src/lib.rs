mod arc;
pub use arc::arc_midpoint;
use criterion::black_box;

pub const IMG_SIZE: u32 = 600;
pub const RADIUS: i32 = 50;
pub const CENTER: (i32, i32) = (300, 300);

const OR: f64 = std::f64::consts::PI / 4.0;

pub fn setup(r: i32) -> image::RgbaImage {
    let mut image = guidelines();
    let center = CENTER;
    imageproc::drawing::draw_hollow_circle_mut(
        &mut image,
        center,
        r,
        image::Rgba([0, 0, 255, 255]),
    );
    draw_markers(&mut image, r, center);
    image
}

fn guidelines() -> image::RgbaImage {
    let mut image = black_box(image::RgbaImage::from_pixel(
        IMG_SIZE,
        IMG_SIZE,
        image::Rgba([255, 255, 255, 255]),
    ));
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

fn draw_markers(image: &mut image::RgbaImage, r: i32, c: (i32, i32)) {
    let rads = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    for o in 0..8 {
        let oa = o as f64 * OR;
        for a in rads {
            let rad = a * OR + oa;
            plot_marker(image, r, rad, c, image::Rgba([0, 255, 0, 255]));
        }
    }
}

fn plot_marker(
    image: &mut image::RgbaImage,
    r: i32,
    angle: f64,
    c: (i32, i32),
    color: image::Rgba<u8>,
) {
    let r = r as f64;
    let x = ((r * angle.cos()).round() as i32 + c.0) as u32;
    let y = ((r * angle.sin()).round() as i32 + c.1) as u32;
    image.put_pixel(x, y, color);
}
