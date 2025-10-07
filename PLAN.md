# Development Plan

1.  **Phase 1: Core Functionality (Initial Implementation)**
    *   [x] Create project structure (`src`, `tests`).
    *   [ ] Implement basic file hashing for a single file using SHA-256.
    *   [ ] Implement hash comparison logic.
    *   [ ] Create a command-line interface (CLI) to accept file paths and expected hashes.
    *   [ ] Write initial unit tests for the core hashing and comparison functions.

2.  **Phase 2: GUI Implementation**
    *   [x] Create a GUI using `tkinter`.
    *   [x] Add drag-and-drop support using `tkinterdnd2`.
    *   [x] Implement file selection, algorithm selection, and hash comparison.
    *   [x] Update documentation with GUI instructions.

3.  **Phase 3: Enhancements & Features**
    *   [ ] Add support for other hashing algorithms (MD5, SHA-1, SHA-512).
    *   [ ] Allow hashing of entire directories.
    *   [ ] Implement a feature to generate and save a hash file for a directory.
    *   [ ] Implement a feature to validate a directory against a hash file.
    *   [ ] Improve CLI with more options and better user feedback.

4.  **Phase 4: Packaging**
    *   [x] Document the build process using PyInstaller.

5.  **Phase 5: Refinement & Documentation**
    *   [ ] Refactor code for clarity and efficiency.
    *   [ ] Add comprehensive error handling.
    *   [ ] Populate all documentation files (`GOALS.md`, `TASKS.md`, `BACKLOG.md`).
