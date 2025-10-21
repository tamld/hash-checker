# GTK4 Migration Plan

## Overview
- **Goal**: transition the Hash Checker GUI off legacy GTK3 dependencies and ship a GTK4-capable experience while preserving current egui functionality.
- **Driver**: issue #23 spike (closed) highlighted follow-up work after adopting the XDG portal backend and MSRV 1.88.
- **Current baseline**:
  - GUI runs on `eframe` 0.33 (`winit` 0.30, `egui_glow`) with the Rusty File Dialog (`rfd`) crate pinned to 0.15.4 (portal backend).
  - Linux builds require `xdg-desktop-portal`; GTK3 libs are no longer part of the dependency graph.
  - CI coverage: Linux job installs `xdg-desktop-portal` and exercises the GUI smoke test (`cargo run --release -- --smoke-test`).

## Assumptions
- GTK4 support is not yet shipped in `rfd` 0.15.4; upstream work is tracked but unavailable by default. Plan includes evaluating upstream status before committing to implementation.
- `eframe` 0.33 continues to provide a GL-based Linux backend (`egui_glow`). GTK4 integration will likely rely on embedding the egui surface into a GTK4 window or switching to an alternative windowing stack (`wgpu` + `tao`).
- Packaging targets (AppImage, `.deb`, NSIS, DMG) must continue to build without host GTK runtime dependencies.
- We maintain MSRV 1.88 introduced with the egui upgrade; no additional toolchain bumps are expected during this phase.

## Constraints & Dependencies
- `gtk4` crates (`gtk4`, `gio`, `glib`, `gtk4-macros`) require libgtk-4 runtime packages on Linux; our CI images and documentation must cover installation.
- Wayland/X11 support must remain via `winit`; we cannot regress support for headless CI or Windows/macOS builds.
- Documentation guardrail: all human-facing updates belong in Markdown within `docs/`.

## Proposed Phases

### Phase 1 – Research & Prototype (1–2 weeks)
1. Track upstream GTK4 progress in `rfd`:
   - Inspect `rfd` master branch / issues for GTK4 feature flag.
   - If unavailable, scope alternative file dialog strategies (e.g. `gtk4::FileDialog`, platform-specific wrappers).
2. Prototype embeddings:
   - Option A: Embed egui via `eframe::run_native` with a custom GTK4 window wrapper (using `gtk4` crate with `GLArea`).
   - Option B: Evaluate `eframe`'s experimental `wgpu` backend + `tao` windowing for GTK4-style widgets, keeping dialogs via GTK4 or portal.
3. Document findings (APIs, blockers, performance implications) in a spike log under `logs/gtk4/`.

### Phase 2 – Implementation Branch (2–3 weeks)
1. Create feature branch `feature/gtk4-migration`.
2. Update Cargo manifests:
   - Add `gtk4` dependencies with feature gating (`gtk4_libadwaita` vs `wgpu_tao_stack`).
   - Feature flag to toggle between portal-only and GTK4 modes (`gtk4-native`).
3. Refactor GUI entry point:
   - Provide GTK4 window initialization wrapper.
   - Route egui rendering through `gtk4::GLArea` or alternative backend.
4. Replace file dialog integration:
   - If `rfd` adds GTK4 support, enable the new feature.
   - Otherwise, build custom GTK4 dialog implementation with parity tests.
5. Update build scripts / CI:
   - Extend Linux Docker images and GitHub Actions to install `libgtk-4-dev`, `gtk4`, `libadwaita-1-dev`.
   - Ensure macOS/Windows builds conditionally skip GTK packages.

### Phase 3 – Validation & Packaging (1–2 weeks)
1. Automated checks:
   - `make ci-linux-local` (Docker) – covers fmt, clippy, tests.
   - `cargo run --release -- --smoke-test` on Linux host with GTK4 packages.
   - Windows/macOS smoke via existing CI jobs (ensure build still succeeds without GTK4 libs).
2. Manual verification:
   - Run GTK4 build inside Vagrant/Linux host, capture screenshots of file dialog and main flows (stored in `logs/qa/gtk4-<date>.md`).
   - Packaging rehearsal: AppImage, `.deb`, DMG, NSIS (ensure no extra runtime dependencies are required).
3. Documentation updates:
   - README + `docs/OPERATIONS.md` for GTK4 runtime steps.
   - `docs/DEPENDENCY_MIGRATION.md` to record milestone completion.
   - Release notes (CHANGELOG entry).

### Phase 4 – Rollout & Cleanup (1 week)
1. Final PR merge with issue references (`Fixes #34`, `Fixes #35`, follow-up issues for GTK4 tasks).
2. Remove feature flag if GTK4 becomes default; retain portal path as fallback if needed.
3. Schedule regression audit 30 days post-merge to confirm no regressions in telemetry or packaging.

## Testing Matrix
| Platform | Commands | Notes |
|----------|----------|-------|
| Linux (Wayland & X11) | `make ci-linux-local`, `cargo run --release -- --smoke-test`, GTK4 manual smoke | Requires `libgtk-4-dev`, `libadwaita-1-dev`, `xdg-desktop-portal` |
| macOS | `cargo run --release -- --smoke-test`, packaging via `cargo packager` | Ensure GTK libs not pulled in |
| Windows | `cargo run --release -- --smoke-test` (PowerShell) | File dialogs should use Win32 (no GTK) |
| Packaging CI | `workflow_dispatch` with `run_packaging=true` | Attach logs to release notes |

## Risks & Mitigations
- **GTK4 support in dependencies is incomplete** → Maintain feature flag fallback to portal-only mode, contribute upstream PRs if feasible.
- **CI runtime bloat** → Use conditional installation scripts; cache GTK4 packages in Docker images.
- **Performance regressions** → Benchmark with existing criterion suites; capture GUI frame timings before/after.
- **Accessibility regressions** → Re-run manifest layout tests and keyboard navigation checks.

## Deliverables
- Prototype report (`logs/gtk4/<date>-prototype.md`).
- Implementation PR with code changes, CI updates, and documentation.
- CHANGELOG + release note entries describing GTK4 switch and runtime requirements.
- Updated `.agents/project_state.yml` reflecting completion and new maintenance cadence.

## Next Steps
1. Open tracking issue(s) for each phase (prototype, implementation, validation).
2. Kick off Phase 1 research on a dedicated branch (e.g., `spike/gtk4-prototype`) and schedule a review with stakeholders.
