# Vagrant Release Validation Playbook

_Updated: 2025-10-14_

## Purpose
Execute GUI smoke tests inside real virtual machines, ensuring releases behave the same as on end-user desktops (beyond container-only CI).

## Why it remains manual
- GitHub-hosted runners do not support nested VMware/VirtualBox and cannot ship licensed Windows images; self-hosted hardware is required.
- The helper script `scripts/vagrant-gui-smoke.sh` can orchestrate the run and capture logs, but creating/destroying the VM and providing secrets/licences still requires an operator.
- Workflow `.github/workflows/vagrant-smoke-reminder.yml` creates quarterly reminder issues. Once suitable self-hosted runners are available, this checklist can be automated within CI.

## Environments
- Linux VM: `generic/ubuntu2204` defined in the root `Vagrantfile` (headless, VMware Fusion provider).
- Optional Windows VM: provision manually when Windows smoke validation is required; record the steps in release notes.

## Prerequisites
- Vagrant + VMware Fusion (see README).
- Artefacts from the target release (`dist/` or downloaded from the draft release).

## Steps
1. Run `make rust-gui-smoke` (host) for a quick pre-check.
2. Launch the VM:
   ```bash
   vagrant up
   ```
3. Inside the VM run smoke tests:
   - Linux: `hash-checker-gui --smoke-test` or use the helper script.
   - Windows (if available): `hash-checker-gui.exe --smoke-test`; capture a PowerShell transcript at `C:\vagrant\logs\windows-smoke.txt`.
4. Collect logs in `/workspace/logs/` (Linux) or the synced folder.
5. Tear down the environment:
   ```bash
   vagrant halt
   vagrant destroy -f
   ```

## Log archival
- Copy logs to `logs/release-history/<tag>/` on the host.
- Fill out `docs/vagrant/RELEASE_LOG_TEMPLATE.md` and link it in the release PR/issue.

## Failure handling
- Open an issue with VM details, artefact names, and log attachments.
- Do not publish the release until all smoke tests pass.

## Future automation
- When self-hosted runners with virtualization are available, integrate this playbook into CI and upload logs automatically.
- Consider headless UI testing (e.g., Playwright) once the environment is automated.
