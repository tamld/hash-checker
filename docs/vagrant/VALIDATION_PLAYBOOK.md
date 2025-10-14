# Vagrant Release Validation Playbook

_Updated: 2025-10-13_

## Purpose
Run Windows and Linux smoke tests in clean Vagrant VMs before publishing a release.

## Environments
- Windows VM: `hash-checker-win-smoke` (see `scripts/vagrant-gui-smoke.sh`).
- Linux VM: `ubuntu/focal64` box with Docker pre-installed.

## Prerequisites
- Vagrant + VMware Fusion (as documented in README).
- Release artefacts placed under `dist/` or downloaded from the draft release.

## Steps
1. `make rust-gui-smoke` (Linux) or `make rust-gui-smoke-host` for quick local verification.
2. `vagrant up` (or the helper script) to start the environment.
3. Windows VM:
   - Copy artefacts via synced folder or `vagrant scp`.
   - Run `hash-checker-gui.exe --smoke-test`; if installer is present, install then launch from Start Menu.
   - Save a PowerShell transcript to `C:\vagrant\logs\windows-smoke.txt`.
4. Linux VM:
   - Install `.deb` and AppImage from `dist/linux`.
   - Execute `hash-checker-gui --smoke-test`.
   - Save terminal output to `/vagrant/logs/linux-smoke.txt`.
5. Destroy VMs (`vagrant destroy -f`) after tests.

## Log archival
- Copy logs back to `logs/release-history/<tag>/`.
- Fill out `docs/vagrant/RELEASE_LOG_TEMPLATE.md` and commit or attach to release notes.

## Failure handling
- File an issue with VM details, artefact names, and logs.
- Do not publish until smoke tests pass; rerun the entire flow after fixing.

## Automation notes
- Extend `scripts/vagrant-gui-smoke.sh` to copy artefacts/logs automatically.
- Once SignPath signing is active, add signature verification inside the VM.
