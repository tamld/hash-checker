# Package Distribution Roadmap (Staging Checklist)

_Updated: 2025-10-14_

This document captures the detailed plan for publishing Hash Checker to major package managers. It is not urgent for the current release cycle, but should be used during staging / future phases when we expand distribution.

## 0. Common Preparation
- **Repository hygiene**: LICENSE, README, CHANGELOG, CONTRIBUTING, SECURITY, CI badges; semantic version tags `vX.Y.Z`; GitHub Release assets + SHA256 checksums (+ optional GPG signatures).
- **Build matrix**: Windows (x64, arm64), macOS (x86_64, arm64 universal), Linux (x86_64, arm64).
- **Signing**: integrate SignPath job in GitHub Actions; can publish unsigned binaries first and re-upload signed versions once SignPath approves.
- **Testing**: unit/integration tests, smoke tests on sample files, basic performance check, hash validation.
- **Checksums**: maintain SHA256 manifest for reuse across manifests.
- **Versioning**: strict SemVer; release URLs must remain stable (package managers depend on them).

## 1. Debian/Ubuntu via Launchpad PPA
- **Prerequisites**: Ubuntu LTS build host, toolchain (`build-essential`, `dpkg-dev`, `debhelper`, `devscripts`, `lintian`, `dh-make`), GPG key linked to Launchpad.
- **Debianize source**: generate `debian/` directory (`dh_make`/`debmake`); fill `control`, `changelog`, `rules`, `copyright`.
- **Build source package**: create `.orig.tar.gz`; run `debuild -S -sa`; lint with `lintian`.
- **Create PPA**: add SSH & GPG keys; configure `dput`.
- **Upload**: `dput ppa:<id>/<ppa-name> <source.changes>`; monitor Launchpad builds.
- **User instructions**: `add-apt-repository ppa:...`; `apt install hash-checker`.
- **Long-term**: consider Debian sponsorship if adoption grows.

## 2. Homebrew (macOS)
- **Option A**: PR to `homebrew-core` (requires popularity/stability).
- **Option B**: maintain personal tap for rapid distribution.
- **Steps**: create formula from release tarball (build-from-source via cargo); include `test do` block; run `brew audit --new --strict --online`; submit PR to `homebrew-core` or publish to tap.
- **Automation**: script to bump formula on new release.

## 3. WinGet (Windows)
- **Installer**: MSI preferred; EXE acceptable with silent switches; host on GitHub Releases.
- **Manifest**: use `wingetcreate new` or manual YAML (version/installer/defaultLocale); include SHA256, architecture list.
- **Validation**: `winget validate`; run `SandboxTest.ps1`; local `winget install --manifest`.
- **Submission**: PR to `microsoft/winget-pkgs`; handle bot feedback.
- **Automation**: `wingetcreate update` script as part of release workflow.

## 4. Chocolatey (Windows)
- **Scaffold**: `choco new hash-checker`; update `.nuspec` metadata.
- **Install script**: `tools/chocolateyinstall.ps1` using `Install-ChocolateyPackage` with installer URL/checksum.
- **Pack & Test**: `choco pack`; local `choco install hash-checker --source .`; verify uninstall.
- **Publish**: configure API key; `choco push ...`; monitor moderation queue.

## 5. CI/CD & Signing
- **Workflow outline**:
  - Build matrix (Linux/macOS/Windows).
  - Optional SignPath signing job (gated by secrets).
  - Release job attaches assets/checksums.
  - Package manager jobs (Debian source upload, Homebrew formula update, WinGet manifest PR, Chocolatey push).
- **Unsigned flow**: allow publishing unsigned packages for preliminary review, then regenerate with signed binaries once certificates available.
- **Best practices**: separate secrets per environment, audit logs, reproducible builds, verify checksums pre/post signing.

## 6. Timeline Template (14 days)
1–2: Repo hygiene, CI, installer prep.
3–4: Debianize + upload PPA.
5–6: Homebrew formula draft + PR; personal tap fallback.
7: WinGet manifest creation + PR.
8: Chocolatey packaging + push.
9–14: Monitor reviews, address feedback, document installation instructions.

## 7. Tracking
- For each package manager, maintain status in `.agents/project_state.yml` (planning/in-progress/shipped).
- Keep log of submission links and moderation feedback for future updates.

References: see numbered sources in original plan (GitHub releases, SignPath docs, Debian packaging guides, Homebrew documentation, WinGet spec, Chocolatey quick start, etc.).
