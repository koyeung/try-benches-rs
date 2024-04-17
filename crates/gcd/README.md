# gcd

On Macbook Pro M1

```
% cargo bench -p gcd --bench benchmark_divan
    Finished bench [optimized] target(s) in 0.03s
     Running benches/benchmark_divan.rs (target/release/deps/benchmark_divan-ce67760f36894a9b)
Timer precision: 41 ns
benchmark_divan                     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ bench_0_naive_gcd                4.508 ms      │ 4.841 ms      │ 4.532 ms      │ 4.555 ms      │ 100     │ 100
│                                   7.267 Mitem/s │ 6.768 Mitem/s │ 7.229 Mitem/s │ 7.192 Mitem/s │         │
├─ bench_binary_gcd_if_0_return     2.459 ms      │ 2.611 ms      │ 2.467 ms      │ 2.479 ms      │ 100     │ 100
│                                   13.32 Mitem/s │ 12.54 Mitem/s │ 13.27 Mitem/s │ 13.21 Mitem/s │         │
├─ bench_binary_gcd_if_0_return_v2  2.458 ms      │ 2.571 ms      │ 2.465 ms      │ 2.472 ms      │ 100     │ 100
│                                   13.32 Mitem/s │ 12.74 Mitem/s │ 13.29 Mitem/s │ 13.25 Mitem/s │         │
├─ bench_binary_gcd_minmax          2.456 ms      │ 2.989 ms      │ 2.469 ms      │ 2.511 ms      │ 100     │ 100
│                                   13.33 Mitem/s │ 10.96 Mitem/s │ 13.26 Mitem/s │ 13.04 Mitem/s │         │
├─ bench_binary_gcd_minmax_v2       2.459 ms      │ 2.532 ms      │ 2.468 ms      │ 2.472 ms      │ 100     │ 100
│                                   13.32 Mitem/s │ 12.93 Mitem/s │ 13.27 Mitem/s │ 13.25 Mitem/s │         │
├─ bench_binary_gcd_noswap          2.485 ms      │ 2.557 ms      │ 2.489 ms      │ 2.496 ms      │ 100     │ 100
│                                   13.18 Mitem/s │ 12.81 Mitem/s │ 13.16 Mitem/s │ 13.12 Mitem/s │         │
├─ bench_binary_gcd_noswap_v2       2.459 ms      │ 2.528 ms      │ 2.463 ms      │ 2.469 ms      │ 100     │ 100
│                                   13.32 Mitem/s │ 12.95 Mitem/s │ 13.3 Mitem/s  │ 13.26 Mitem/s │         │
├─ bench_binary_gcd_recursive       2.693 ms      │ 2.747 ms      │ 2.699 ms      │ 2.705 ms      │ 100     │ 100
│                                   12.16 Mitem/s │ 11.92 Mitem/s │ 12.13 Mitem/s │ 12.11 Mitem/s │         │
├─ bench_binary_gcd_recursive_v2    2.698 ms      │ 2.776 ms      │ 2.704 ms      │ 2.71 ms       │ 100     │ 100
│                                   12.14 Mitem/s │ 11.8 Mitem/s  │ 12.11 Mitem/s │ 12.08 Mitem/s │         │
├─ bench_binary_gcd_swap            2.949 ms      │ 3.007 ms      │ 2.956 ms      │ 2.962 ms      │ 100     │ 100
│                                   11.1 Mitem/s  │ 10.89 Mitem/s │ 11.08 Mitem/s │ 11.06 Mitem/s │         │
├─ bench_binary_gcd_swap_v2         2.942 ms      │ 3.003 ms      │ 2.95 ms       │ 2.955 ms      │ 100     │ 100
│                                   11.13 Mitem/s │ 10.91 Mitem/s │ 11.1 Mitem/s  │ 11.08 Mitem/s │         │
╰─ bench_num_gcd                    2.827 ms      │ 2.917 ms      │ 2.831 ms      │ 2.84 ms       │ 100     │ 100
                                    11.58 Mitem/s │ 11.23 Mitem/s │ 11.57 Mitem/s │ 11.53 Mitem/s │         │
```
