# Criterion Benchmarks – 2025-10-15

Environment: macOS (M-series runner), Rust stable.

| Size (MB) | SHA-256 | SHA-512 | BLAKE2s | BLAKE2b |
| --- | --- | --- | --- | --- |
| 1 | ~530 MiB/s | ~810 MiB/s | ~820 MiB/s | ~1.31 GiB/s |
| 10 | ~517 MiB/s | ~820 MiB/s | ~815 MiB/s | ~1.30 GiB/s |
| 50 | ~524 MiB/s | ~816 MiB/s | ~810 MiB/s | ~1.29 GiB/s |

See `cargo bench --manifest-path rust/hash-checker/Cargo.toml` for detailed reports under `target/criterion/`.
