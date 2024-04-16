# gcd

On Macbook Pro M1

```
    Finished bench [optimized] target(s) in 0.04s
     Running benches/benchmark_divan.rs (target/release/deps/benchmark_divan-ce67760f36894a9b)
Timer precision: 41 ns
benchmark_divan                fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ bench_0_naive_gcd           2.233 ms      │ 2.328 ms      │ 2.237 ms      │ 2.246 ms      │ 100     │ 100
│                              7.336 Mitem/s │ 7.037 Mitem/s │ 7.322 Mitem/s │ 7.291 Mitem/s │         │
├─ bench_binary_gcd_minmax     1.228 ms      │ 1.343 ms      │ 1.232 ms      │ 1.236 ms      │ 100     │ 100
│                              13.33 Mitem/s │ 12.19 Mitem/s │ 13.29 Mitem/s │ 13.24 Mitem/s │         │
├─ bench_binary_gcd_noswap     1.24 ms       │ 1.334 ms      │ 1.244 ms      │ 1.25 ms       │ 100     │ 100
│                              13.2 Mitem/s  │ 12.27 Mitem/s │ 13.16 Mitem/s │ 13.1 Mitem/s  │         │
├─ bench_binary_gcd_recursive  1.338 ms      │ 1.4 ms        │ 1.341 ms      │ 1.346 ms      │ 100     │ 100
│                              12.23 Mitem/s │ 11.69 Mitem/s │ 12.2 Mitem/s  │ 12.16 Mitem/s │         │
├─ bench_binary_gcd_swap       1.488 ms      │ 1.549 ms      │ 1.491 ms      │ 1.496 ms      │ 100     │ 100
│                              11 Mitem/s    │ 10.57 Mitem/s │ 10.98 Mitem/s │ 10.95 Mitem/s │         │
├─ bench_binary_gcd_swap_v2    1.494 ms      │ 1.559 ms      │ 1.497 ms      │ 1.502 ms      │ 100     │ 100
│                              10.96 Mitem/s │ 10.5 Mitem/s  │ 10.93 Mitem/s │ 10.9 Mitem/s  │         │
╰─ bench_num_gcd               1.415 ms      │ 1.47 ms       │ 1.419 ms      │ 1.422 ms      │ 100     │ 100
                               11.57 Mitem/s │ 11.14 Mitem/s │ 11.54 Mitem/s │ 11.51 Mitem/s │         │
```
