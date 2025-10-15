use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use tempfile::tempdir;

const MB: usize = 1024 * 1024;
const SIZES: [usize; 3] = [MB, 10 * MB, 50 * MB];
const ALGORITHMS: [&str; 4] = ["sha256", "sha512", "blake2s", "blake2b"];

fn bench_hashing(c: &mut Criterion) {
    let tmp_dir = tempdir().expect("tempdir");
    let mut group = c.benchmark_group("hash_file");
    if std::env::var("CI").is_ok() {
        group.sample_size(5);
        group.measurement_time(Duration::from_secs(1));
        group.warm_up_time(Duration::from_millis(500));
    } else {
        group.sample_size(10);
        group.measurement_time(Duration::from_secs(2));
        group.warm_up_time(Duration::from_secs(1));
    }

    for &size in &SIZES {
        let file_path = prepare_file(tmp_dir.path().join(format!("sample_{}mb.bin", size / MB)), size);
        group.throughput(Throughput::Bytes(size as u64));

        for &algo in &ALGORITHMS {
            group.bench_function(BenchmarkId::new(algo, size / MB), |b| {
                b.iter_batched(
                    || file_path.clone(),
                    |path| hash_checker::compute_hash(path.as_path(), algo).expect("hash"),
                    BatchSize::SmallInput,
                );
            });
        }
    }

    group.finish();
}

fn prepare_file(path: PathBuf, size: usize) -> PathBuf {
    if path.exists() {
        return path;
    }

    let file = File::create(&path).expect("create file");
    let mut writer = BufWriter::new(file);
    let chunk = [0u8; MB];
    let mut remaining = size;

    while remaining >= MB {
        writer.write_all(&chunk).expect("write chunk");
        remaining -= MB;
    }
    if remaining > 0 {
        writer.write_all(&chunk[..remaining]).expect("write tail");
    }
    writer.flush().expect("flush");
    path
}

criterion_group!(benches, bench_hashing);
criterion_main!(benches);
