const CENTER_F: freehand::Pt<f64> = freehand::Pt::new(300.0, 300.0);
const IMG_SIZE: u32 = 600;

mod consts {
    include!("test/consts.rs");
}

fn main() -> Result<(), image::ImageError> {
    let mut image = image::RgbaImage::new(IMG_SIZE, IMG_SIZE);

    // DRAW MULTIPLE ANTIALIASED CIRCLES
    (0..consts::STARTS.len())
        .map(|i| {
            freehand::conics::AntialiasedArc::new(
                consts::STARTS[i],
                consts::ENDS[i],
                consts::RADII[i],
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
