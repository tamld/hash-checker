use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use tempfile::tempdir;

#[test]
fn headless_snapshot_writes_json() {
    let dir = tempdir().expect("tempdir");
    let json_path = dir.path().join("state.json");

    Command::cargo_bin("hash-checker-gui")
        .expect("binary")
        .args(["--headless", "--snapshot", json_path.to_str().unwrap()])
        .assert()
        .success();

    let data = fs::read_to_string(&json_path).expect("read json");
    let value: Value = serde_json::from_str(&data).expect("valid json");
    let captures = value
        .get("captures")
        .and_then(|c| c.as_array())
        .expect("captures array");
    assert!(!captures.is_empty(), "captures should not be empty");
}

#[test]
fn headless_manifest_requires_snapshot() {
    Command::cargo_bin("hash-checker-gui")
        .expect("binary")
        .args(["--headless", "--manifest-dir", "foo"])
        .assert()
        .failure()
        .code(2);
}

#[test]
fn capture_golden_respects_output_dir() {
    let dir = tempdir().expect("tempdir");
    std::env::set_var("HASH_CHECKER_GOLDEN_DIR", dir.path());

    Command::cargo_bin("hash-checker-gui")
        .expect("binary")
        .args(["--headless", "--capture-golden", "minimal-scan"])
        .assert()
        .success();

    let platform_dir = dir.path().join(std::env::consts::OS);
    let file_path = platform_dir.join("minimal-scan.json");
    let contents = fs::read_to_string(file_path).expect("golden file");
    let value: Value = serde_json::from_str(&contents).expect("valid json");
    assert_eq!(value["version"], "1.0.0");
    std::env::remove_var("HASH_CHECKER_GOLDEN_DIR");
}

#[test]
fn compare_golden_detects_changes() {
    let dir = tempdir().expect("tempdir");

    // baseline capture
    Command::cargo_bin("hash-checker-gui")
        .expect("binary")
        .env("HASH_CHECKER_GOLDEN_DIR", dir.path())
        .args(["--headless", "--capture-golden", "minimal-scan"])
        .assert()
        .success();

    // match case
    Command::cargo_bin("hash-checker-gui")
        .expect("binary")
        .env("HASH_CHECKER_GOLDEN_DIR", dir.path())
        .args(["--headless", "--compare-golden", "minimal-scan"])
        .assert()
        .success();

    // mutate golden to trigger diff
    let golden_path = dir
        .path()
        .join(std::env::consts::OS)
        .join("minimal-scan.json");
    let mut value: Value =
        serde_json::from_str(&fs::read_to_string(&golden_path).unwrap()).expect("json");
    if let Some(version) = value.get_mut("version") {
        *version = Value::String("9.9.9".to_owned());
    }
    fs::write(&golden_path, serde_json::to_string_pretty(&value).unwrap()).unwrap();

    Command::cargo_bin("hash-checker-gui")
        .expect("binary")
        .env("HASH_CHECKER_GOLDEN_DIR", dir.path())
        .args(["--headless", "--compare-golden", "minimal-scan"])
        .assert()
        .failure()
        .code(1);
}

#[test]
fn compare_golden_fuzzy_tolerates_dimension_drift() {
    let dir = tempdir().expect("tempdir");

    // baseline capture
    Command::cargo_bin("hash-checker-gui")
        .expect("binary")
        .env("HASH_CHECKER_GOLDEN_DIR", dir.path())
        .args(["--headless", "--capture-golden", "minimal-scan"])
        .assert()
        .success();

    let platform_dir = dir.path().join(std::env::consts::OS);
    let golden_path = platform_dir.join("minimal-scan.json");
    let mut value: Value =
        serde_json::from_str(&fs::read_to_string(&golden_path).unwrap()).expect("json");
    if let Some(window) = value
        .get_mut("captures")
        .and_then(|captures| captures.get_mut(0))
        .and_then(|capture| capture.get_mut("window"))
        .and_then(|window| window.as_object_mut())
    {
        window.insert("width".to_owned(), Value::from(1284));
        window.insert("height".to_owned(), Value::from(804));
    }
    fs::write(&golden_path, serde_json::to_string_pretty(&value).unwrap()).unwrap();

    // Structural (default) should fail
    Command::cargo_bin("hash-checker-gui")
        .expect("binary")
        .env("HASH_CHECKER_GOLDEN_DIR", dir.path())
        .args(["--headless", "--compare-golden", "minimal-scan"])
        .assert()
        .failure()
        .code(1);

    // Fuzzy should accept tolerance
    Command::cargo_bin("hash-checker-gui")
        .expect("binary")
        .env("HASH_CHECKER_GOLDEN_DIR", dir.path())
        .args([
            "--headless",
            "--compare-mode",
            "fuzzy",
            "--compare-golden",
            "minimal-scan",
        ])
        .assert()
        .success();
}
