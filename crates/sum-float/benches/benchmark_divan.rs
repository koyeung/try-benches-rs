use common::prepare;
use sum_float::*;

mod common;

const ITEMS: usize = 1000;

#[divan::bench]
fn bench_sum_scalar(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| (prepare(ITEMS * 4), vec![0.0; ITEMS]))
        .bench_values(|(d, mut r)| {
            sum_scalar(&d, &mut r);
        });
}

#[divan::bench]
fn bench_sum_fast(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| (prepare(ITEMS * 4), vec![0.0; ITEMS]))
        .bench_values(|(d, mut r)| unsafe { sum_fast(&d, &mut r) });
}

#[divan::bench]
fn bench_sum_fast_no_bound_checks(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| (prepare(ITEMS * 4), vec![0.0; ITEMS]))
        .bench_values(|(d, mut r)| sum_fast_no_bound_checks(&d, &mut r));
}

fn main() {
    divan::main();
}
