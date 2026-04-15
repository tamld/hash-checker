use std::io::Write;

use hash_checker::{compute_hash, detect_algorithm, supported_algorithms, verify_hash, HashError};
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
fn detect_algorithm_with_prefix() {
    assert_eq!(
        detect_algorithm("sha256:19fe5f3e518ba46537ddf4bcd098d66e2873fda2dccf58e66f6ab1f932c6d811"),
        Some("sha256")
    );
}

#[test]
fn detect_algorithm_edge_cases() {
    // Prefix with whitespace padding and mixed case
    assert_eq!(
        detect_algorithm("  sHa256 : 19fe5f3e518ba46537ddf4bcd098d66e2873fda2dccf58e66f6ab1f932c6d811  "),
        Some("sha256")
    );

    // Unknown prefix fallback to length-based inference
    assert_eq!(
        detect_algorithm("unknown:109788a70f52a60437d3c8867124ca72"),
        Some("md5") // 32 chars
    );

    // Valid prefix prioritized over length-based inference
    // 32 chars is md5, but prefix is sha256
    assert_eq!(
        detect_algorithm("sha256:109788a70f52a60437d3c8867124ca72"),
        Some("sha256")
    );

    // Multiple colons: only the first is the boundary
    // The "digest" becomes "bar:baz" which is 7 chars.
    // However, if the prefix is valid, it returns the prefix.
    assert_eq!(
        detect_algorithm("sha256:bar:baz"),
        Some("sha256")
    );

    // Multiple colons with unknown prefix and invalid digest length
    assert_eq!(
        detect_algorithm("foo:bar:baz"),
        None
    );

    // No prefix and unsupported length
    assert_eq!(
        detect_algorithm("12345"), // 5 chars
        None
    );

    // Check all known lengths
    assert_eq!(detect_algorithm(&"a".repeat(32)), Some("md5"));
    assert_eq!(detect_algorithm(&"a".repeat(40)), Some("sha1"));
    assert_eq!(detect_algorithm(&"a".repeat(56)), Some("sha224"));
    assert_eq!(detect_algorithm(&"a".repeat(64)), Some("sha256"));
    assert_eq!(detect_algorithm(&"a".repeat(96)), Some("sha384"));
    assert_eq!(detect_algorithm(&"a".repeat(128)), Some("sha512"));

    // Empty prefix
    assert_eq!(
        detect_algorithm(":109788a70f52a60437d3c8867124ca72"),
        Some("md5") // Falls back to length because prefix is empty
    );
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
fn verify_hash_accepts_prefixed_digest() {
    let file = write_sample_file();
    let digest = compute_hash(file.path(), "sha256").expect("hash");
    let prefixed = format!("sha256:{digest}");
    let (matches, computed) = verify_hash(file.path(), &prefixed, None).expect("verify");
    assert!(matches);
    assert_eq!(computed, digest);
}

#[test]
fn verify_hash_rejects_unknown_prefix() {
    let file = write_sample_file();
    let digest = compute_hash(file.path(), "sha256").expect("hash");
    let prefixed = format!("sha999:{digest}");
    let err = verify_hash(file.path(), &prefixed, None).expect_err("expected failure");
    assert!(matches!(err, HashError::UnsupportedAlgorithm(_)));
}

#[test]
fn supported_algorithms_contains_expected() {
    let algos = supported_algorithms();
    for name in ["md5", "sha1", "sha256", "blake2b"] {
        assert!(algos.contains(&name));
    }
}

#[test]
fn compute_hash_rejects_directory_paths() {
    let dir = tempfile::tempdir().expect("temp dir");
    let err = compute_hash(dir.path(), "sha256").expect_err("expected directory failure");
    assert!(matches!(err, HashError::NotAFile(_)));
}

#[test]
fn verify_hash_accepts_canonicalised_paths() {
    let dir = tempfile::tempdir().expect("temp dir");
    let file_path = dir.path().join("sample.txt");
    std::fs::write(&file_path, SAMPLE_TEXT.as_bytes()).expect("write sample");
    let nested_dir = dir.path().join("nested");
    std::fs::create_dir(&nested_dir).expect("create nested dir");
    let tricky_path = nested_dir.join("..").join("sample.txt");
    let digest = compute_hash(&file_path, "sha256").expect("hash");
    let (matches, _) = verify_hash(&tricky_path, &digest, None).expect("verify");
    assert!(matches);
}
