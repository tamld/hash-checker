use assert_cmd::Command;
use hash_checker::{compute_hash, run_batch, BatchInput, BatchStatus};
use serde_json::json;
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;

fn write_temp_file(contents: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("temp file");
    file.write_all(contents.as_bytes()).expect("write");
    file
}

#[test]
fn run_batch_reports_match_and_mismatch() {
    let file_ok = write_temp_file("ok");
    let file_bad = write_temp_file("bad");

    let expected_ok = compute_hash(file_ok.path(), "sha256").expect("hash");
    let wrong_hash = "f".repeat(64);

    let inputs = vec![
        BatchInput {
            path: file_ok.path().to_path_buf(),
            expected: expected_ok.clone(),
            algorithm: Some("sha256".into()),
        },
        BatchInput {
            path: file_bad.path().to_path_buf(),
            expected: wrong_hash.clone(),
            algorithm: Some("sha256".into()),
        },
        BatchInput {
            path: file_bad.path().parent().unwrap().join("missing.txt"),
            expected: wrong_hash,
            algorithm: None,
        },
    ];

    let report = run_batch(&inputs);
    assert_eq!(report.summary.total, 3);
    assert_eq!(report.summary.matched, 1);
    assert_eq!(report.summary.mismatched, 1);
    assert_eq!(report.summary.missing, 1);
    assert_eq!(report.summary.errored, 0);

    let statuses: Vec<BatchStatus> = report.entries.iter().map(|e| e.status.clone()).collect();
    assert!(statuses.contains(&BatchStatus::Match));
    assert!(statuses.contains(&BatchStatus::Mismatch));
    assert!(statuses.contains(&BatchStatus::Missing));
}

#[test]
fn cli_batch_outputs_json() {
    let file_ok = write_temp_file("hash-checker");
    let digest = compute_hash(file_ok.path(), "sha256").expect("hash");

    let tmp = NamedTempFile::new().expect("temp input");
    let payload = json!([{
        "path": file_ok.path().display().to_string(),
        "expected": digest,
        "algorithm": "sha256"
    }]);
    serde_json::to_writer(&File::create(tmp.path()).expect("create input"), &payload)
        .expect("write json");

    let mut cmd = Command::cargo_bin("hash-checker").expect("binary");
    cmd.args([
        "batch",
        "--input",
        tmp.path().to_str().unwrap(),
        "--output-format",
        "json",
    ]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("\"matched\": 1"));
}
