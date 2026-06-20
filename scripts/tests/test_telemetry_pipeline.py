from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parents[2]
SCRIPTS_DIR = PROJECT_ROOT / "scripts"


def test_end_to_end_telemetry_pipeline(telemetry_log_file: Path, tmp_path: Path) -> None:
    analysis_report = tmp_path / "telemetry_report.md"

    analyze_cmd = [
        sys.executable,
        str(SCRIPTS_DIR / "analyze_telemetry.py"),
        str(telemetry_log_file),
        str(analysis_report),
    ]
    analyze_result = subprocess.run(
        analyze_cmd,
        capture_output=True,
        text=True,
        check=True,
    )

    assert analysis_report.exists()
    assert "Analyzing telemetry log" in analyze_result.stdout

    current_file = tmp_path / "current_metrics.json"
    baseline_file = tmp_path / "baseline_metrics.json"
    performance_report = tmp_path / "performance_report.md"

    current_file.write_text(
        json.dumps(
            {
                "scan_duration_ms": 250,
                "memory_usage_mb": 600,
            }
        ),
        encoding="utf-8",
    )
    baseline_file.write_text(
        json.dumps(
            {
                "scan_duration_ms": 200,
                "memory_usage_mb": 550,
            }
        ),
        encoding="utf-8",
    )

    regression_cmd = [
        sys.executable,
        str(SCRIPTS_DIR / "check_performance_regression.py"),
        str(current_file),
        str(baseline_file),
        "--tolerance",
        "0.05",
        "--output",
        str(performance_report),
    ]
    regression_result = subprocess.run(
        regression_cmd,
        capture_output=True,
        text=True,
        check=False,
    )

    assert performance_report.exists()
    assert regression_result.returncode == 1
    assert "Performance regression report written to" in regression_result.stdout
    assert "regressions detected" in regression_result.stdout
