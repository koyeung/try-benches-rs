use num::Integer;
use rand::distributions::Standard;
use rand::{Rng, SeedableRng};

use gcd::*;

fn main() {
    divan::main();
}

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

#[divan::bench]
fn bench_num_gcd(bencher: divan::Bencher) {
    let (x, y) = prepare();

    bencher
        .with_inputs(|| (&x, &y))
        .input_counter(|(x, y)| x.len() * y.len())
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add((*i).gcd(j));
                }
            }

            counter
        });
}

#[divan::bench]
fn bench_0_naive_gcd(bencher: divan::Bencher) {
    let (x, y) = prepare();

    bencher
        .with_inputs(|| (&x, &y))
        .input_counter(|(x, y)| x.len() * y.len())
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(naive_gcd(*i, *j));
                }
            }

            counter
        });
}

#[divan::bench]
fn bench_binary_gcd_swap(bencher: divan::Bencher) {
    let (x, y) = prepare();

    bencher
        .with_inputs(|| (&x, &y))
        .input_counter(|(x, y)| x.len() * y.len())
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_swap(*i, *j));
                }
            }

            counter
        });
}

#[divan::bench]
fn bench_binary_gcd_swap_v2(bencher: divan::Bencher) {
    let (x, y) = prepare();

    bencher
        .with_inputs(|| (&x, &y))
        .input_counter(|(x, y)| x.len() * y.len())
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_swap_v2(*i, *j));
                }
            }

            counter
        });
}

#[divan::bench]
fn bench_binary_gcd_noswap(bencher: divan::Bencher) {
    let (x, y) = prepare();

    bencher
        .with_inputs(|| (&x, &y))
        .input_counter(|(x, y)| x.len() * y.len())
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_noswap(*i, *j));
                }
            }

            counter
        });
}

#[divan::bench]
fn bench_binary_gcd_minmax(bencher: divan::Bencher) {
    let (x, y) = prepare();

    bencher
        .with_inputs(|| (&x, &y))
        .input_counter(|(x, y)| x.len() * y.len())
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_minmax(*i, *j));
                }
            }

            counter
        });
}
