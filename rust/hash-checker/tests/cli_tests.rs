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
fn cli_conflicting_algorithm_hints_error_message_names_both_algorithms() {
    // Verify the CLI error output mentions both the flag algorithm and the prefixed algorithm.
    let (temp_dir, path) = sample_file();
    let digest = compute_hash(path.as_path(), "sha256").expect("hash");
    let prefixed = format!("sha512:{digest}");
    let mut cmd = Command::cargo_bin("hash-checker").expect("binary");
    cmd.arg(&path)
        .arg(prefixed)
        .arg("--algorithm")
        .arg("sha256");
    let output = cmd.output().expect("run command");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("sha256"),
        "stderr should mention flag algorithm: {stderr}"
    );
    assert!(
        stderr.contains("sha512"),
        "stderr should mention prefix algorithm: {stderr}"
    );
    drop(temp_dir);
}

#[test]
fn cli_accepts_matching_prefix_and_algorithm_flag() {
    // When digest prefix and --algorithm flag agree, verification should succeed.
    let (temp_dir, path) = sample_file();
    let digest = compute_hash(path.as_path(), "sha256").expect("hash");
    let prefixed = format!("sha256:{digest}");
    let mut cmd = Command::cargo_bin("hash-checker").expect("binary");
    cmd.arg(&path).arg(prefixed).arg("--algorithm").arg("sha256");
    cmd.assert().success().stdout(contains("Hashes match"));
    drop(temp_dir);
}

#[test]
fn cli_rejects_conflicting_hints_different_algorithm_pair() {
    // Conflict detection should also fire for pairs other than sha256/sha512.
    let (temp_dir, path) = sample_file();
    let digest = compute_hash(path.as_path(), "sha1").expect("hash");
    let prefixed = format!("md5:{digest}");
    let mut cmd = Command::cargo_bin("hash-checker").expect("binary");
    cmd.arg(&path)
        .arg(prefixed)
        .arg("--algorithm")
        .arg("sha1");
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