use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use common::prepare;
use sum_float::*;

mod common;

const ITEMS: usize = 1000;

fn bench_func(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum-float");
    let data = prepare(ITEMS * 4);
    let mut result = vec![0.0; ITEMS];

    group.bench_function("sum_scalar", |b| b.iter(|| sum_scalar(&data, &mut result)));
    group.bench_function("sum_fast", |b| {
        b.iter(|| unsafe { sum_fast(&data, &mut result) })
    });
    group.bench_function("sum_fast_no_bound_checks", |b| {
        b.iter(|| sum_fast_no_bound_checks(&data, &mut result))
    });

    black_box(result);

    group.finish();
}

criterion_group!(benches, bench_func);
criterion_main!(benches);
