use arc_test::{setup, Arc, CENTER, RADIUS};

const RADS: f64 = std::f64::consts::PI / 4.0;
const START: f64 = RADS * 0.1;
const END: f64 = RADS * 7.75;

// fn main() -> Result<(), image::ImageError> {
fn main() {
    use criterion::black_box;
    let mut image = setup(RADIUS);
    println!("creating arc");
    let mut arc = Arc::new(START, END, RADIUS, CENTER.into());
    println!("drawing");
    arc.draw(&mut image, image::Rgba([255, 0, 0, 255]));
    let _b = black_box(image);
    // image.save("images/main.png")
}
