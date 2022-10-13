use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
mod consts {
    include!("../src/test/consts.rs");
}
mod misc;

const IMG_SIZE: u32 = 600;
const RADIUS: i32 = 240;
const CENTER: (i32, i32) = (300, 300);
const RADIUS_F: f64 = RADIUS as f64;
const CENTER_F: freehand::Pt<f64> = freehand::Pt::new(CENTER.0 as f64, CENTER.1 as f64);

pub fn blank() -> image::RgbaImage {
    image::RgbaImage::from_pixel(IMG_SIZE, IMG_SIZE, image::Rgba([255, 255, 255, 255]))
}

fn bench_imageproc_circle(c: &mut Criterion) {
    c.bench_function("stock_circle", |b| {
        b.iter_batched(
            blank,
            |mut image| {
                imageproc::drawing::draw_hollow_circle_mut(
                    &mut image,
                    CENTER,
                    RADIUS,
                    image::Rgba([255, 0, 0, 255]),
                )
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_warmup(c: &mut Criterion) {
    c.bench_function("warmup", |b| {
        b.iter_batched(
            blank,
            |image| misc::warmup_arc(image, RADIUS, CENTER),
            BatchSize::SmallInput,
        )
    });
}

fn bench_partial_annulus(c: &mut Criterion) {
    const RADS: f64 = std::f64::consts::PI / 4.0;
    const START: f64 = RADS * 0.2;
    const END: f64 = RADS * 7.75;
    c.bench_function("partial_annulus", |b| {
        b.iter_batched(
            blank,
            |mut image| {
                let arc = freehand::conics::Annulus::new(START, END, RADIUS - 10, RADIUS, CENTER);
                arc.draw(&mut image, image::Rgba([255, 0, 0, 255]));
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_aa_partial_arc(c: &mut Criterion) {
    const RADS: f64 = std::f64::consts::PI / 4.0;
    const START: f64 = RADS * 0.0;
    const END: f64 = RADS * 8.0;
    c.bench_function("aa_arc", |b| {
        b.iter_batched(
            || {
                (
                    blank(),
                    freehand::conics::AntialiasedArc::new(START, END, RADIUS_F as f64, CENTER_F),
                )
            },
            |(mut image, arc)| {
                arc.draw(&mut image, image::Rgba([255, 0, 0, 255]));
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_aa_multiple_arcs(c: &mut Criterion) {
    use consts::*;
    const SIZE: u32 = 600;
    const C: freehand::Pt<f64> = freehand::Pt::new(300.0, 300.0);
    let base = image::RgbaImage::from_pixel(SIZE, SIZE, image::Rgba([255, 255, 255, 255]));
    let arcs: Vec<freehand::conics::AntialiasedArc> = (0..50)
        .map(|i| freehand::conics::AntialiasedArc::new(STARTS[i], ENDS[i], RADII[i], C))
        .collect();

    c.bench_function("50_aa_arcs", |b| {
        b.iter_batched(
            || (base.clone(), arcs.clone()),
            |(mut image, arcs)| {
                for arc in arcs {
                    arc.draw(&mut image, image::Rgba([255, 0, 0, 255]));
                }
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_arc(c: &mut Criterion) {
    const RADS: f64 = std::f64::consts::PI / 4.0;
    const START: f64 = RADS * 0.2;
    const END: f64 = RADS * 7.75;
    c.bench_function("arc", |b| {
        b.iter_batched(
            blank,
            |mut image| {
                freehand::conics::Arc::new(START, END, RADIUS, CENTER)
                    .draw(&mut image, image::Rgba([255, 0, 0, 255]));
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(stock, bench_imageproc_circle); // For comparison - benchmarks default image library crate
criterion_group!(warmup, bench_warmup); // somehow improves performance ???? 🤦

criterion_group! {
    name = antialias;
    config = Criterion::default().sample_size(500);
    targets = bench_aa_partial_arc, bench_aa_multiple_arcs
}

criterion_group!(annulus, bench_partial_annulus);
criterion_group!(arcs, bench_arc);

criterion_main!(warmup, stock, arcs, annulus, antialias);
// criterion_main!(warmup, antialias);
