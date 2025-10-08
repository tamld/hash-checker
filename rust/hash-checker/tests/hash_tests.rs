use std::io::Write;

use hash_checker::{compute_hash, detect_algorithm, supported_algorithms, verify_hash};
use tempfile::NamedTempFile;

const SAMPLE_TEXT: &str = "This is a test file for the hash checker.";

fn write_sample_file() -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("create temp file");
    file.write_all(SAMPLE_TEXT.as_bytes())
        .expect("write sample text");
    file
}

#[test]
fn compute_hash_sha256_matches_expected() {
    let file = write_sample_file();
    let digest = compute_hash(file.path(), "sha256").expect("hash");
    assert_eq!(
        digest,
        "19fe5f3e518ba46537ddf4bcd098d66e2873fda2dccf58e66f6ab1f932c6d811"
    );
}

#[test]
fn compute_hash_md5_matches_expected() {
    let file = write_sample_file();
    let digest = compute_hash(file.path(), "md5").expect("hash");
    assert_eq!(digest, "109788a70f52a60437d3c8867124ca72");
}

#[test]
fn detect_algorithm_by_length() {
    assert_eq!(
        detect_algorithm("109788a70f52a60437d3c8867124ca72"),
        Some("md5")
    );
    assert_eq!(
        detect_algorithm("19fe5f3e518ba46537ddf4bcd098d66e2873fda2dccf58e66f6ab1f932c6d811"),
        Some("sha256")
    );
    assert_eq!(detect_algorithm(""), None);
}

#[test]
fn verify_hash_matches_expected() {
    let file = write_sample_file();
    let (matches, computed) = verify_hash(
        file.path(),
        "19fe5f3e518ba46537ddf4bcd098d66e2873fda2dccf58e66f6ab1f932c6d811",
        None,
    )
    .expect("verify");
    assert!(matches);
    assert_eq!(
        computed,
        "19fe5f3e518ba46537ddf4bcd098d66e2873fda2dccf58e66f6ab1f932c6d811"
    );
}

#[test]
fn verify_hash_infers_blake2s() {
    let file = write_sample_file();
    let blake2s = compute_hash(file.path(), "blake2s").expect("hash");
    let (matches, computed) = verify_hash(file.path(), &blake2s, None).expect("verify");
    assert!(matches);
    assert_eq!(computed, blake2s);
}

#[test]
fn verify_hash_infers_blake2b() {
    let file = write_sample_file();
    let blake2b = compute_hash(file.path(), "blake2b").expect("hash");
    let (matches, computed) = verify_hash(file.path(), &blake2b, None).expect("verify");
    assert!(matches);
    assert_eq!(computed, blake2b);
}

#[test]
fn supported_algorithms_contains_expected() {
    let algos = supported_algorithms();
    for name in ["md5", "sha1", "sha256", "blake2b"] {
        assert!(algos.contains(&name));
    }
}
