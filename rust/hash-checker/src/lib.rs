use std::{fs::File, io::{BufReader, Read}, path::Path};

use anyhow::Result;
use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use blake2::{Blake2b512, Blake2s256};

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
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type HashResult<T> = std::result::Result<T, HashError>;

pub fn supported_algorithms() -> &'static [&'static str] {
    &[
        "md5",
        "sha1",
        "sha224",
        "sha256",
        "sha384",
        "sha512",
        "blake2s",
        "blake2b",
    ]
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

fn compute_with<D>(path: &Path) -> HashResult<String>
where
    D: Digest,
{
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut digest = D::new();
    let mut buffer = [0u8; CHUNK_SIZE];

    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }

    Ok(hex::encode(digest.finalize()))
}

pub fn detect_algorithm(expected_hash: &str) -> Option<&'static str> {
    let digest = expected_hash.trim();
    let length = digest.len();
    match length {
        32 => Some("md5"),
        40 => Some("sha1"),
        56 => Some("sha224"),
        64 => Some("sha256"),
        96 => Some("sha384"),
        128 => Some("sha512"),
        _ => None,
    }
}

fn is_hex(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn verify_hash(path: &Path, expected_hash: &str, algorithm: Option<&str>) -> HashResult<(bool, String)> {
    let digest = expected_hash.trim().to_lowercase();
    if digest.is_empty() {
        return Err(HashError::EmptyExpectedHash);
    }
    if !is_hex(&digest) {
        return Err(HashError::InvalidExpectedHash);
    }

    let algo = match algorithm {
        Some(name) => Some(name.to_string()),
        None => detect_algorithm(&digest).map(|s| s.to_string()),
    };

    let algo = match algo {
        Some(name) => name,
        None => return Err(HashError::InferenceFailed(digest.len())),
    };

    let computed = compute_hash(path, &algo)?;
    Ok((computed == digest, computed))
}
