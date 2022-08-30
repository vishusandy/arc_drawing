#![cfg(test)]

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

#[test]
fn test_arc_midpoint() -> Result<(), image::ImageError> {
    setup(arc_test::R).save("arc_midpoint.png")
}

fn bench_arc_midpoint(c: &mut Criterion) {
    c.bench_function("arc_midpoint_fp", |b| {
        b.iter_batched(
            || arc_test::setup(arc_test::R),
            |image| arc_test::arc_midpoint(image, arc_test::R, arc_test::CENTER),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, bench_arc_midpoint);
criterion_main!(benches);
