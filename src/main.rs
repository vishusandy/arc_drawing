const CENTER_F: freehand::Pt<f64> = freehand::Pt::new(300.0, 300.0);
const IMG_SIZE: u32 = 600;

mod consts;
// use test::consts;

fn main() -> Result<(), image::ImageError> {
    let mut image = image::RgbaImage::new(IMG_SIZE, IMG_SIZE);

    // DRAW MULTIPLE ANTIALIASED CIRCLES
    (0..consts::STARTS.len())
        .map(|i| {
            freehand::AAArc::new(
                consts::STARTS[i],
                consts::ENDS[i],
                consts::RADII[i],
                CENTER_F,
            )
        })
        .for_each(|arc| arc.draw(&mut image, image::Rgba([255, 0, 0, 255])));

    // DRAW ANTIALIASED CIRCLE
    // const START: f64 = RADS * 0.1;
    // const END: f64 = RADS * 7.75;
    // let aa_arc = arc_test::AAArc::new(START, END, RADIUS_F, CENTER_F);
    // aa_arc.draw(&mut image, image::Rgba([255, 0, 0, 255]));

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
