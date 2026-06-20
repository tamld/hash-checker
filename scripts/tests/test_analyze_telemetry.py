from __future__ import annotations

from pathlib import Path

import analyze_telemetry as telemetry


def test_parse_telemetry_log_reads_events(telemetry_log_file: Path) -> None:
    events = telemetry.parse_telemetry_log(telemetry_log_file)

    assert len(events) == 4
    assert events[0]["type"] == "performance"
    assert events[0]["_line_number"] == 1


def test_parse_telemetry_log_warns_when_missing(tmp_path, capsys) -> None:
    missing_file = tmp_path / "missing.log"

    events = telemetry.parse_telemetry_log(missing_file)
    captured = capsys.readouterr().out

    assert events == []
    assert f"Warning: Log file {missing_file} not found" in captured


def test_parse_telemetry_log_skips_invalid_lines(tmp_path, capsys) -> None:
    malformed_file = tmp_path / "malformed.log"
    malformed_file.write_text('{"type": "performance"}\nnot-json\n', encoding="utf-8")

    events = telemetry.parse_telemetry_log(malformed_file)
    captured = capsys.readouterr().out

    assert len(events) == 1
    assert "Warning: Invalid JSON on line 2" in captured


def test_analyze_events_generates_summary(telemetry_events) -> None:
    analysis = telemetry.analyze_events(list(telemetry_events))

    summary = analysis["summary"]
    assert summary["total_events"] == 4
    assert summary["unique_event_types"] == 3
    assert summary["error_count"] == 1
    assert summary["state_transitions"] == 1
    assert summary["avg_performance_ms"] == 100
    assert analysis["event_types"]["performance"] == 2


def test_analyze_events_empty_returns_error() -> None:
    assert telemetry.analyze_events([]) == {"error": "No events found"}


def test_generate_report_contains_key_sections(telemetry_events) -> None:
    analysis = telemetry.analyze_events(list(telemetry_events))
    report = telemetry.generate_report(analysis)

    assert report.startswith("# Telemetry Analysis Report")
    assert "## Summary" in report
    assert "## Performance Events" in report
    assert "- **Count:** 2" in report
