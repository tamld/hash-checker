"""Hash Checker package exposing hashing utilities and CLI helpers."""
from .core.hashing import (
    compute_hash,
    detect_algorithm,
    get_supported_algorithms,
    verify_hash,
)

__all__ = [
    "compute_hash",
    "detect_algorithm",
    "get_supported_algorithms",
    "verify_hash",
]
