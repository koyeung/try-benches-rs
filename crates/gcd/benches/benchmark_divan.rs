use num::Integer;

use common::prepare;
use gcd::*;

mod common;

fn main() {
    divan::main();
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
fn bench_binary_gcd_noswap_v2(bencher: divan::Bencher) {
    let (x, y) = prepare();

    bencher
        .with_inputs(|| (&x, &y))
        .input_counter(|(x, y)| x.len() * y.len())
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_noswap_v2(*i, *j));
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
#[divan::bench]
fn bench_binary_gcd_minmax_v2(bencher: divan::Bencher) {
    let (x, y) = prepare();

    bencher
        .with_inputs(|| (&x, &y))
        .input_counter(|(x, y)| x.len() * y.len())
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_minmax_v2(*i, *j));
                }
            }

            counter
        });
}

#[divan::bench]
fn bench_binary_gcd_if_0_return(bencher: divan::Bencher) {
    let (x, y) = prepare();

    bencher
        .with_inputs(|| (&x, &y))
        .input_counter(|(x, y)| x.len() * y.len())
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_if_0_return(*i, *j));
                }
            }

            counter
        });
}
#[divan::bench]
fn bench_binary_gcd_if_0_return_v2(bencher: divan::Bencher) {
    let (x, y) = prepare();

    bencher
        .with_inputs(|| (&x, &y))
        .input_counter(|(x, y)| x.len() * y.len())
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_if_0_return_v2(*i, *j));
                }
            }

            counter
        });
}

#[divan::bench]
fn bench_binary_gcd_recursive(bencher: divan::Bencher) {
    let (x, y) = prepare();

    bencher
        .with_inputs(|| (&x, &y))
        .input_counter(|(x, y)| x.len() * y.len())
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_recursive(*i, *j));
                }
            }

            counter
        });
}
#[divan::bench]
fn bench_binary_gcd_recursive_v2(bencher: divan::Bencher) {
    let (x, y) = prepare();

    bencher
        .with_inputs(|| (&x, &y))
        .input_counter(|(x, y)| x.len() * y.len())
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_recursive_v2(*i, *j));
                }
            }

            counter
        });
}
