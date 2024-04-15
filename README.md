# try-rs-benches

## Package gcd

On Macbook Pro M1

```
  % cargo bench -p gcd --bench benchmark_divan
      Finished bench [optimized] target(s) in 0.03s
       Running benches/benchmark_divan.rs (target/release/deps/benchmark_divan-ce67760f36894a9b)
  Timer precision: 41 ns
  benchmark_divan              fastest       │ slowest       │ median        │ mean          │ samples │ iters
  ├─ bench_0_naive_gcd         2.234 ms      │ 2.41 ms       │ 2.238 ms      │ 2.25 ms       │ 100     │ 100
  │                            7.331 Mitem/s │ 6.797 Mitem/s │ 7.317 Mitem/s │ 7.279 Mitem/s │         │
  ├─ bench_binary_gcd_minmax   1.228 ms      │ 1.318 ms      │ 1.232 ms      │ 1.236 ms      │ 100     │ 100
  │                            13.33 Mitem/s │ 12.42 Mitem/s │ 13.29 Mitem/s │ 13.24 Mitem/s │         │
  ├─ bench_binary_gcd_noswap   1.24 ms       │ 1.357 ms      │ 1.244 ms      │ 1.25 ms       │ 100     │ 100
  │                            13.2 Mitem/s  │ 12.06 Mitem/s │ 13.16 Mitem/s │ 13.1 Mitem/s  │         │
  ├─ bench_binary_gcd_swap     1.488 ms      │ 1.597 ms      │ 1.492 ms      │ 1.499 ms      │ 100     │ 100
  │                            11 Mitem/s    │ 10.25 Mitem/s │ 10.98 Mitem/s │ 10.92 Mitem/s │         │
  ├─ bench_binary_gcd_swap_v2  1.493 ms      │ 1.585 ms      │ 1.497 ms      │ 1.504 ms      │ 100     │ 100
  │                            10.97 Mitem/s │ 10.33 Mitem/s │ 10.94 Mitem/s │ 10.89 Mitem/s │         │
  ╰─ bench_num_gcd             1.414 ms      │ 1.521 ms      │ 1.419 ms      │ 1.428 ms      │ 100     │ 100
                               11.57 Mitem/s │ 10.76 Mitem/s │ 11.54 Mitem/s │ 11.46 Mitem/s │         │
```
