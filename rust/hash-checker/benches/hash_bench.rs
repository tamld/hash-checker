use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

const MB: usize = 1024 * 1024;
const SIZES: [usize; 3] = [1 * MB, 10 * MB, 50 * MB];
const ALGORITHMS: [&str; 4] = ["sha256", "sha512", "blake2s", "blake2b"];

fn bench_hashing(c: &mut Criterion) {
    let tmp_dir = tempfile::tempdir().expect("tempdir");

    for &size in &SIZES {
        let file_path = tmp_dir.path().join(format!("sample_{}mb.bin", size / MB));
        ensure_file_with_size(&file_path, size);

        for &algo in &ALGORITHMS {
            let bench_name = format!("{} {}MB", algo, size / MB);
            c.bench_function(&bench_name, |b| {
                b.iter_batched(
                    || file_path.clone(),
                    |path| hash_checker::compute_hash(path.as_path(), algo).expect("hash"),
                    BatchSize::SmallInput,
                );
            });
        }
    }
}

fn ensure_file_with_size(path: &Path, size: usize) {
    if path.exists() {
        return;
    }
    let file = File::create(path).expect("create file");
    let mut writer = BufWriter::new(file);
    let pattern = [0u8; MB];
    let mut remaining = size;
    while remaining >= MB {
        writer.write_all(&pattern).expect("write chunk");
        remaining -= MB;
    }
    if remaining > 0 {
        writer
            .write_all(&pattern[..remaining])
            .expect("write remainder");
    }
    writer.flush().expect("flush");
}

criterion_group!(benches, bench_hashing);
criterion_main!(benches);
