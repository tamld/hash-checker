use std::io::Write;

use assert_cmd::Command;
use predicates::str::contains;
use tempfile::NamedTempFile;

const SAMPLE_TEXT: &str = "This is a test file for the hash checker.";

fn sample_file() -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("temp file");
    file.write_all(SAMPLE_TEXT.as_bytes()).expect("write");
    file
}

#[test]
fn cli_reports_success_when_hash_matches() {
    let file = sample_file();
    let mut cmd = Command::cargo_bin("hash-checker").expect("binary");
    cmd.arg(file.path())
        .arg("19fe5f3e518ba46537ddf4bcd098d66e2873fda2dccf58e66f6ab1f932c6d811");
    cmd.assert()
        .success()
        .stdout(contains("Hashes match"));
}

#[test]
fn cli_reports_failure_when_hash_mismatch() {
    let file = sample_file();
    let mut cmd = Command::cargo_bin("hash-checker").expect("binary");
    cmd.arg(file.path())
        .arg("deadbeef");
    cmd.assert()
        .code(1)
        .stderr(contains("Verification failed"));
}

#[test]
fn cli_reports_mismatch_exit_code_three() {
    let file = sample_file();
    let mut cmd = Command::cargo_bin("hash-checker").expect("binary");
    cmd.arg(file.path())
        .arg("0000000000000000000000000000000000000000000000000000000000000000");
    cmd.assert()
        .code(3)
        .stderr(contains("Hashes do not match"));
}

#[test]
fn list_algorithms_outputs_supported() {
    let mut cmd = Command::cargo_bin("hash-checker").expect("binary");
    cmd.arg("--list-algorithms");
    cmd.assert()
        .success()
        .stdout(contains("sha256"));
}
