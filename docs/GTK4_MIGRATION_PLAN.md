# GTK4 Migration Plan

> Last reviewed: 2025-10-28 (see `logs/gtk4/20251022-research.md` for the
> supporting investigation log translated the same day).

## Overview
- **Goal**: add an optional GTK4-native mode for the Hash Checker GUI while keeping the XDG portal as the default.
- **Current Context**: The application uses `eframe 0.33` + `egui_glow` and `rfd 0.15.4` with the portal backend. `rfd` does not yet provide an official GTK4 dialog, so the GTK3 portal flow remains the stable default while upstream `gtk4-rs` support matures.

## Assumptions & Dependencies
- Continue maintaining the portal as the default to support sandboxed environments like Flatpak.
- When activating GTK4, install `libgtk-4-dev`, `libadwaita-1-dev`, and the GL/EGL headers that the official GTK guide calls out for development environments.
- Supporting Libraries:
  - `gtk-easy-egui` (formerly the `gtk5` branch) is being developed to embed egui into GTK4 via GLArea; treat it as experimental because the maintainers still mark it under active development.

## Risks & Mitigations
- `gtk::GLArea` on Xorg has previously encountered rendering errors when using the GSK renderer 4.16, which were mitigated either by setting `GSK_RENDERER=vulkan` or upgrading to patch 4.16.1 or newer.
- The egui team previously removed the GLArea backend due to GTK4 bugs, so experiments must monitor for regressions and be ready to fall back to the portal workflow.

## Phases
### Phase 1 – Detailed Research & Prototype (1–2 weeks)
1. Monitor the progress of `rfd` (issue/branch) and consider a community fork if it's stable.
2. Evaluate `gtk-easy-egui`/`gtk-egui-area` and try building a prototype to embed egui into `gtk4::GLArea`.
3. Log findings in `logs/gtk4/` (log started on 2025-10-22) and update documentation if new blockers are discovered.

### Phase 2 – Execution (2–3 weeks)
1. Create branch `feature/gtk4-native-dialogs-issue39`.
2. Add a new feature flag (e.g., `gtk4-native`) in the GUI crate; keep the portal as the default.
3. Write a GTK4 dialog wrapper (based on the `FileDialog`/`FileChooserNative` API) and unify its behavior with the portal.
4. Update Docker/Makefile scripts to install GTK4 packages when the flag is enabled.

### Phase 3 – Testing & Packaging (1–2 weeks)
1. Automated Testing: `make ci-linux-local`, `cargo test`, smoke test the GUI with GTK4.
2. Manual Testing: Xorg + Wayland, AppImage, `.deb`; log results in `logs/qa/gtk4-<date>.md`.
3. Adjust the Flatpak manifest to keep the portal in sandboxed environments.

### Phase 4 – Rollout
1. Summarize results, update CHANGELOG/README/docs/OPERATIONS.md.
2. Prepare a PR, link Issue #39, and provide test logs and installation instructions.
3. After merging, monitor for regressions (especially on Xorg with the GL renderer).

## Deliverables
- Research Log (`logs/gtk4/20251022-research.md`).
- Prototype/Proof-of-Concept PR with CI Linux run 18708523500 (smoke portal + `cargo check` + `dbus-run-session` GTK4-native).
- Updated documentation (README, docs/DEPENDENCY_MIGRATION.md, CHANGELOG, guides).
- Test plan and packaging bundle have been approved.
