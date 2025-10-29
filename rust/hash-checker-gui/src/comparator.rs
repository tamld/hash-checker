use serde::Serialize;
use serde_json::{Map, Value};

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

pub fn compare_exact(expected: &Value, actual: &Value) -> ComparisonResult {
    let mut diffs = Vec::new();
    compare_values("", expected, actual, &mut diffs);
    if diffs.is_empty() {
        ComparisonResult::Match
    } else {
        ComparisonResult::Diff(diffs)
    }
}

pub fn compare_structural(expected: &Value, actual: &Value) -> ComparisonResult {
    let expected = normalize_value(expected);
    let actual = normalize_value(actual);
    compare_exact(&expected, &actual)
}

fn compare_values(path: &str, expected: &Value, actual: &Value, diffs: &mut Vec<DiffEntry>) {
    match (expected, actual) {
        (Value::Object(e), Value::Object(a)) => compare_objects(path, e, a, diffs),
        (Value::Array(e), Value::Array(a)) => compare_arrays(path, e, a, diffs),
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
) {
    for (key, e_value) in expected {
        let new_path = format!("{}/{}", path, key);
        match actual.get(key) {
            Some(a_value) => compare_values(&new_path, e_value, a_value, diffs),
            None => diffs.push(DiffEntry {
                path: new_path,
                expected: e_value.clone(),
                actual: Value::Null,
            }),
        }
    }
    for (key, _) in actual {
        if !expected.contains_key(key) {
            let new_path = format!("{}/{}", path, key);
            diffs.push(DiffEntry {
                path: new_path,
                expected: Value::Null,
                actual: actual[key].clone(),
            });
        }
    }
}

fn compare_arrays(path: &str, expected: &[Value], actual: &[Value], diffs: &mut Vec<DiffEntry>) {
    if expected.len() != actual.len() {
        diffs.push(DiffEntry {
            path: format!("{}/length", path),
            expected: Value::from(expected.len()),
            actual: Value::from(actual.len()),
        });
    }
    let len = expected.len().min(actual.len());
    for index in 0..len {
        let new_path = format!("{}/{}", path, index);
        compare_values(&new_path, &expected[index], &actual[index], diffs);
    }
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
    match value.as_array() {
        Some(arr) => Value::Array(
            arr.iter()
                .filter_map(|v| v.as_str())
                .filter(|arg| {
                    !(arg.starts_with("--capture-golden") || arg.starts_with("--compare-golden"))
                })
                .map(|s| Value::String(s.to_owned()))
                .collect(),
        ),
        None => normalize_value(value),
    }
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
