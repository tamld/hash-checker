use std::f64;

use serde::Serialize;
use serde_json::{Map, Number, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonResult {
    Match,
    Diff(Vec<DiffEntry>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DiffEntry {
    pub path: String,
    pub expected: Value,
    pub actual: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComparisonMode {
    Exact,
    Structural,
    Fuzzy,
}

pub fn compare_exact(expected: &Value, actual: &Value) -> ComparisonResult {
    compare_internal(expected, actual, ComparisonMode::Exact)
}

pub fn compare_structural(expected: &Value, actual: &Value) -> ComparisonResult {
    let expected = normalize_value(expected);
    let actual = normalize_value(actual);
    compare_internal(&expected, &actual, ComparisonMode::Structural)
}

pub fn compare_fuzzy(expected: &Value, actual: &Value) -> ComparisonResult {
    let expected = normalize_value(expected);
    let actual = normalize_value(actual);
    compare_internal(&expected, &actual, ComparisonMode::Fuzzy)
}

fn compare_internal(expected: &Value, actual: &Value, mode: ComparisonMode) -> ComparisonResult {
    let mut diffs = Vec::new();
    compare_values("", expected, actual, &mut diffs, mode);
    if diffs.is_empty() {
        ComparisonResult::Match
    } else {
        ComparisonResult::Diff(diffs)
    }
}

fn compare_values(
    path: &str,
    expected: &Value,
    actual: &Value,
    diffs: &mut Vec<DiffEntry>,
    mode: ComparisonMode,
) {
    match (expected, actual) {
        (Value::Object(e), Value::Object(a)) => compare_objects(path, e, a, diffs, mode),
        (Value::Array(e), Value::Array(a)) => compare_arrays(path, e, a, diffs, mode),
        (Value::Number(e), Value::Number(a)) => {
            let mut within_tolerance = false;
            if matches!(mode, ComparisonMode::Fuzzy) {
                within_tolerance = numbers_within_tolerance(path, e, a);
            }
            if !within_tolerance && expected != actual {
                diffs.push(DiffEntry {
                    path: path.to_owned(),
                    expected: expected.clone(),
                    actual: actual.clone(),
                });
            }
        }
        _ => {
            if expected != actual {
                diffs.push(DiffEntry {
                    path: path.to_owned(),
                    expected: expected.clone(),
                    actual: actual.clone(),
                });
            }
        }
    }
}

fn compare_objects(
    path: &str,
    expected: &Map<String, Value>,
    actual: &Map<String, Value>,
    diffs: &mut Vec<DiffEntry>,
    mode: ComparisonMode,
) {
    for (key, e_value) in expected {
        let new_path = format!("{path}/{key}");
        match actual.get(key) {
            Some(a_value) => compare_values(&new_path, e_value, a_value, diffs, mode),
            None => diffs.push(DiffEntry {
                path: new_path,
                expected: e_value.clone(),
                actual: Value::Null,
            }),
        }
    }
    for (key, _) in actual {
        if !expected.contains_key(key) {
            let new_path = format!("{path}/{key}");
            diffs.push(DiffEntry {
                path: new_path,
                expected: Value::Null,
                actual: actual[key].clone(),
            });
        }
    }
}

fn compare_arrays(
    path: &str,
    expected: &[Value],
    actual: &[Value],
    diffs: &mut Vec<DiffEntry>,
    mode: ComparisonMode,
) {
    if expected.len() != actual.len() {
        diffs.push(DiffEntry {
            path: format!("{path}/length"),
            expected: Value::from(expected.len()),
            actual: Value::from(actual.len()),
        });
    }
    let len = expected.len().min(actual.len());
    for index in 0..len {
        let new_path = format!("{path}/{index}");
        compare_values(&new_path, &expected[index], &actual[index], diffs, mode);
    }
}

fn numbers_within_tolerance(path: &str, expected: &Number, actual: &Number) -> bool {
    let Some(expected) = expected.as_f64() else {
        return false;
    };
    let Some(actual) = actual.as_f64() else {
        return false;
    };
    let diff = (expected - actual).abs();
    if diff == 0.0 {
        return true;
    }

    let segments = path_segments(path);
    if segments.is_empty() {
        return false;
    }

    if is_dimension_field(&segments) {
        return diff <= 5.0;
    }

    if is_color_component(&segments) || is_percentage_field(&segments) {
        let baseline = expected.abs().max(actual.abs());
        let baseline = if baseline < f64::EPSILON {
            1.0
        } else {
            baseline
        };
        return diff <= baseline * 0.05 + f64::EPSILON;
    }

    false
}

fn path_segments(path: &str) -> Vec<&str> {
    path.split('/')
        .filter(|segment| !segment.is_empty())
        .collect()
}

fn is_dimension_field(segments: &[&str]) -> bool {
    const DIRECT_DIM_KEYS: &[&str] = &[
        "width", "height", "x", "y", "left", "top", "right", "bottom", "pos_x", "pos_y",
        "center_x", "center_y",
    ];
    if let Some(&last) = segments.last() {
        if DIRECT_DIM_KEYS
            .iter()
            .any(|key| last.eq_ignore_ascii_case(key))
        {
            return true;
        }
    }
    is_indexed_dimension(segments)
}

fn is_indexed_dimension(segments: &[&str]) -> bool {
    if segments.len() < 2 {
        return false;
    }
    if segments
        .last()
        .and_then(|segment| segment.parse::<usize>().ok())
        .is_none()
    {
        return false;
    }
    if let Some(parent) = segments.get(segments.len() - 2) {
        const DIM_ARRAY_KEYS: &[&str] = &["position", "pos", "size", "rect", "bounds"];
        return DIM_ARRAY_KEYS
            .iter()
            .any(|key| parent.eq_ignore_ascii_case(key));
    }
    false
}

fn is_color_component(segments: &[&str]) -> bool {
    const COLOR_COMPONENT_KEYS: &[&str] = &[
        "r",
        "g",
        "b",
        "a",
        "red",
        "green",
        "blue",
        "alpha",
        "h",
        "s",
        "l",
        "hue",
        "saturation",
        "lightness",
        "brightness",
    ];
    if let Some(&last) = segments.last() {
        if COLOR_COMPONENT_KEYS
            .iter()
            .any(|key| last.eq_ignore_ascii_case(key))
        {
            return true;
        }
    }
    if segments.iter().any(|segment| {
        let lower = segment.to_ascii_lowercase();
        lower.contains("color")
    }) {
        return true;
    }
    is_indexed_color(segments)
}

fn is_indexed_color(segments: &[&str]) -> bool {
    if segments.len() < 2 {
        return false;
    }
    if segments
        .last()
        .and_then(|segment| segment.parse::<usize>().ok())
        .is_none()
    {
        return false;
    }
    if let Some(parent) = segments.get(segments.len() - 2) {
        let lower = parent.to_ascii_lowercase();
        return lower.contains("color") || lower.contains("rgba");
    }
    false
}

fn is_percentage_field(segments: &[&str]) -> bool {
    segments.iter().any(|segment| {
        let lower = segment.to_ascii_lowercase();
        lower.contains("percent") || lower.contains("percentage") || lower.contains("ratio")
    })
}

fn normalize_value(value: &Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(normalize_object(map)),
        Value::Array(arr) => Value::Array(arr.iter().map(normalize_value).collect()),
        _ => value.clone(),
    }
}

fn normalize_object(map: &Map<String, Value>) -> Map<String, Value> {
    let mut out = Map::new();
    for (key, value) in map {
        let normalized = match key.as_str() {
            "captured_at" => Value::String("<normalized>".to_owned()),
            "cli_args" => normalize_cli_args(value),
            "git_commit" => Value::Null,
            "telemetry" => normalize_telemetry(value),
            _ => normalize_value(value),
        };
        out.insert(key.clone(), normalized);
    }
    out
}

fn normalize_cli_args(value: &Value) -> Value {
    let Some(arr) = value.as_array() else {
        return normalize_value(value);
    };
    let mut normalized = Vec::new();
    let mut skip_next = false;
    for entry in arr {
        let Some(arg) = entry.as_str() else {
            return normalize_value(value);
        };
        if skip_next {
            skip_next = false;
            continue;
        }
        if arg.starts_with("--capture-golden") || arg.starts_with("--compare-golden") {
            skip_next = true;
            continue;
        }
        if arg == "--compare-mode" {
            skip_next = true;
            continue;
        }
        normalized.push(Value::String(arg.to_owned()));
    }
    Value::Array(normalized)
}

fn normalize_telemetry(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut out = Map::new();
            for (key, val) in map {
                let normalized = if key == "scan_progress" {
                    Value::String("<normalized>".to_owned())
                } else {
                    normalize_value(val)
                };
                out.insert(key.clone(), normalized);
            }
            Value::Object(out)
        }
        _ => normalize_value(value),
    }
}
