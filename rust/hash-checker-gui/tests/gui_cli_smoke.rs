//! CLI smoke tests for the `hash-checker-gui` binary.
//!
//! The GUI crate currently exposes CLI flags primarily for pre-loading manifests.
//! These tests focus on exercising stable, headless-friendly paths so they can run
//! inside CI without needing a display server.

use assert_cmd::Command;

fn gui_binary() -> Command {
    Command::cargo_bin("hash-checker-gui").expect("binary should compile")
}

#[test]
fn help_lists_manifest_flags() {
    let assert = gui_binary().arg("--help").assert().success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert!(
        stdout.contains("--manifest-dir"),
        "help output should mention --manifest-dir"
    );
    assert!(
        stdout.contains("--manifest-report"),
        "help output should mention --manifest-report"
    );
    assert!(
        stdout.contains("--manifest-algorithm"),
        "help output should mention --manifest-algorithm"
    );
}

#[test]
fn unknown_flag_produces_error() {
    let assert = gui_binary()
        .arg("--definitely-not-a-real-flag")
        .assert()
        .failure();
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(
        stderr.contains("Unrecognized argument: --definitely-not-a-real-flag"),
        "stderr should report the unknown argument"
    );
}

#[test]
fn version_flag_currently_absent() {
    let assert = gui_binary().arg("--version").assert().failure();
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(
        stderr.contains("Unrecognized argument: --version"),
        "stderr should explain that --version is not yet supported"
    );
}

#[test]
fn compare_mode_requires_compare_golden() {
    let assert = gui_binary()
        .args(["--headless", "--compare-mode", "fuzzy"])
        .assert()
        .failure();
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(
        stderr.contains("--compare-mode requires --compare-golden"),
        "stderr should mention missing --compare-golden dependency"
    );
}

#[test]
fn compare_mode_rejects_invalid_value() {
    let assert = gui_binary()
        .args([
            "--headless",
            "--compare-mode",
            "approximate",
            "--compare-golden",
            "minimal-scan",
        ])
        .assert()
        .failure();
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(
        stderr.contains("Unsupported compare mode"),
        "stderr should explain valid compare modes"
    );
}
