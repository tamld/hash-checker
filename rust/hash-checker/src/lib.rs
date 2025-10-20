use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::Path,
};

mod batch;
mod manifest;

pub use batch::{run_batch, BatchEntry, BatchInput, BatchReport, BatchStatus, BatchSummary};
pub use manifest::{
    apply_entry_path, detect_format_from_extension, generate_manifest, read_manifest,
    relative_path_string, resolve_root, verify_manifest, write_manifest, Manifest, ManifestEntry,
    ManifestFormat, VerificationReport,
};

use blake2::{Blake2b512, Blake2s256};
use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};

const CHUNK_SIZE: usize = 1024 * 1024;

#[derive(thiserror::Error, Debug)]
pub enum HashError {
    #[error("unsupported algorithm '{0}'")]
    UnsupportedAlgorithm(String),
    #[error("expected hash cannot be empty")]
    EmptyExpectedHash,
    #[error("unable to infer algorithm from digest length {0}")]
    InferenceFailed(usize),
    #[error("expected hash contains non-hex characters")]
    InvalidExpectedHash,
    #[error("failed to canonicalize path '{path}': {source}")]
    Canonicalize {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("path '{0}' is not a regular file")]
    NotAFile(String),
    #[error("path '{0}' is not a directory")]
    NotADirectory(String),
    #[error("path '{0}' does not exist")]
    PathNotFound(String),
    #[error("unsupported manifest format '{0}'")]
    UnsupportedManifestFormat(String),
    #[error("failed to serialize manifest: {0}")]
    ManifestSerialize(String),
    #[error("failed to parse manifest: {0}")]
    ManifestParse(String),
    #[error("unsupported manifest version '{found}', expected {expected}")]
    ManifestVersion {
        expected: &'static str,
        found: String,
    },
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type HashResult<T> = std::result::Result<T, HashError>;

pub fn supported_algorithms() -> &'static [&'static str] {
    &[
        "md5", "sha1", "sha224", "sha256", "sha384", "sha512", "blake2s", "blake2b",
    ]
}

const ALGO_MD5: [&str; 1] = ["md5"];
const ALGO_SHA1: [&str; 1] = ["sha1"];
const ALGO_SHA224: [&str; 1] = ["sha224"];
const ALGO_SHA384: [&str; 1] = ["sha384"];
const ALGO_SHA256_FAMILY: [&str; 2] = ["sha256", "blake2s"];
const ALGO_SHA512_FAMILY: [&str; 2] = ["sha512", "blake2b"];
const ALGO_EMPTY: [&str; 0] = [];

fn candidate_algorithms_for_length(length: usize) -> &'static [&'static str] {
    match length {
        32 => &ALGO_MD5,
        40 => &ALGO_SHA1,
        56 => &ALGO_SHA224,
        64 => &ALGO_SHA256_FAMILY,
        96 => &ALGO_SHA384,
        128 => &ALGO_SHA512_FAMILY,
        _ => &ALGO_EMPTY,
    }
}

pub fn compute_hash(path: &Path, algorithm: &str) -> HashResult<String> {
    let normalized = algorithm.to_lowercase();
    match normalized.as_str() {
        "md5" => compute_with::<Md5>(path),
        "sha1" => compute_with::<Sha1>(path),
        "sha224" => compute_with::<Sha224>(path),
        "sha256" => compute_with::<Sha256>(path),
        "sha384" => compute_with::<Sha384>(path),
        "sha512" => compute_with::<Sha512>(path),
        "blake2s" => compute_with::<Blake2s256>(path),
        "blake2b" => compute_with::<Blake2b512>(path),
        other => Err(HashError::UnsupportedAlgorithm(other.to_string())),
    }
}

