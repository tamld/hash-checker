# GUI Framework Evaluation

## Requirements
- Cross-platform support (Windows, macOS, Linux) with native feel.
- Minimal runtime dependencies and ability to build via container/VM (no host pollution).
- Bundled executable support for future releases.
- Easy integration with existing Rust core crate.
- Support for drag-and-drop and accessibility (keyboard navigation, contrast).
- Reasonable community support and maintenance.

## Candidates

### Tauri
- **Pros:** Excellent packaging story (signed installers), uses web technologies (HTML/CSS/JS), lightweight runtime compared to Electron, strong community.
- **Cons:** Requires Node.js toolchain and frontend build steps; harder to run entirely in headless container; GUI code split between Rust backend + JS frontend; more setup for automated tests.

### egui / eframe
- **Pros:** Pure Rust, no Node dependency, works well in headless testing via winit mock; good support for drag-and-drop, theming; simple integration with Rust core via shared crate.
- **Cons:** Custom look-and-feel (not native); packaging requires extra tooling (e.g., winit + `cargo-bundle`).

### Fyne (Go) / Flutter
- Discarded: different language + heavier dependencies; does not align with Rust migration goals.

## Decision
Proceed with **egui/eframe** for the MVP GUI.

### Rationale
- Keeps the stack purely Rust, aligning with the migration strategy.
- Simplifies CI/CD in sandboxed environments (only Rust toolchain required).
- Facilitates rapid prototyping and accessible UI components.
- Later packaging can use `cargo-bundle` or integrate with `cargo-dist`.

### Next Steps
1. Scaffold new crate `rust/hash-checker-gui` depending on `hash-checker` core.
2. Implement MVP screen (file picker, algorithm dropdown, hash output).
3. Add GUI smoke test harness (launch via Vagrant headless once UI stabilized).
4. Document build instructions and update automation scripts.

Review this decision quarterly or when requirements change (e.g., need for advanced native integration or web-based UI).
