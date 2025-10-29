from __future__ import annotations

from pathlib import Path

import pytest

import check_performance_regression as regression


def test_load_metrics_reads_json(current_metrics_file: Path) -> None:
    metrics = regression.load_metrics(current_metrics_file)

    assert "scan_duration_ms" in metrics
    assert metrics["memory_usage_mb"] == 512


def test_load_metrics_missing_file_raises(tmp_path: Path) -> None:
    missing = tmp_path / "nope.json"

    with pytest.raises(FileNotFoundError):
        regression.load_metrics(missing)


def test_compare_metrics_classifies_changes() -> None:
    current = {
        "latency_ms": 132,
        "throughput_ops": 80,
        "new_metric": 5,
    }
    baseline = {
        "latency_ms": 100,
        "throughput_ops": 100,
    }

    results = regression.compare_metrics(current, baseline, tolerance=0.1)
    summary = results["summary"]

    assert summary["total_metrics"] == 2
    assert summary["regressions"] == 1
    assert summary["improvements"] == 1
    assert summary["within_tolerance"] == 0

    statuses = {c["metric"]: c["status"] for c in results["comparisons"]}
    assert statuses["latency_ms"] == "regression"
    assert statuses["throughput_ops"] == "improvement"

    new_metric = next(c for c in results["comparisons"] if c["metric"] == "new_metric")
    assert new_metric["status"] == "new"
    assert new_metric["baseline"] is None


def test_generate_markdown_report_summarizes_results() -> None:
    current = {"latency_ms": 120, "throughput_ops": 90}
    baseline = {"latency_ms": 100, "throughput_ops": 110}

    results = regression.compare_metrics(current, baseline, tolerance=0.05)
    report = regression.generate_markdown_report(results)

    assert report.startswith("# Performance Regression Report")
    assert "Regressions" in report
    assert "| latency_ms | 120 | 100 | 20.0% |" in report
    assert "| throughput_ops | 90 | 110 | -18.18% |" in report
