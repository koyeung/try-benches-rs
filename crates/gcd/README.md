# gcd

On Macbook Pro M1

```
% cargo bench -p gcd --bench benchmark_divan
    Finished bench [optimized] target(s) in 0.03s
     Running benches/benchmark_divan.rs (target/release/deps/benchmark_divan-ce67760f36894a9b)
Timer precision: 41 ns
benchmark_divan                     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ bench_0_naive_gcd                2.232 ms      │ 2.325 ms      │ 2.237 ms      │ 2.246 ms      │ 100     │ 100
│                                   7.339 Mitem/s │ 7.046 Mitem/s │ 7.322 Mitem/s │ 7.293 Mitem/s │         │
├─ bench_binary_gcd_if_0_return     1.229 ms      │ 1.314 ms      │ 1.232 ms      │ 1.237 ms      │ 100     │ 100
│                                   13.32 Mitem/s │ 12.46 Mitem/s │ 13.28 Mitem/s │ 13.23 Mitem/s │         │
├─ bench_binary_gcd_if_0_return_v2  1.228 ms      │ 1.304 ms      │ 1.231 ms      │ 1.235 ms      │ 100     │ 100
│                                   13.33 Mitem/s │ 12.55 Mitem/s │ 13.3 Mitem/s  │ 13.26 Mitem/s │         │
├─ bench_binary_gcd_minmax          1.227 ms      │ 1.292 ms      │ 1.231 ms      │ 1.236 ms      │ 100     │ 100
│                                   13.34 Mitem/s │ 12.67 Mitem/s │ 13.3 Mitem/s  │ 13.25 Mitem/s │         │
├─ bench_binary_gcd_minmax_v2       1.228 ms      │ 1.286 ms      │ 1.231 ms      │ 1.235 ms      │ 100     │ 100
│                                   13.34 Mitem/s │ 12.73 Mitem/s │ 13.3 Mitem/s  │ 13.25 Mitem/s │         │
├─ bench_binary_gcd_noswap          1.24 ms       │ 1.311 ms      │ 1.244 ms      │ 1.247 ms      │ 100     │ 100
│                                   13.2 Mitem/s  │ 12.49 Mitem/s │ 13.16 Mitem/s │ 13.12 Mitem/s │         │
├─ bench_binary_gcd_noswap_v2       1.227 ms      │ 1.277 ms      │ 1.231 ms      │ 1.234 ms      │ 100     │ 100
│                                   13.34 Mitem/s │ 12.82 Mitem/s │ 13.3 Mitem/s  │ 13.27 Mitem/s │         │
├─ bench_binary_gcd_recursive       1.337 ms      │ 1.454 ms      │ 1.34 ms       │ 1.348 ms      │ 100     │ 100
│                                   12.24 Mitem/s │ 11.26 Mitem/s │ 12.21 Mitem/s │ 12.14 Mitem/s │         │
├─ bench_binary_gcd_recursive_v2    1.358 ms      │ 1.528 ms      │ 1.361 ms      │ 1.371 ms      │ 100     │ 100
│                                   12.05 Mitem/s │ 10.71 Mitem/s │ 12.02 Mitem/s │ 11.94 Mitem/s │         │
├─ bench_binary_gcd_swap            1.479 ms      │ 1.543 ms      │ 1.483 ms      │ 1.487 ms      │ 100     │ 100
│                                   11.07 Mitem/s │ 10.61 Mitem/s │ 11.04 Mitem/s │ 11.01 Mitem/s │         │
├─ bench_binary_gcd_swap_v2         1.453 ms      │ 1.493 ms      │ 1.456 ms      │ 1.46 ms       │ 100     │ 100
│                                   11.27 Mitem/s │ 10.96 Mitem/s │ 11.24 Mitem/s │ 11.21 Mitem/s │         │
╰─ bench_num_gcd                    1.416 ms      │ 1.48 ms       │ 1.419 ms      │ 1.424 ms      │ 100     │ 100
                                    11.56 Mitem/s │ 11.06 Mitem/s │ 11.54 Mitem/s │ 11.5 Mitem/s  │         │
```
