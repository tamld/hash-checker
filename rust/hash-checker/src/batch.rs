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
    use std::fs::File;
    use std::io::{self, Write};
    use tempfile::tempdir;

    fn create_report(errored: usize, mismatched: usize, missing: usize) -> BatchReport {
        BatchReport {
            summary: BatchSummary {
                total: 10,
                matched: 10usize.saturating_sub(errored + mismatched + missing),
                mismatched,
                missing,
                errored,
            },
            entries: vec![],
        }
    }

    #[test]
    fn test_batch_report_exit_code() {
        assert_eq!(create_report(0, 0, 0).exit_code(), 0);
        assert_eq!(create_report(0, 1, 0).exit_code(), 3);
        assert_eq!(create_report(0, 0, 1).exit_code(), 3);
        assert_eq!(create_report(0, 1, 1).exit_code(), 3);
        assert_eq!(create_report(1, 0, 0).exit_code(), 2);
        assert_eq!(create_report(1, 1, 0).exit_code(), 2);
        assert_eq!(create_report(1, 0, 1).exit_code(), 2);
    }

    #[test]
    fn test_run_batch() {
        let dir = tempdir().unwrap();

        let match_path = dir.path().join("match.txt");
        let mut f = File::create(&match_path).unwrap();
        f.write_all(b"hello").unwrap();
        let expected_match = crate::compute_hash(&match_path, "sha256").unwrap();

        let mismatch_path = dir.path().join("mismatch.txt");
        let mut f = File::create(&mismatch_path).unwrap();
        f.write_all(b"world").unwrap();
        let expected_mismatch = "a".repeat(64);

        let missing_path = dir.path().join("missing.txt");
        let expected_missing = "b".repeat(64);

        let error_path = dir.path().join("error.txt");
        let mut f = File::create(&error_path).unwrap();
        f.write_all(b"error").unwrap();
        let expected_error = "invalid-hash-string".to_string();

        let inputs = vec![
            BatchInput {
                path: match_path,
                expected: expected_match,
                algorithm: None,
            },
            BatchInput {
                path: mismatch_path,
                expected: expected_mismatch,
                algorithm: Some("SHA256".to_string()),
            },
            BatchInput {
                path: missing_path,
                expected: expected_missing,
                algorithm: None,
            },
            BatchInput {
                path: error_path,
                expected: expected_error,
                algorithm: None,
            },
        ];

        let report = run_batch(&inputs);
        assert_eq!(report.summary.total, 4);
        assert_eq!(report.summary.matched, 1);
        assert_eq!(report.summary.mismatched, 1);
        assert_eq!(report.summary.missing, 1);
        assert_eq!(report.summary.errored, 1);
        assert_eq!(report.exit_code(), 2);
        assert_eq!(report.entries[0].status, BatchStatus::Match);
        assert_eq!(report.entries[1].status, BatchStatus::Mismatch);
        assert_eq!(report.entries[2].status, BatchStatus::Missing);
        assert_eq!(report.entries[3].status, BatchStatus::Error);
    }

    #[test]
    fn test_map_error_variants() {
        let (status, _) = map_error(&HashError::PathNotFound("test.txt".to_string()));
        assert_eq!(status, BatchStatus::Missing);

        let (status, _) = map_error(&HashError::NotAFile("test.txt".to_string()));
        assert_eq!(status, BatchStatus::Error);

        let (status, _) = map_error(&HashError::UnsupportedAlgorithm("md4".to_string()));
        assert_eq!(status, BatchStatus::Error);

        let (status, _) = map_error(&HashError::InferenceFailed(16));
        assert_eq!(status, BatchStatus::Error);

        let (status, _) = map_error(&HashError::InvalidExpectedHash);
        assert_eq!(status, BatchStatus::Error);

        let (status, _) = map_error(&HashError::EmptyExpectedHash);
        assert_eq!(status, BatchStatus::Error);

        let (status, _) = map_error(&HashError::Canonicalize {
            path: "test.txt".to_string(),
            source: io::Error::new(io::ErrorKind::NotFound, "not found"),
        });
        assert_eq!(status, BatchStatus::Missing);

        let (status, _) = map_error(&HashError::Canonicalize {
            path: "test.txt".to_string(),
            source: io::Error::new(io::ErrorKind::PermissionDenied, "permission denied"),
        });
        assert_eq!(status, BatchStatus::Error);

        let (status, _) = map_error(&HashError::Io(io::Error::new(
            io::ErrorKind::NotFound,
            "not found",
        )));
        assert_eq!(status, BatchStatus::Missing);

        let (status, _) = map_error(&HashError::Io(io::Error::other("other io error")));
        assert_eq!(status, BatchStatus::Error);
    }
}
