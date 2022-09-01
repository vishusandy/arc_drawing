#![cfg(test)]

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

#[test]
fn test_arc_midpoint() -> Result<(), image::ImageError> {
    setup(arc_test::RADIUS).save("arc_midpoint.png")
}

fn bench_arc_midpoint(c: &mut Criterion) {
    c.bench_function("arc_midpoint_fp", |b| {
        b.iter_batched(
            || arc_test::setup(arc_test::RADIUS),
            |image| arc_test::arc_midpoint(image, arc_test::RADIUS, arc_test::CENTER),
            BatchSize::SmallInput,
        )
    });
}

fn bench_arc_integer(c: &mut Criterion) {
    c.bench_function("arc_integer", |b| {
        b.iter_batched(
            || arc_test::setup(arc_test::RADIUS),
            |image| arc_test::arc_integer(image, arc_test::RADIUS, arc_test::CENTER),
            BatchSize::SmallInput,
        )
    });
}

fn bench_arc_integer2_full(c: &mut Criterion) {
    c.bench_function("arc_integer2_full", |b| {
        b.iter_batched(
            || arc_test::setup(arc_test::RADIUS),
            |mut image| {
                arc_test::full_circle(
                    &mut image,
                    arc_test::RADIUS,
                    arc_test::CENTER,
                    image::Rgba([255, 0, 0, 255]),
                )
            },
            BatchSize::SmallInput,
        )
    });
}
fn bench_arc_integer2_single(c: &mut Criterion) {
    c.bench_function("arc_integer2_single", |b| {
        b.iter_batched(
            || arc_test::setup(arc_test::RADIUS),
            |mut image| {
                arc_test::full_arc_oct(
                    &mut image,
                    arc_test::RADIUS,
                    arc_test::CENTER,
                    0,
                    image::Rgba([255, 0, 0, 255]),
                )
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    fp_benches,
    bench_arc_midpoint,
    bench_arc_integer,
    bench_arc_integer2_full,
    bench_arc_integer2_single,
);
criterion_main!(fp_benches);
