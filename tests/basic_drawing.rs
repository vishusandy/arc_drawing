fn logger(level: log::LevelFilter) {
    let _ = env_logger::Builder::new()
        .filter_level(level)
        .format_module_path(false)
        .format_target(false)
        .format_timestamp(None)
        .format_level(false)
        .try_init();
}

use freehand::Pt;
use image::Rgba;

#[test]
fn basic_drawing() -> Result<(), image::ImageError> {
    logger(log::LevelFilter::Warn);
    let height = 400;
    let width = 400;

    let mut image = image::RgbaImage::from_pixel(width, height, Rgba([255, 255, 255, 255]));

    freehand::lines::vertical_line(
        &mut image,
        (width / 2, 0),
        height - 1,
        Rgba([0, 255, 0, 255]),
    );
    freehand::lines::horizontal_line(
        &mut image,
        (0, height / 2),
        width - 1,
        Rgba([0, 255, 0, 255]),
    );

    freehand::lines::horizontal_dashed_line(
        &mut image,
        (0, 100),
        width * 2,
        2,
        Rgba([174, 252, 178, 255]),
    );
    freehand::lines::vertical_dashed_line(
        &mut image,
        (100, 0),
        width - 1,
        2,
        Rgba([174, 252, 178, 255]),
    );
    freehand::lines::horizontal_dashed_line_alpha(
        &mut image,
        (0, 300),
        800,
        2,
        0.4,
        Rgba([174, 252, 178, 255]),
    );
    freehand::lines::vertical_dashed_line_alpha(
        &mut image,
        (300, 0),
        400,
        2,
        0.4,
        Rgba([174, 252, 178, 255]),
    );

    freehand::shapes::rectangle(&mut image, Pt::new(0, 0), 100, 100, Rgba([255, 0, 0, 255]));

    freehand::shapes::rectangle_filled(
        &mut image,
        Pt::new(300, 300),
        150,
        150,
        Rgba([255, 0, 0, 255]),
    );

    freehand::shapes::rectangle_filled_alpha(
        &mut image,
        Pt::new(0, 300),
        100,
        100,
        0.2,
        Rgba([255, 0, 0, 255]),
    );

    freehand::lines::diagonal_line(&mut image, (200, 200), (400, 0), Rgba([255, 98, 0, 255]));
    freehand::lines::diagonal_line(&mut image, (200, 200), (0, 0), Rgba([255, 98, 0, 255]));
    freehand::lines::diagonal_line(&mut image, (200, 200), (400, 400), Rgba([255, 98, 0, 255]));
    freehand::lines::diagonal_line(&mut image, (200, 200), (0, 500), Rgba([255, 98, 0, 255]));

    freehand::lines::diagonal_dashed_line(
        &mut image,
        (0, 100),
        (300, 400),
        2,
        Rgba([255, 210, 181, 255]),
    );
    freehand::lines::diagonal_dashed_line(
        &mut image,
        (0, 100),
        (300, 400),
        100,
        Rgba([255, 98, 0, 255]),
    );
    freehand::lines::diagonal_dashed_line(
        &mut image,
        (400, 50),
        (50, 400),
        2,
        Rgba([255, 210, 181, 255]),
    );

    freehand::lines::diagonal_dashed_line_alpha(
        &mut image,
        (0, 50),
        (350, 400),
        2,
        0.3,
        Rgba([255, 192, 0, 255]),
    );

    image.save("images/basic_drawing.png")
}
