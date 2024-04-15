use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num::Integer;
use rand::distributions::Standard;
use rand::{Rng, SeedableRng};

use gcd::*;

fn prepare() -> (Box<[u64]>, Box<[u64]>) {
    const SAMPLES: usize = 128;

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0xbeef_cafe);
    let xs = (&mut rng)
        .sample_iter(Standard)
        .take(SAMPLES)
        .collect::<Box<_>>();
    let ys = (&mut rng)
        .sample_iter(Standard)
        .take(SAMPLES)
        .collect::<Box<_>>();

    (xs, ys)
}

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

    group.finish();
}

criterion_group!(benches, bench_func);
criterion_main!(benches);
