use assert_cmd::Command;
use predicates::str::contains;
use std::fs;
use tempfile::TempDir;

fn sample_directory() -> TempDir {
    let dir = TempDir::new().expect("temp dir");
    fs::create_dir_all(dir.path().join("nested")).expect("create nested");
    fs::write(dir.path().join("alpha.txt"), b"alpha").expect("write alpha");
    fs::write(dir.path().join("nested").join("beta.txt"), b"beta").expect("write beta");
    dir
}

#[test]
fn export_manifest_to_file_and_verify_success() {
    let dir = sample_directory();
    let manifest_path = dir.path().join("manifest.json");

    let mut export_cmd = Command::cargo_bin("hash-checker").expect("binary");
    export_cmd.args([
        "manifest",
        "export",
        dir.path().to_str().unwrap(),
        "-o",
        manifest_path.to_str().unwrap(),
        "-r",
    ]);
    export_cmd.assert().success();

    let mut verify_cmd = Command::cargo_bin("hash-checker").expect("binary");
    verify_cmd.args(["manifest", "verify", manifest_path.to_str().unwrap()]);
    verify_cmd
        .assert()
        .success()
        .stdout(contains("All 2 entries matched"));
}

#[test]
fn verify_detects_mismatch_after_modification() {
    let dir = sample_directory();
    let manifest_path = dir.path().join("manifest.txt");

    let mut export_cmd = Command::cargo_bin("hash-checker").expect("binary");
    export_cmd.args([
        "manifest",
        "export",
        dir.path().to_str().unwrap(),
        "-o",
        manifest_path.to_str().unwrap(),
        "--format",
        "txt",
        "-r",
    ]);
    export_cmd.assert().success();

    fs::write(dir.path().join("alpha.txt"), b"mutated").expect("mutate file");

    let mut verify_cmd = Command::cargo_bin("hash-checker").expect("binary");
    verify_cmd.args(["manifest", "verify", manifest_path.to_str().unwrap()]);
    verify_cmd
        .assert()
        .code(3)
        .stdout(contains("Mismatched files"));
}
