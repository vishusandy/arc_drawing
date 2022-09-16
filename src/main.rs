use arc_test::{CENTER_F, RADIUS, RADIUS_F};
const RADS: f64 = std::f64::consts::PI / 4.0; // range of a single octant

fn main() -> Result<(), image::ImageError> {
    let mut image = arc_test::setup(RADIUS);

    // DRAW PARTIAL ARC
    const START: f64 = RADS * 0.1;
    const END: f64 = RADS * 7.75;
    // let mut arc = arc_test::Arc::new(START, END, RADIUS, CENTER.into());
    // arc.draw(&mut image, image::Rgba([255, 0, 0, 255]));

    // DRAW PARTIAL ANNULUS
    // const RADS: f64 = std::f64::consts::PI / 4.0;
    // let ri = crate::RADIUS - 10;
    // let ro = crate::RADIUS;
    // let start = RADS * 6.00;
    // let end = RADS * 7.0 - std::f64::EPSILON;
    // let mut an = arc_test::Annulus::new(start, end, ri, ro, crate::CENTER.into());
    // an.draw(&mut image, image::Rgba([255, 0, 0, 255]));

    // DRAW ANTIALIASED CIRCLE
    let aa_arc = arc_test::AAArc::new(START, END, RADIUS_F, CENTER_F);
    aa_arc.draw(&mut image, image::Rgba([255, 0, 0, 255]));

    if image.height() > 0 {
        Ok(())
    } else {
        Err(image::ImageError::Unsupported(
            image::error::UnsupportedError::from_format_and_kind(
                image::error::ImageFormatHint::Name("Invalid dimensions".into()),
                image::error::UnsupportedErrorKind::GenericFeature("Invalid image size".into()),
            ),
        ))
    }
}
