use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use freehand::*;

const IMG_SIZE: u32 = 600;

pub fn blank() -> image::RgbaImage {
    image::RgbaImage::from_pixel(IMG_SIZE, IMG_SIZE, image::Rgba([255, 255, 255, 255]))
}

fn bench_basic_vertical_line(c: &mut Criterion) {
    c.bench_function("vertical_line", |b| {
        b.iter_batched(
            blank,
            |mut image| {
                vertical_line(
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
                horizontal_line(
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
                rectangle_filled(
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
                horizontal_dashed_line(
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
                vertical_dashed_line(
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

criterion_group!(
    basic,
    bench_basic_vertical_line,
    bench_basic_horizontal_line,
    bench_basic_rectangle_filled,
    bench_basic_horizontal_dashed,
    bench_basic_vertical_dashed
);

criterion_main!(basic);
