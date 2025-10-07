"""Hashing primitives shared across the CLI and GUI applications."""
from __future__ import annotations

import hashlib
from pathlib import Path
from typing import Dict, Iterable, List, Optional, Sequence, Tuple

CHUNK_SIZE = 1024 * 1024  # 1 MiB
_DEFAULT_ALGORITHMS: Sequence[str] = (
    "md5",
    "sha1",
    "sha224",
    "sha256",
    "sha384",
    "sha512",
    "blake2s",
    "blake2b",
)
_HASH_LENGTH_HINTS: Dict[int, Sequence[str]] = {
    32: ("md5",),
    40: ("sha1",),
    56: ("sha224",),
    64: ("sha256", "blake2s"),
    96: ("sha384",),
    128: ("sha512", "blake2b"),
}


def get_supported_algorithms() -> List[str]:
    """Return a sorted preference-ordered list of available algorithms."""
    available = {name.lower() for name in hashlib.algorithms_available}
    preferred = [algo for algo in _DEFAULT_ALGORITHMS if algo in available]
    extras = sorted(available - set(preferred))
    return preferred + extras


def compute_hash(file_path: Path, algorithm: str) -> str:
    """Compute a hexadecimal digest for ``file_path`` using ``algorithm``."""
    normalized = algorithm.lower()
    if normalized not in hashlib.algorithms_available:
        raise ValueError(f"Algorithm '{algorithm}' is not supported on this system")

    hasher = hashlib.new(normalized)
    with file_path.expanduser().open("rb") as handle:
        while True:
            chunk = handle.read(CHUNK_SIZE)
            if not chunk:
                break
            hasher.update(chunk)
    return hasher.hexdigest()


def detect_algorithm(expected_hash: str) -> Optional[str]:
    """Best-effort inference of hash algorithm based on digest length."""
    if not expected_hash:
        return None
    digest = expected_hash.lower().strip()
    if any(char not in "0123456789abcdef" for char in digest):
        return None
    hints = _HASH_LENGTH_HINTS.get(len(digest))
    if not hints:
        return None
    if len(hints) == 1:
        return hints[0]
    for candidate in hints:
        if candidate in hashlib.algorithms_available:
            return candidate
    return None


def verify_hash(
    file_path: Path,
    expected_hash: str,
    algorithm: Optional[str] = None,
) -> Tuple[bool, str]:
    """Validate a file against an expected digest.

    Returns ``(matches, computed_digest)``.
    """
    path = file_path.expanduser()
    if not path.is_file():
        raise FileNotFoundError(f"File not found: {path}")

    digest = expected_hash.strip().lower()
    if not digest:
        raise ValueError("Expected hash cannot be empty")

    algo = algorithm or detect_algorithm(digest)
    if not algo:
        raise ValueError("Unable to infer hash algorithm. Provide --algorithm explicitly.")

    computed = compute_hash(path, algo)
    return computed == digest, computed
