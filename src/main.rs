const CENTER_F: freehand::Pt<f64> = freehand::Pt::new(300.0, 300.0);
const IMG_SIZE: u32 = 600;

mod test_consts;

fn main() -> Result<(), image::ImageError> {
    let mut image = image::RgbaImage::new(IMG_SIZE, IMG_SIZE);

    // DRAW MULTIPLE ANTIALIASED CIRCLES
    (0..test_consts::STARTS.len())
        .map(|i| {
            freehand::AntialiasedArc::new(
                test_consts::STARTS[i],
                test_consts::ENDS[i],
                test_consts::RADII[i],
                CENTER_F,
            )
        })
        .for_each(|arc| arc.draw(&mut image, image::Rgba([255, 0, 0, 255])));

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
