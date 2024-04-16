# gcd

On Macbook Pro M1

```
% cargo bench -p gcd --bench benchmark_divan
      Finished bench [optimized] target(s) in 0.03s
       Running benches/benchmark_divan.rs (/Users/koyeung/Documents/lab/playground.nosync.noindex/try-rs-benches/target/release/deps/benchmark_divan-ce67760f36894a9b)
  Timer precision: 41 ns
  benchmark_divan              fastest       │ slowest       │ median        │ mean          │ samples │ iters
  ├─ bench_0_naive_gcd         2.233 ms      │ 2.316 ms      │ 2.237 ms      │ 2.246 ms      │ 100     │ 100
  │                            7.337 Mitem/s │ 7.072 Mitem/s │ 7.321 Mitem/s │ 7.293 Mitem/s │         │
  ├─ bench_binary_gcd_minmax   1.228 ms      │ 1.333 ms      │ 1.232 ms      │ 1.236 ms      │ 100     │ 100
  │                            13.33 Mitem/s │ 12.28 Mitem/s │ 13.29 Mitem/s │ 13.24 Mitem/s │         │
  ├─ bench_binary_gcd_noswap   1.241 ms      │ 1.339 ms      │ 1.244 ms      │ 1.25 ms       │ 100     │ 100
  │                            13.2 Mitem/s  │ 12.22 Mitem/s │ 13.16 Mitem/s │ 13.1 Mitem/s  │         │
  ├─ bench_binary_gcd_swap     1.488 ms      │ 1.537 ms      │ 1.491 ms      │ 1.496 ms      │ 100     │ 100
  │                            11 Mitem/s    │ 10.65 Mitem/s │ 10.98 Mitem/s │ 10.94 Mitem/s │         │
  ├─ bench_binary_gcd_swap_v2  1.493 ms      │ 1.57 ms       │ 1.497 ms      │ 1.501 ms      │ 100     │ 100
  │                            10.97 Mitem/s │ 10.43 Mitem/s │ 10.94 Mitem/s │ 10.91 Mitem/s │         │
  ╰─ bench_num_gcd             1.414 ms      │ 1.478 ms      │ 1.418 ms      │ 1.423 ms      │ 100     │ 100
                               11.57 Mitem/s │ 11.07 Mitem/s │ 11.55 Mitem/s │ 11.51 Mitem/s │         │
```
