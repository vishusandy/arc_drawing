//! Code for testing/benchmarking purposes

#[cfg(test)]
mod arc2;
/// Image functions for testing
#[cfg(test)]
pub(crate) mod img;

#[macro_export]
macro_rules! test_pixels_changed {
    ( $test_name:ident, $f:ident( $($a:expr),+ ), $size:literal, $m:expr ) => {
        #[test]
        fn $test_name() {
            $crate::logger($crate::LOG_LEVEL);
            let mut image = $crate::test::img::blank(($size, $size));
            let img_name = format!("images/tests/failed_{}.png", stringify! {$test_name});
            let white = image::Rgba([255, 255, 255, 255]);
            let color = image::Rgba([255, 0, 0, 255]);
            super::$f(&mut image, $($a),+, color);
            let mut image_test = image.clone();
            let m: &[(u32, u32)] = $m;

            for (x, y) in m {
                let p = image_test.get_pixel_mut(*x, *y);
                if p != &white {
                    #[cfg(test)]
                    log::trace!("Found ({}, {})", x, y);
                    *p = white;
                } else {
                    log::trace!("Missed ({}, {})", x, y);
                    image.save(&img_name).unwrap();
                    eprintln!(
                        "\nTEST FAILED\n  Test: {}\n  Expected pixel not found ({},{})\n  saving: '{}'\n",
                        stringify! {$test_name},
                        x,
                        y,
                        &img_name
                    );
                    panic!(
                        "Expected pixel not found at ({}, {})",
                        x,
                        y
                    );
                }
            }

            for (x, y, p) in image_test.enumerate_pixels() {
                if p != &white {
                    image.save(&img_name).unwrap();
                    eprintln!(
                        "\nTEST FAILED\n  Test: {}\n  Unpexpected pixel found at ({},{})\n  saving: '{}'\n",
                        stringify! {$test_name},
                        x,
                        y,
                        &img_name
                    );
                    panic!(
                        "Unexpected pixel found at ({}, {})",
                        x,
                        y
                    );
                }
            }

            let _ = std::fs::remove_file(img_name);
        }
    };
}

#[macro_export]
macro_rules! test_pixel_colors {
    ( $test_name:ident, $f:ident( $($a:expr),+ ), $size:literal, $color:expr, $pixels:expr, $colors:expr ) => {
        #[test]
        fn $test_name() {
            $crate::logger($crate::LOG_LEVEL);
            let mut image = $crate::test::img::blank(($size, $size));
            let img_name = format!("images/tests/failed_{}.png", stringify! {$test_name});
            let white = image::Rgba([255, 255, 255, 255]);
            let color = $color;
            super::$f(&mut image, $($a),+, color);
            let mut image_test = image.clone();
            let m: &[(u32, u32)] = $pixels;
            let colors: &[_] = $colors;

            for (i, (x, y)) in m.iter().enumerate() {
                let p = image_test.get_pixel_mut(*x, *y);
                let col = colors[i];
                if p == &col {
                    *p = white;
                } else {
                    image.save(&img_name).unwrap();
                    eprintln!(
                        "\nTEST FAILED\n  Test: {}\n  Expected color {:?} at ({},{})\n  saving: '{}'\n",
                        stringify! {$test_name},
                        col.0,
                        x,
                        y,
                        &img_name
                    );
                    panic!(
                        "Expected color {:?} but found {:?} at ({}, {})",
                        col.0,
                        p.0,
                        x,
                        y
                    );
                }
            }

            for (x, y, p) in image_test.enumerate_pixels() {
                if p != &white {
                    image.save(&img_name).unwrap();
                    eprintln!(
                        "\nTEST FAILED\n  Test: {}\n  Unpexpected pixel {:?} found at ({},{}) with color: {:?}\n  saving: '{}'\n",
                        stringify! {$test_name},
                        p.0,
                        x,
                        y,
                        p.0,
                        &img_name
                    );
                    panic!(
                        "Unexpected pixel found at ({}, {})",
                        x,
                        y
                    );
                }
            }

            let _ = std::fs::remove_file(img_name);
        }
    };
}
