use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use substring::*;

const TEXT: &str = "Hello✨, 🎈 this 🎉 is a text.";

fn bench_short_text(c: &mut Criterion) {
    let mut group = c.benchmark_group("short_text");

    const START: usize = 1;
    const END: usize = 20;
    const EXPECTED: &str = "ello✨, 🎈 this 🎉 is ";

    group.bench_function("substring_v1", |b| {
        b.iter(|| assert_eq!(substring_v1(black_box(TEXT), START, END), EXPECTED))
    });
    group.bench_function("substring_v4", |b| {
        b.iter(|| assert_eq!(substring_v4(black_box(TEXT), START, END), EXPECTED))
    });
    group.bench_function("substring_v5", |b| {
        b.iter(|| assert_eq!(substring_v5(black_box(TEXT), START, END), EXPECTED))
    });

    group.finish();
}

fn bench_large_text(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_text");

    const REPEAT: usize = 1000;
    const START: usize = 1;
    const END: usize = 20000;

    let input = TEXT.repeat(REPEAT);

    group.bench_function("substring_v1", |b| {
        b.iter(|| black_box(substring_v1(black_box(&input), START, END)))
    });
    group.bench_function("substring_v4", |b| {
        b.iter(|| black_box(substring_v4(black_box(&input), START, END)))
    });
    group.bench_function("substring_v5", |b| {
        b.iter(|| black_box(substring_v5(black_box(&input), START, END)))
    });

    group.finish();
}

criterion_group!(benches, bench_short_text, bench_large_text);
criterion_main!(benches);
