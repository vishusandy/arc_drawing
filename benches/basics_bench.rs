use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use freehand::{horizontal_line, rectangle_filled, vertical_line};

const IMG_SIZE: u32 = 600;

pub fn blank() -> image::RgbaImage {
    image::RgbaImage::from_pixel(IMG_SIZE, IMG_SIZE, image::Rgba([255, 255, 255, 255]))
}

fn bench_basic_vertical_line(c: &mut Criterion) {
    c.bench_function("vertical_line", |b| {
        b.iter_batched(
            || blank(),
            |mut image| {
                vertical_line(
                    &mut image,
                    IMG_SIZE / 2,
                    0,
                    IMG_SIZE,
                    image::Rgba([255, 0, 0, 255]),
                );
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_basic_horizontal_line(c: &mut Criterion) {
    c.bench_function("vertical_line", |b| {
        b.iter_batched(
            || blank(),
            |mut image| {
                horizontal_line(
                    &mut image,
                    IMG_SIZE / 2,
                    0,
                    IMG_SIZE,
                    image::Rgba([255, 0, 0, 255]),
                );
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_basic_rectangle_filled(c: &mut Criterion) {
    c.bench_function("vertical_line", |b| {
        b.iter_batched(
            || blank(),
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

criterion_group!(
    basic,
    bench_basic_vertical_line,
    bench_basic_horizontal_line,
    bench_basic_rectangle_filled
);

criterion_main!(basic);
