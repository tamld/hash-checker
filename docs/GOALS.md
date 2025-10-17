# Project Goals

*   **Correctness:** The tool must produce accurate and reliable file hashes.
*   **Cross-Platform Compatibility:** The application must run seamlessly on Windows, macOS, and Linux.
*   **Ease of Use:** The command-line interface should be intuitive and user-friendly.
*   **Performance:** The tool should be efficient, especially when hashing large files or directories.
*   **Extensibility:** The codebase should be designed to easily accommodate new hashing algorithms and features.

## Strategic Focus

1. **Core Integrity Scope** – Stay specialised in integrity verification (single files, manifests, batch comparison). Adhesion to verification means avoiding feature creep into backup, antivirus, or storage orchestration.
2. **Trust & Compliance** – Deliver enterprise trust signals (signed artefacts, SBOMs, audited supply chain, clear security SLAs). Security automation is prioritised over tangential functionality.
3. **Automation-Friendly CLI** – CLI is the primary surface for CI/CD and scripting. Investment goes into structured logs, manifest operations, exit-code hygiene, and plugin hooks.
4. **Operator-Friendly GUI** – GUI focuses on accessibility, visual feedback, and drag-and-drop workflows. Release pacing can differ from CLI, but both share the same core engine and parity expectations.

## Roadmap Boundaries

* **Do** – hashing for files/directories, manifest generation/validation, batch comparison reports, telemetry/logging to aid troubleshooting, integrations via lightweight SDK or plugin.
* **Do Not** – own storage lifecycle, malware scanning, or heavy identity/access management. Such concerns stay external so the product remains lightweight and auditable.

## Parallel Development Principles

* **Shared Core** – CLI and GUI consume the same hashing engine; shared unit/bench tests guarantee parity.
* **Dedicated Tracks** – CLI and GUI maintain independent user stories and UX requirements. Each can iterate at its own speed while triggering the unified release checklist to ensure compatibility.
* **Evidence-First Iteration** – Every change follows Plan → Execute → Document → Report, attaching test logs, release notes, and retrospective updates (`docs/OPERATIONS.md`, `docs/PROJECT_STATUS.md`).