fn open_canonical_file(path: &Path) -> HashResult<File> {
    let canonical = path
        .canonicalize()
        .map_err(|source| HashError::Canonicalize {
            path: path.display().to_string(),
            source,
        })?;
    let metadata = fs::metadata(&canonical)?;
    if !metadata.is_file() {
        return Err(HashError::NotAFile(canonical.display().to_string()));
    }
    Ok(File::open(&canonical)?)
}

fn compute_with<D>(path: &Path) -> HashResult<String>
where
    D: Digest,
{
    let file = open_canonical_file(path)?;
    let mut reader = BufReader::new(file);
    let mut digest = D::new();
    let mut buffer = vec![0u8; CHUNK_SIZE];

    loop {
        let read = reader.read(&mut buffer[..])?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }

    Ok(hex::encode(digest.finalize()))
}

fn normalize_algorithm_name(name: &str) -> Option<&'static str> {
    match name.trim().to_ascii_lowercase().as_str() {
        "md5" => Some("md5"),
        "sha1" => Some("sha1"),
        "sha224" => Some("sha224"),
        "sha256" => Some("sha256"),
        "sha384" => Some("sha384"),
        "sha512" => Some("sha512"),
        "blake2s" => Some("blake2s"),
        "blake2b" => Some("blake2b"),
        _ => None,
    }
}

fn split_algorithm_prefix(expected_hash: &str) -> (Option<&str>, &str) {
    let trimmed = expected_hash.trim();
    if let Some(idx) = trimmed.find(':') {
        let (prefix, rest) = trimmed.split_at(idx);
        let digest = rest[1..].trim();
        (Some(prefix), digest)
    } else {
        (None, trimmed)
    }
}

pub fn detect_algorithm(expected_hash: &str) -> Option<&'static str> {
    let (maybe_prefix, digest_part) = split_algorithm_prefix(expected_hash);
    if let Some(prefix) = maybe_prefix {
        if let Some(normalized) = normalize_algorithm_name(prefix) {
            return Some(normalized);
        }
    }

    candidate_algorithms_for_length(digest_part.len())
        .first()
        .copied()
}

fn is_hex(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn verify_hash(
    path: &Path,
    expected_hash: &str,
    algorithm: Option<&str>,
) -> HashResult<(bool, String)> {
    let trimmed = expected_hash.trim();
    let (maybe_prefix, digest_part) = split_algorithm_prefix(trimmed);

    let digest = digest_part.to_ascii_lowercase();
    if digest.is_empty() {
        return Err(HashError::EmptyExpectedHash);
    }
    if !is_hex(&digest) {
        return Err(HashError::InvalidExpectedHash);
    }

    let provided_algorithm: Option<&'static str> = if let Some(name) = algorithm {
        if name.trim().is_empty() {
            None
        } else {
            Some(
                normalize_algorithm_name(name)
                    .ok_or_else(|| HashError::UnsupportedAlgorithm(name.trim().to_string()))?,
            )
        }
    } else {
        None
    };

    let prefix_algorithm: Option<&'static str> = match maybe_prefix {
        Some(prefix) if !prefix.trim().is_empty() => Some(
            normalize_algorithm_name(prefix)
                .ok_or_else(|| HashError::UnsupportedAlgorithm(prefix.trim().to_string()))?,
        ),
        _ => None,
    };

    let candidates: Vec<&'static str> = if let Some(name) = provided_algorithm {
        vec![name]
    } else if let Some(name) = prefix_algorithm {
        vec![name]
    } else {
        let inferred = candidate_algorithms_for_length(digest.len());
        if inferred.is_empty() {
            return Err(HashError::InferenceFailed(digest.len()));
        }
        inferred.to_vec()
    };

    let mut first_computed: Option<String> = None;
    for &candidate in &candidates {
        let computed = compute_hash(path, candidate)?;
        if computed == digest {
            return Ok((true, computed));
        }
        if first_computed.is_none() {
            first_computed = Some(computed);
        }
    }

    let computed = first_computed.unwrap_or_default();
    Ok((false, computed))
}
