"""Command-line interface for the Hash Checker project."""
from __future__ import annotations

import argparse
import sys
from pathlib import Path
from typing import Optional, Sequence

from ..core import get_supported_algorithms, verify_hash
from ..gui.app import launch_gui


def parse_args(argv: Optional[Sequence[str]] = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        prog="hash-checker",
        description="Verify file integrity with cryptographic hashes.",
    )
    parser.add_argument("file", nargs="?", help="Path to the file to verify")
    parser.add_argument(
        "expected_hash",
        nargs="?",
        help="Expected digest value (hex-encoded)",
    )
    parser.add_argument(
        "-a",
        "--algorithm",
        choices=sorted(get_supported_algorithms()),
        help="Hash algorithm to use (auto-detected by default)",
    )
    parser.add_argument(
        "--list-algorithms",
        action="store_true",
        help="List available algorithms and exit",
    )
    parser.add_argument(
        "--gui",
        action="store_true",
        help="Launch the graphical interface",
    )
    parser.add_argument(
        "--no-cli",
        action="store_true",
        help="Force GUI even if CLI arguments are provided",
    )
    return parser.parse_args(argv)


def run_cli(namespace: argparse.Namespace) -> int:
    if not namespace.file or not namespace.expected_hash:
        print("Error: provide both <file> and <expected_hash> for CLI mode", file=sys.stderr)
        return 2
    file_path = Path(namespace.file)
    try:
        matches, computed = verify_hash(file_path, namespace.expected_hash, namespace.algorithm)
    except Exception as exc:  # pylint: disable=broad-except
        print(f"Verification failed: {exc}", file=sys.stderr)
        return 1
    if matches:
        print("Hashes match ✅")
        return 0
    print("Hashes do not match ❌", file=sys.stderr)
    print(f"Computed: {computed}", file=sys.stderr)
    return 3


def main(argv: Optional[Sequence[str]] = None) -> int:
    namespace = parse_args(argv)

    if namespace.list_algorithms:
        print("Available algorithms:")
        for algo in get_supported_algorithms():
            print(f"- {algo}")
        return 0

    should_launch_gui = namespace.gui or namespace.no_cli or not (
        namespace.file and namespace.expected_hash
    )

    if should_launch_gui:
        code = launch_gui(get_supported_algorithms())
        if code != 0 or namespace.gui or namespace.no_cli:
            return code
        # Fall back to CLI if the user closed GUI immediately and CLI args exist
    return run_cli(namespace)


if __name__ == "__main__":  # pragma: no cover
    raise SystemExit(main())
