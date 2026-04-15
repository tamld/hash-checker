use crate::{detect_algorithm, verify_hash, HashError};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct BatchInput {
    pub path: PathBuf,
    pub expected: String,
    #[serde(default)]
    pub algorithm: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BatchSummary {
    pub total: usize,
    pub matched: usize,
    pub mismatched: usize,
    pub missing: usize,
    pub errored: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct BatchEntry {
    pub path: String,
    pub status: BatchStatus,
    pub expected: String,
    pub actual: Option<String>,
    pub algorithm: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BatchStatus {
    Match,
    Mismatch,
    Missing,
    Error,
}

#[derive(Debug, Clone, Serialize)]
pub struct BatchReport {
    pub summary: BatchSummary,
    pub entries: Vec<BatchEntry>,
}

impl BatchReport {
    pub fn exit_code(&self) -> i32 {
        if self.summary.errored > 0 {
            2
        } else if self.summary.mismatched > 0 || self.summary.missing > 0 {
            3
        } else {
            0
        }
    }
}

pub fn run_batch(inputs: &[BatchInput]) -> BatchReport {
    let mut entries = Vec::with_capacity(inputs.len());
    let mut matched = 0usize;
    let mut mismatched = 0usize;
    let mut missing = 0usize;
    let mut errored = 0usize;

    for input in inputs {
        let algorithm_hint = algorithm_hint(input);
        let path = input.path.clone();
        let path_display = path.display().to_string();
        let expected = input.expected.trim().to_string();

        match verify_file(&path, &expected, input.algorithm.as_deref()) {
            Ok((BatchStatus::Match, actual)) => {
                matched += 1;
                entries.push(BatchEntry {
                    path: path_display,
                    status: BatchStatus::Match,
                    expected: expected.clone(),
                    actual: Some(actual),
                    algorithm: algorithm_hint,
                    error: None,
                });
            }
            Ok((BatchStatus::Mismatch, actual)) => {
                mismatched += 1;
                entries.push(BatchEntry {
                    path: path_display,
                    status: BatchStatus::Mismatch,
                    expected: expected.clone(),
                    actual: Some(actual),
                    algorithm: algorithm_hint,
                    error: None,
                });
            }
            Ok((BatchStatus::Missing, _)) | Ok((BatchStatus::Error, _)) => {
                unreachable!("verify_file does not return missing/error statuses");
            }
            Err(err) => {
                let (status, err_msg) = map_error(&err);
                match status {
                    BatchStatus::Missing => missing += 1,
                    BatchStatus::Error => errored += 1,
                    _ => {}
                }
                entries.push(BatchEntry {
                    path: path_display,
                    status,
                    expected,
                    actual: None,
                    algorithm: algorithm_hint,
                    error: Some(err_msg),
                });
            }
        }
    }

    let summary = BatchSummary {
        total: inputs.len(),
        matched,
        mismatched,
        missing,
        errored,
    };

    BatchReport { summary, entries }
}

fn algorithm_hint(input: &BatchInput) -> Option<String> {
    if let Some(alg) = input
        .algorithm
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        return Some(alg.to_ascii_lowercase());
    }
    detect_algorithm(&input.expected).map(|alg| alg.to_string())
}

fn verify_file(
    path: &Path,
    expected: &str,
    algorithm: Option<&str>,
) -> Result<(BatchStatus, String), HashError> {
    match verify_hash(path, expected, algorithm) {
        Ok((true, computed)) => Ok((BatchStatus::Match, computed)),
        Ok((false, computed)) => Ok((BatchStatus::Mismatch, computed)),
        Err(err) => Err(err),
    }
}

fn map_error(err: &HashError) -> (BatchStatus, String) {
    match err {
        HashError::PathNotFound(_) => (BatchStatus::Missing, err.to_string()),
        HashError::NotAFile(_) => (BatchStatus::Error, err.to_string()),
        HashError::UnsupportedAlgorithm(_) => (BatchStatus::Error, err.to_string()),
        HashError::InferenceFailed(_) => (BatchStatus::Error, err.to_string()),
        HashError::InvalidExpectedHash => (BatchStatus::Error, err.to_string()),
        HashError::EmptyExpectedHash => (BatchStatus::Error, err.to_string()),
        HashError::Canonicalize { source, .. } => {
            if source.kind() == std::io::ErrorKind::NotFound {
                (BatchStatus::Missing, err.to_string())
            } else {
                (BatchStatus::Error, err.to_string())
            }
        }
        HashError::Io(io_err) => {
            if io_err.kind() == std::io::ErrorKind::NotFound {
                (BatchStatus::Missing, err.to_string())
            } else {
                (BatchStatus::Error, err.to_string())
            }
        }
        _ => (BatchStatus::Error, err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_map_error_path_not_found() {
        let err = HashError::PathNotFound("test.txt".to_string());
        let (status, msg) = map_error(&err);
        assert_eq!(status, BatchStatus::Missing);
        assert_eq!(msg, "path 'test.txt' does not exist");
    }

    #[test]
    fn test_map_error_not_a_file() {
        let err = HashError::NotAFile("test.txt".to_string());
        let (status, msg) = map_error(&err);
        assert_eq!(status, BatchStatus::Error);
        assert_eq!(msg, "path 'test.txt' is not a regular file");
    }

    #[test]
    fn test_map_error_unsupported_algorithm() {
        let err = HashError::UnsupportedAlgorithm("md4".to_string());
        let (status, msg) = map_error(&err);
        assert_eq!(status, BatchStatus::Error);
        assert_eq!(msg, "unsupported algorithm 'md4'");
    }

    #[test]
    fn test_map_error_inference_failed() {
        let err = HashError::InferenceFailed(16);
        let (status, msg) = map_error(&err);
        assert_eq!(status, BatchStatus::Error);
        assert_eq!(msg, "unable to infer algorithm from digest length 16");
    }

    #[test]
    fn test_map_error_invalid_expected_hash() {
        let err = HashError::InvalidExpectedHash;
        let (status, msg) = map_error(&err);
        assert_eq!(status, BatchStatus::Error);
        assert_eq!(msg, "expected hash contains non-hex characters");
    }

    #[test]
    fn test_map_error_empty_expected_hash() {
        let err = HashError::EmptyExpectedHash;
        let (status, msg) = map_error(&err);
        assert_eq!(status, BatchStatus::Error);
        assert_eq!(msg, "expected hash cannot be empty");
    }

    #[test]
    fn test_map_error_canonicalize_not_found() {
        let err = HashError::Canonicalize {
            path: "test.txt".to_string(),
            source: io::Error::new(io::ErrorKind::NotFound, "not found"),
        };
        let (status, msg) = map_error(&err);
        assert_eq!(status, BatchStatus::Missing);
        assert_eq!(msg, "failed to canonicalize path 'test.txt': not found");
    }

    #[test]
    fn test_map_error_canonicalize_other_error() {
        let err = HashError::Canonicalize {
            path: "test.txt".to_string(),
            source: io::Error::new(io::ErrorKind::PermissionDenied, "permission denied"),
        };
        let (status, msg) = map_error(&err);
        assert_eq!(status, BatchStatus::Error);
        assert_eq!(msg, "failed to canonicalize path 'test.txt': permission denied");
    }

    #[test]
    fn test_map_error_io_not_found() {
        let err = HashError::Io(io::Error::new(io::ErrorKind::NotFound, "not found"));
        let (status, msg) = map_error(&err);
        assert_eq!(status, BatchStatus::Missing);
        assert_eq!(msg, "not found");
    }

    #[test]
    fn test_map_error_io_other_error() {
        let err = HashError::Io(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "permission denied",
        ));
        let (status, msg) = map_error(&err);
        assert_eq!(status, BatchStatus::Error);
        assert_eq!(msg, "permission denied");
    }

    #[test]
    fn test_map_error_fallback() {
        let err = HashError::NotADirectory("test.txt".to_string());
        let (status, msg) = map_error(&err);
        assert_eq!(status, BatchStatus::Error);
        assert_eq!(msg, "path 'test.txt' is not a directory");
    }
}
