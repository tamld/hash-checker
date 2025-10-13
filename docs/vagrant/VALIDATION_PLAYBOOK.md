# Vagrant Release Validation Playbook

_Updated: 2025-10-13_

## Purpose
Đảm bảo artefact Windows và Linux (installer/portable) hoạt động trong môi trường sạch trước khi phát hành chính thức.

## Environments
- **Windows smoke VM**: `hash-checker-win-smoke` (Vagrantfile nằm trong repo).
- **Linux smoke VM**: sử dụng box `ubuntu/focal64` với Docker đã cài sẵn.

## Prerequisites
- Vagrant + provider (VMware Fusion theo hướng dẫn trong README).
- Artefact cần test nằm trong `dist/` hoặc tải từ GitHub Release draft.

## Execution Steps
1. `make rust-gui-smoke` (Linux) hoặc `make rust-gui-smoke-host` để xác nhận GUI chạy qua CLI.
2. `vagrant up` (sử dụng script `scripts/vagrant-gui-smoke.sh` nếu cần tự động).
3. Bên trong máy ảo Windows:
   - Copy artefact từ host (`vagrant scp` hoặc thư mục sync `./dist` → `C:\vagrant`).
   - Chạy `hash-checker-gui.exe --smoke-test` và ghi lại kết quả.
   - Nếu có installer, chạy setup, sau đó chạy ứng dụng từ Start Menu.
4. Bên trong máy ảo Linux:
   - Cài đặt gói `.deb` và AppImage từ `dist/linux`.
   - Thực thi `hash-checker-gui --smoke-test`.
5. Thu thập log:
   - Windows: lưu PowerShell transcript (`Start-Transcript`) vào `C:\vagrant\logs\windows-smoke.txt`.
   - Linux: ghi output vào `/vagrant/logs/linux-smoke.txt`.
6. Sau khi kiểm tra xong: `vagrant destroy -f` để giải phóng tài nguyên.

## Log Archiving
- Sao chép các file log về host và lưu tại `logs/release-history/<tag>/` với cấu trúc:
  - `logs/release-history/<tag>/windows-smoke.txt`
  - `logs/release-history/<tag>/linux-smoke.txt`
- Ghi chú nhanh vào `logs/release-history/<tag>/README.md` (template tùy chọn) tóm tắt kết quả.

## Failure Handling
- Nếu smoke test thất bại, mở issue với mô tả VM, output, commit hash.
- Không phát hành cho tới khi smoke test pass. Lặp lại toàn bộ quy trình sau khi fix.

## Automation Notes
- Script `scripts/vagrant-gui-smoke.sh` có thể mở rộng để tự động copy artefact và thu log.
- Khi SignPath ký Windows installer, bổ sung bước xác minh chữ ký trong máy ảo.
