use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use num::Integer;

use common::prepare;
use gcd::*;

mod common;

fn bench_func(c: &mut Criterion) {
    let mut group = c.benchmark_group("gcd");
    let (x, y) = prepare();

    group.bench_function("num_gcd", |b| {
        b.iter(|| {
            let mut counter = 0_u64;
            for i in black_box(&x).iter() {
                for j in black_box(&y).iter() {
                    counter = counter.wrapping_add((*i).gcd(j));
                }
            }

            black_box(counter)
        })
    });

    group.bench_function("naive_gcd", |b| {
        b.iter(|| {
            let mut counter = 0_u64;
            for i in black_box(&x).iter() {
                for j in black_box(&y).iter() {
                    counter = counter.wrapping_add(naive_gcd(*i, *j));
                }
            }

            black_box(counter)
        })
    });

    group.bench_function("binary_gcd_swap", |b| {
        b.iter(|| {
            let mut counter = 0_u64;
            for i in black_box(&x).iter() {
                for j in black_box(&y).iter() {
                    counter = counter.wrapping_add(binary_gcd_swap(*i, *j));
                }
            }

            black_box(counter)
        })
    });

    group.bench_function("binary_gcd_swap_v2", |b| {
        b.iter(|| {
            let mut counter = 0_u64;
            for i in black_box(&x).iter() {
                for j in black_box(&y).iter() {
                    counter = counter.wrapping_add(binary_gcd_swap_v2(*i, *j));
                }
            }

            black_box(counter)
        })
    });

    group.bench_function("binary_gcd_noswap", |b| {
        b.iter(|| {
            let mut counter = 0_u64;
            for i in black_box(&x).iter() {
                for j in black_box(&y).iter() {
                    counter = counter.wrapping_add(binary_gcd_noswap(*i, *j));
                }
            }

            black_box(counter)
        })
    });

    group.bench_function("binary_gcd_minmax", |b| {
        b.iter(|| {
            let mut counter = 0_u64;
            for i in black_box(&x).iter() {
                for j in black_box(&y).iter() {
                    counter = counter.wrapping_add(binary_gcd_minmax(*i, *j));
                }
            }

            black_box(counter)
        })
    });

    group.bench_function("binary_gcd_if_0_return", |b| {
        b.iter(|| {
            let mut counter = 0_u64;
            for i in black_box(&x).iter() {
                for j in black_box(&y).iter() {
                    counter = counter.wrapping_add(binary_gcd_if_0_return(*i, *j));
                }
            }

            black_box(counter)
        })
    });

    group.bench_function("binary_gcd_recursive", |b| {
        b.iter(|| {
            let mut counter = 0_u64;
            for i in black_box(&x).iter() {
                for j in black_box(&y).iter() {
                    counter = counter.wrapping_add(binary_gcd_recursive(*i, *j));
                }
            }

            black_box(counter)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_func);
criterion_main!(benches);
