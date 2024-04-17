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
        .input_counter(|(x, y)| x.len() * y.len() * 2)
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add((*i).gcd(j));
                }
            }
            // avoid bias on inner/outer loop, perform tests again with i j reversed
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add((*j).gcd(i));
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
        .input_counter(|(x, y)| x.len() * y.len() * 2)
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(naive_gcd(*i, *j));
                }
            }
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(naive_gcd(*j, *i));
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
        .input_counter(|(x, y)| x.len() * y.len() * 2)
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_swap(*i, *j));
                }
            }
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_swap(*j, *i));
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
        .input_counter(|(x, y)| x.len() * y.len() * 2)
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_swap_v2(*i, *j));
                }
            }
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_swap_v2(*j, *i));
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
        .input_counter(|(x, y)| x.len() * y.len() * 2)
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_noswap(*i, *j));
                }
            }
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_noswap(*j, *i));
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
        .input_counter(|(x, y)| x.len() * y.len() * 2)
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_noswap_v2(*i, *j));
                }
            }
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_noswap_v2(*j, *i));
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
        .input_counter(|(x, y)| x.len() * y.len() * 2)
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_minmax(*i, *j));
                }
            }
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_minmax(*j, *i));
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
        .input_counter(|(x, y)| x.len() * y.len() * 2)
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_minmax_v2(*i, *j));
                }
            }
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_minmax_v2(*j, *i));
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
        .input_counter(|(x, y)| x.len() * y.len() * 2)
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_if_0_return(*i, *j));
                }
            }
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_if_0_return(*j, *i));
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
        .input_counter(|(x, y)| x.len() * y.len() * 2)
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_if_0_return_v2(*i, *j));
                }
            }
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_if_0_return_v2(*j, *i));
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
        .input_counter(|(x, y)| x.len() * y.len() * 2)
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_recursive(*i, *j));
                }
            }
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_recursive(*j, *i));
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
        .input_counter(|(x, y)| x.len() * y.len() * 2)
        .bench_values(|(x, y)| {
            let mut counter = 0_u64;
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_recursive_v2(*i, *j));
                }
            }
            for i in x.iter() {
                for j in y.iter() {
                    counter = counter.wrapping_add(binary_gcd_recursive_v2(*j, *i));
                }
            }

            counter
        });
}
