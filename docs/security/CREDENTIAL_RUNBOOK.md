# Credential Runbook

_Updated: 2025-10-13_

Tài liệu này mô tả quy trình quản lý khóa và chứng thư liên quan tới việc ký phát hành Hash Checker.

## 1. Phạm vi
- Chứng thư/khoá do SignPath Foundation cấp (test certificate, release certificate).
- Biến môi trường/secret trong GitHub Actions (`SIGNPATH_API_TOKEN`, `SIGNPATH_ORGANIZATION_ID`, ...).
- Khóa GPG dùng để ký file `SHA256SUMS` (nếu được bật trong pipeline).

## 2. Lưu trữ & phân quyền
- Secret dài hạn được lưu trong GitHub Actions secrets (repo private hoặc organization) với quyền truy cập giới hạn cho maintainers.
- Passphrase và file PGP private key chỉ lưu trong vault nội bộ (không commit). Truy cập yêu cầu xác thực MFA.
- Các biến SignPath ở dạng `vars` (non-secret) chỉ chứa mã định danh; token thực phải ở `secrets`.

## 3. Cấp mới / onboarding
1. Hoàn tất quy trình SignPath OSS để nhận test certificate.
2. Ghi log vào `logs/credentials/<yyyy-mm-dd>-signpath-onboarding.md` (template trong mục 6).
3. Thêm token SignPath vào GitHub secrets (`SIGNPATH_API_TOKEN`), các slug Id vào GitHub variables.
4. Cập nhật workflow `.github/workflows/release.yml` nếu có đường dẫn artefact mới.

## 4. Rotation định kỳ
- Đặt lịch 6 tháng/lần cho token SignPath và passphrase GPG.
- Khi rotation: tạo token mới, cập nhật secret, thử chạy `gh workflow run release.yml --ref main --field run_windows=true --field run_linux=false --field run_macos=false` ở chế độ dry-run.
- Lưu log rotation trong `logs/credentials/<yyyy-mm-dd>-rotation.md` kèm người thực hiện.

## 5. Sự cố & khôi phục
- Nếu nghi ngờ rò rỉ: thu hồi token SignPath trong dashboard, vô hiệu hóa workflow release (tạm thời chỉnh `permissions: contents: read`).
- Thu hồi chứng thư thông qua SignPath ticket; phát hành bản vá mới với chứng thư mới.
- Tạo báo cáo sự cố trong `logs/credentials/<yyyy-mm-dd>-incident.md`.

## 6. Templates log
```text
# SignPath onboarding log
Date:
Operator:
Certificate type: test | release
Secrets updated:
Verification: (workflow run URL)
Notes:
```

```text
# Credential rotation log
Date:
Operator:
Secrets rotated:
Verification:
Notes:
```

## 7. Công việc tiếp theo
- Đồng bộ runbook này vào không gian nội bộ nếu có thay đổi.
- Cập nhật `docs/SIGNING.md` khi quy trình SignPath chính thức hoạt động.
