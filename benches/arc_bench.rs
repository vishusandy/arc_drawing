#![cfg(test)]

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

#[test]
fn test_arc_midpoint() -> Result<(), image::ImageError> {
    setup(arc_test::RADIUS).save("arc_midpoint.png")
}

fn bench_imageproc_circle(c: &mut Criterion) {
    c.bench_function("imageproc_circle", |b| {
        b.iter_batched(
            || arc_test::blank(),
            |mut image| {
                imageproc::drawing::draw_hollow_circle_mut(
                    &mut image,
                    arc_test::CENTER,
                    arc_test::RADIUS,
                    image::Rgba([255, 0, 0, 255]),
                )
            },
            BatchSize::SmallInput,
        )
    });
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

fn bench_bres_iter_o1(c: &mut Criterion) {
    c.bench_function("bres_iter_o1", |b| {
        b.iter_batched(
            || arc_test::setup(arc_test::RADIUS),
            |mut image| {
                arc_test::draw_iter(
                    &mut image,
                    arc_test::Oct1::full(arc_test::RADIUS, arc_test::CENTER),
                    image::Rgba([255, 0, 0, 255]),
                )
            },
            BatchSize::SmallInput,
        )
    });
}

#[allow(dead_code)]
fn bench_bres_all_octants(c: &mut Criterion) {
    c.bench_function("bres_iter_circle", |b| {
        b.iter_batched(
            || arc_test::setup(arc_test::RADIUS),
            |mut image| {
                arc_test::draw_bres_circle(
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

fn bench_partial_arc(c: &mut Criterion) {
    const RADS: f64 = std::f64::consts::PI / 4.0;
    const START: f64 = RADS * 0.1;
    const END: f64 = RADS * 0.75;
    c.bench_function("partial_arc", |b| {
        b.iter_batched(
            || arc_test::setup(arc_test::RADIUS),
            |mut image| {
                let mut arc =
                    arc_test::Arc::new(START, END, arc_test::RADIUS, arc_test::CENTER.into());
                arc.draw(&mut image, image::Rgba([255, 0, 0, 255]));
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_partial_annulus(c: &mut Criterion) {
    const RADS: f64 = std::f64::consts::PI / 4.0;
    const START: f64 = RADS * 0.1;
    const END: f64 = RADS * 0.75;
    c.bench_function("partial_annulus", |b| {
        b.iter_batched(
            || arc_test::setup(arc_test::RADIUS),
            |mut image| {
                let mut arc = arc_test::Annulus::new(
                    START,
                    END,
                    arc_test::RADIUS - 10,
                    arc_test::RADIUS,
                    arc_test::CENTER.into(),
                );
                arc.draw(&mut image, image::Rgba([255, 0, 0, 255]));
            },
            BatchSize::SmallInput,
        )
    });
}

// Old
criterion_group!(
    bres_iterators,
    bench_arc_midpoint,
    bench_arc_integer,
    bench_arc_integer2_single,
    bench_bres_iter_o1,
    bench_bres_all_octants,
    bench_arc_integer2_full
);

// For comparison
criterion_group!(imageproc, bench_imageproc_circle); // circle drawing from imageproc crate

// Current but don't benchmark by default
criterion_group!(arc_circle_segment, bench_partial_arc);

// These should be benchmarked by default
criterion_group!(bench_warmup, bench_arc_integer); // somehow improves performance
criterion_group!(annulus, bench_partial_annulus);

criterion_main!(bench_warmup, annulus);
