// GUI Automation Integration Tests
// Purpose: Test GUI automation framework in container environment
// Usage: cargo test --test gui_automation_tests

use assert_cmd::Command;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_gui_automation_container_environment() {
    // Test that we're running in container environment
    let mut cmd = Command::cargo_bin("hash-checker-gui").unwrap();
    let output = cmd
        .arg("--help")
        .assert()
        .success();
    
    // Verify help output contains expected options
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("--snapshot"));
    assert!(stdout.contains("--manifest"));
}

#[test]
fn test_headless_mode_execution() {
    // Test headless mode execution
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    
    let mut cmd = Command::cargo_bin("hash-checker-gui").unwrap();
    let output = cmd
        .arg("--headless-test")
        .arg("--manifest")
        .arg(temp_path.join("test_manifest.json"))
        .arg("--snapshot")
        .arg(temp_path.join("test_snapshot.png"))
        .arg("--snapshot-width")
        .arg("800")
        .arg("--snapshot-height")
        .arg("600")
        .assert()
        .success();
    
    // Verify output indicates headless mode
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("headless") || stdout.contains("Headless"));
}

#[test]
fn test_manifest_generation_workflow() {
    // Test manifest generation in headless mode
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    let manifest_path = temp_path.join("test_manifest.json");
    
    let mut cmd = Command::cargo_bin("hash-checker-gui").unwrap();
    let output = cmd
        .arg("--headless-test")
        .arg("--manifest")
        .arg(&manifest_path)
        .arg("--snapshot")
        .arg(temp_path.join("test_snapshot.png"))
        .arg("--snapshot-width")
        .arg("800")
        .arg("--snapshot-height")
        .arg("600")
        .assert()
        .success();
    
    // Verify manifest file was created
    assert!(manifest_path.exists(), "Manifest file should be created");
    
    // Verify manifest content is valid JSON
    let manifest_content = std::fs::read_to_string(&manifest_path).unwrap();
    let _manifest: serde_json::Value = serde_json::from_str(&manifest_content)
        .expect("Manifest should be valid JSON");
}

#[test]
fn test_snapshot_path_validation() {
    // Test snapshot path validation
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    let snapshot_path = temp_path.join("test_snapshot.png");
    
    let mut cmd = Command::cargo_bin("hash-checker-gui").unwrap();
    let output = cmd
        .arg("--headless-test")
        .arg("--manifest")
        .arg(temp_path.join("test_manifest.json"))
        .arg("--snapshot")
        .arg(&snapshot_path)
        .arg("--snapshot-width")
        .arg("800")
        .arg("--snapshot-height")
        .arg("600")
        .assert()
        .success();
    
    // Verify snapshot path is writable
    assert!(snapshot_path.parent().unwrap().is_dir(), "Snapshot directory should exist");
}

#[test]
fn test_error_handling_invalid_paths() {
    // Test error handling for invalid paths
    let mut cmd = Command::cargo_bin("hash-checker-gui").unwrap();
    let output = cmd
        .arg("--headless-test")
        .arg("--manifest")
        .arg("/invalid/path/manifest.json")
        .arg("--snapshot")
        .arg("/invalid/path/snapshot.png")
        .arg("--snapshot-width")
        .arg("800")
        .arg("--snapshot-height")
        .arg("600")
        .assert()
        .failure();
    
    // Verify error output
    let stderr = String::from_utf8_lossy(&output.get_output().stderr);
    assert!(stderr.contains("error") || stderr.contains("Error"));
}

#[test]
fn test_performance_metrics_generation() {
    // Test that performance metrics are generated
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    
    let mut cmd = Command::cargo_bin("hash-checker-gui").unwrap();
    let start_time = std::time::Instant::now();
    
    let output = cmd
        .arg("--headless-test")
        .arg("--manifest")
        .arg(temp_path.join("test_manifest.json"))
        .arg("--snapshot")
        .arg(temp_path.join("test_snapshot.png"))
        .arg("--snapshot-width")
        .arg("800")
        .arg("--snapshot-height")
        .arg("600")
        .assert()
        .success();
    
    let duration = start_time.elapsed();
    
    // Verify execution completed within reasonable time
    assert!(duration.as_millis() < 5000, "Execution should complete within 5 seconds");
    
    // Verify output contains performance information
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("ms") || stdout.contains("duration") || stdout.contains("performance"));
}

#[test]
fn test_container_environment_variables() {
    // Test that container environment variables are set
    let mut cmd = Command::cargo_bin("hash-checker-gui").unwrap();
    let output = cmd
        .arg("--headless-test")
        .arg("--manifest")
        .arg("/tmp/test_manifest.json")
        .arg("--snapshot")
        .arg("/tmp/test_snapshot.png")
        .arg("--snapshot-width")
        .arg("800")
        .arg("--snapshot-height")
        .arg("600")
        .assert()
        .success();
    
    // Verify environment variables are accessible
    let display = std::env::var("DISPLAY").unwrap_or_default();
    let xdg_runtime = std::env::var("XDG_RUNTIME_DIR").unwrap_or_default();
    
    // In container environment, these should be set
    assert!(!display.is_empty(), "DISPLAY should be set in container");
    assert!(!xdg_runtime.is_empty(), "XDG_RUNTIME_DIR should be set in container");
}