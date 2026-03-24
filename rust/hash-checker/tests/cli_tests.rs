use assert_cmd::Command;
use hash_checker::compute_hash;
use predicates::str::contains;
use std::path::PathBuf;
use tempfile::TempDir;

const SAMPLE_TEXT: &str = "This is a test file for the hash checker.";

fn sample_file() -> (TempDir, PathBuf) {
    let dir = TempDir::new().expect("temp dir");
    let path = dir.path().join("sample.txt");
    std::fs::write(&path, SAMPLE_TEXT.as_bytes()).expect("write");
    (dir, path)
}

#[test]
fn cli_reports_success_when_hash_matches() {
    let (temp_dir, path) = sample_file();
    let mut cmd = Command::cargo_bin("hash-checker").expect("binary");
    cmd.arg(&path)
        .arg("19fe5f3e518ba46537ddf4bcd098d66e2873fda2dccf58e66f6ab1f932c6d811");
    cmd.env("RUST_BACKTRACE", "1");
    cmd.assert().success().stdout(contains("Hashes match"));
    drop(temp_dir);
}

#[test]
fn cli_reports_failure_when_hash_mismatch() {
    let (temp_dir, path) = sample_file();
    let mut cmd = Command::cargo_bin("hash-checker").expect("binary");
    cmd.arg(&path).arg("deadbeef");
    cmd.env("RUST_BACKTRACE", "1");
    cmd.assert().code(1).stderr(contains("Verification failed"));
    drop(temp_dir);
}

#[test]
fn cli_reports_mismatch_exit_code_three() {
    let (temp_dir, path) = sample_file();
    let mut cmd = Command::cargo_bin("hash-checker").expect("binary");
    cmd.arg(&path)
        .arg("0000000000000000000000000000000000000000000000000000000000000000");
    cmd.env("RUST_BACKTRACE", "1");
    cmd.assert().code(3).stderr(contains("Hashes do not match"));
    drop(temp_dir);
}

#[test]
fn list_algorithms_outputs_supported() {
    let mut cmd = Command::cargo_bin("hash-checker").expect("binary");
    cmd.arg("--list-algorithms");
    cmd.env("RUST_BACKTRACE", "1");
    cmd.assert().success().stdout(contains("sha256"));
}

#[test]
fn cli_infers_blake2s_without_algorithm_flag() {
    let (temp_dir, path) = sample_file();
    let digest = compute_hash(path.as_path(), "blake2s").expect("hash");
    let mut cmd = Command::cargo_bin("hash-checker").expect("binary");
    cmd.arg(&path).arg(digest);
    cmd.env("RUST_BACKTRACE", "1");
    cmd.assert().success().stdout(contains("Hashes match"));
    drop(temp_dir);
}

#[test]
fn cli_infers_blake2b_without_algorithm_flag() {
    let (temp_dir, path) = sample_file();
    let digest = compute_hash(path.as_path(), "blake2b").expect("hash");
    let mut cmd = Command::cargo_bin("hash-checker").expect("binary");
    cmd.arg(&path).arg(digest);
    cmd.env("RUST_BACKTRACE", "1");
    cmd.assert().success().stdout(contains("Hashes match"));
    drop(temp_dir);
}

#[test]
fn cli_rejects_conflicting_algorithm_hints() {
    let (temp_dir, path) = sample_file();
    let digest = compute_hash(path.as_path(), "sha256").expect("hash");
    let prefixed = format!("sha512:{digest}");
    let mut cmd = Command::cargo_bin("hash-checker").expect("binary");
    cmd.arg(&path)
        .arg(prefixed)
        .arg("--algorithm")
        .arg("sha256");
    cmd.assert()
        .code(1)
        .stderr(contains("conflicting algorithm hints"));
    drop(temp_dir);
}

#[test]
fn cli_emits_json_logs_when_requested() {
    let (temp_dir, path) = sample_file();
    let digest = compute_hash(path.as_path(), "sha256").expect("hash");
    let mut cmd = Command::cargo_bin("hash-checker").expect("binary");
    cmd.arg(&path).arg(digest).arg("--log-format").arg("json");
    cmd.assert()
        .success()
        .stdout(contains("Hashes match"))
        .stderr(contains("\"Hashes match\""));
    drop(temp_dir);
}
