mod comparator {
    include!("../src/comparator.rs");
}

use comparator::{compare_exact, compare_structural, ComparisonResult};
use serde_json::json;

#[test]
fn exact_match_returns_match() {
    let expected = json!({"a": 1, "b": [1, 2]});
    let actual = json!({"a": 1, "b": [1, 2]});
    assert!(matches!(
        compare_exact(&expected, &actual),
        ComparisonResult::Match
    ));
}

#[test]
fn exact_mismatch_detected() {
    let expected = json!({"a": 1, "b": [1, 2]});
    let actual = json!({"a": 2, "b": [1]});
    let diffs = match compare_exact(&expected, &actual) {
        ComparisonResult::Diff(diffs) => diffs,
        _ => panic!("Expected diff"),
    };
    assert!(diffs.iter().any(|d| d.path.ends_with("/a")));
    assert!(diffs.iter().any(|d| d.path.ends_with("/b/length")));
}

#[test]
fn structural_comparison_ignores_timestamp_and_golden_flag() {
    let expected = json!({
        "version": "1.0.0",
        "platform": "macos",
        "captures": [{
            "captured_at": "2025-10-28T12:00:00Z",
            "scenario": "minimal-scan",
            "window": {"width": 1280, "height": 800, "theme": "Slate"},
            "navigation": {"active_tab": "File Hash", "breadcrumb": []},
            "widgets": [],
            "telemetry": {"scan_progress": 1.0, "hash_mismatches": 0, "elapsed_ms": 0},
            "metadata": {"app_version": "0.1.5", "git_commit": null, "cli_args": ["--capture-golden", "minimal-scan"]}
        }]
    });
    let actual = json!({
        "version": "1.0.0",
        "platform": "macos",
        "captures": [{
            "captured_at": "2025-10-29T01:02:03Z",
            "scenario": "minimal-scan",
            "window": {"width": 1280, "height": 800, "theme": "Slate"},
            "navigation": {"active_tab": "File Hash", "breadcrumb": []},
            "widgets": [],
            "telemetry": {"scan_progress": 1.0, "hash_mismatches": 0, "elapsed_ms": 0},
            "metadata": {"app_version": "0.1.5", "git_commit": "abc123", "cli_args": ["--capture-golden", "minimal-scan"]}
        }]
    });

    assert!(matches!(
        compare_structural(&expected, &actual),
        ComparisonResult::Match
    ));
}
