# Hash Checker

Cross-platform utility for verifying file integrity via cryptographic hashes. The
project ships both a command-line interface and a Tkinter-based GUI with drag-and-drop support.

## Features
- Supports common hashing algorithms (MD5, SHA-1, SHA-224, SHA-256, SHA-384, SHA-512, BLAKE2b, BLAKE2s).
- Detects the correct algorithm automatically from digest length when possible.
- Provides both CLI and GUI flows in a single package.
- Works on Windows, macOS, and Linux.

## Requirements
- Python 3.8+
- Optional GUI dependency: [`tkinterdnd2`](https://pypi.org/project/tkinterdnd2/) (install automatically on most platforms via pip).
- Tk/Tcl support for the GUI (preinstalled on macOS/Windows; on Linux install `python3-tk`).

## Installation
```bash
pip install .
```
For development installs, use `pip install -e .` to enable editable mode.

## Usage
### CLI
Compute and verify a file hash:
```bash
python -m hash_checker /path/to/file.ext 19fe5f3e518ba46537ddf4bcd098d66e2873fda2dccf58e66f6ab1f932c6d811
```

Specify an exact algorithm:
```bash
hash-checker /path/to/file.ext 109788a70f52a60437d3c8867124ca72 --algorithm md5
```

List available algorithms:
```bash
hash-checker --list-algorithms
```

Exit codes follow this convention:
- `0`: Hashes match / informational output.
- `1`: Verification failed due to runtime error.
- `2`: Missing CLI arguments.
- `3`: Hash mismatch.
- `5`: GUI launch unavailable (Tk missing).

### GUI
Launch the graphical interface:
```bash
hash-checker --gui
```

Inside the window you can drag-and-drop a file, choose an algorithm (or keep `auto` for detection), provide an expected hash, and verify the result.

## Packaging (PyInstaller)
Run inside the project root on the target operating system:
```bash
pip install -r requirements-build.txt
pyinstaller --name HashChecker --onefile src/hash_checker/__main__.py
```
The generated executable appears under `dist/`. Repeat on Windows, macOS, and Linux to ship native binaries. For reliable builds, execute PyInstaller inside disposable virtual machines or containers to avoid polluting the host system.

## Testing
```bash
python -m unittest discover tests
```

## Project Documents
- `PLAN.md` – development roadmap.
- `TASKS.md` – granular work items.
- `BACKLOG.md` – future enhancements.
- `GOALS.md` – project objectives.
- `.agent/AGENTS.md` – operational guidelines for assistants.
