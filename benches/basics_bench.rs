use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

const IMG_SIZE: u32 = 600;

pub fn blank() -> image::RgbaImage {
    image::RgbaImage::from_pixel(IMG_SIZE, IMG_SIZE, image::Rgba([255, 255, 255, 255]))
}

fn bench_basic_vertical_line(c: &mut Criterion) {
    c.bench_function("vertical_line", |b| {
        b.iter_batched(
            blank,
            |mut image| {
                freehand::lines::vertical_line(
                    &mut image,
                    (IMG_SIZE / 2, 0),
                    IMG_SIZE,
                    image::Rgba([255, 0, 0, 255]),
                );
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_basic_horizontal_line(c: &mut Criterion) {
    c.bench_function("horizontal_line", |b| {
        b.iter_batched(
            blank,
            |mut image| {
                freehand::lines::horizontal_line(
                    &mut image,
                    (0, IMG_SIZE / 2),
                    IMG_SIZE,
                    image::Rgba([255, 0, 0, 255]),
                );
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_basic_rectangle_filled(c: &mut Criterion) {
    c.bench_function("rectangle_filled", |b| {
        b.iter_batched(
            blank,
            |mut image| {
                freehand::shapes::rectangle_filled(
                    &mut image,
                    freehand::Pt::new(50, 50),
                    100,
                    100,
                    image::Rgba([255, 0, 0, 255]),
                );
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_basic_horizontal_dashed(c: &mut Criterion) {
    c.bench_function("horizontal_dashed", |b| {
        b.iter_batched(
            blank,
            |mut image| {
                freehand::lines::horizontal_dashed_line(
                    &mut image,
                    (0, IMG_SIZE / 2),
                    10,
                    IMG_SIZE,
                    image::Rgba([255, 0, 0, 255]),
                );
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_basic_vertical_dashed(c: &mut Criterion) {
    c.bench_function("vertical_dashed", |b| {
        b.iter_batched(
            blank,
            |mut image| {
                freehand::lines::vertical_dashed_line(
                    &mut image,
                    (IMG_SIZE / 2, 0),
                    10,
                    IMG_SIZE,
                    image::Rgba([255, 0, 0, 255]),
                );
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_blend_safe(c: &mut Criterion) {
    c.bench_function("safe_blend", |b| {
        b.iter(|| {
            let mut image = blank();
            let color = image::Rgba([255, 0, 0, 255]);
            let o = 0.5;
            for (x, y) in (0..IMG_SIZE).zip(0..IMG_SIZE) {
                // let o = x as f32 / IMG_SIZE as f32;
                freehand::ops::blend_at(&mut image, x, y, o, color);
            }
        })
    });
}

fn bench_blend_unsafe(c: &mut Criterion) {
    c.bench_function("unsafe_blend", |b| {
        b.iter(|| {
            let mut image = blank();
            let color = image::Rgba([255, 0, 0, 255]);
            let o = 0.5;
            for (x, y) in (0..IMG_SIZE).zip(0..IMG_SIZE) {
                // let o = x as f32 / IMG_SIZE as f32;
                unsafe {
                    freehand::ops::blend_at_unchecked(&mut image, x, y, o, color);
                }
            }
        })
    });
}

fn bench_imageproc_blend(c: &mut Criterion) {
    use image::Pixel;
    c.bench_function("imageproc_blend", |b| {
        b.iter(|| {
            let mut image = blank();
            let color = image::Rgba([255, 0, 0, 127]);
            for (x, y) in (0..IMG_SIZE).zip(0..IMG_SIZE) {
                let o = x as f32 / IMG_SIZE as f32;
                image.get_pixel_mut(x, y).blend(&color);
            }
        })
    });
}

criterion_group!(
    lines,
    bench_basic_vertical_line,
    bench_basic_horizontal_line,
    bench_basic_rectangle_filled,
    bench_basic_horizontal_dashed,
    bench_basic_vertical_dashed
);

criterion_group!(
    ops,
    bench_blend_safe,
    bench_blend_unsafe,
    bench_imageproc_blend
);

criterion_main!(ops);
