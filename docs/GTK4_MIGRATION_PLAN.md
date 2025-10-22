# GTK4 Migration Plan

## Overview
- **Goal**: bổ sung chế độ GTK4-native tùy chọn cho Hash Checker GUI trong khi vẫn giữ đường XDG portal mặc định.
- **Bối cảnh hiện tại**: ứng dụng sử dụng `eframe 0.33` + `egui_glow` và `rfd 0.15.4` với backend portal; `rfd` chưa cung cấp GTK4 dialogue chính thức, thread "Add GTK4 dialog support" xác nhận cần chờ `gtk4-rs` ổn định và hiện tại tiếp tục dùng portal/GTK3 citeturn1search1.

## Assumptions & Dependencies
- Tiếp tục duy trì portal làm mặc định để đáp ứng môi trường sandbox như Flatpak.
- Khi kích hoạt GTK4, cần cài đặt `libgtk-4-dev`, `libadwaita-1-dev` và các header GL/EGL theo hướng dẫn GTK chính thức citeturn13search7.
- Các thư viện bổ trợ:
  - `gtk-easy-egui` (tiền thân `gtk5` branch) đang được phát triển nhằm nhúng egui vào GTK4 thông qua GLArea; vẫn cảnh báo đang trong quá trình hoàn thiện citeturn1search0.

## Risks & Mitigations
- `gtk::GLArea` trên Xorg từng gặp lỗi render khi sử dụng GSK renderer 4.16, yêu cầu đặt `GSK_RENDERER=vulkan` hoặc cập nhật lên bản vá 4.16.1 trở lên citeturn0search3.
- Đội egui từng gỡ backend GLArea do lỗi trong GTK4, vì vậy việc thử nghiệm phải theo dõi các issue tương tự và chuẩn bị fallback về portal citeturn0search5.

## Phases
### Phase 1 – Nghiên cứu chi tiết & prototype (1–2 tuần)
1. Theo dõi tiến trình `rfd` (issue/branch) và cân nhắc fork cộng đồng nếu ổn định.
2. Đánh giá `gtk-easy-egui`/`gtk-egui-area` và thử dựng prototype nhúng egui vào `gtk4::GLArea`.
3. Ghi chép vào `logs/gtk4/` (đã mở nhật ký ngày 2025-10-22) và cập nhật tài liệu nếu phát hiện blocker mới.

### Phase 2 – Thực thi (2–3 tuần)
1. Tạo nhánh `feature/gtk4-native-dialogs-issue39`.
2. Thêm feature flag mới (ví dụ `gtk4-native`) trong GUI crate; giữ portal làm default.
3. Viết wrapper GTK4 dialog (dựa trên API `FileDialog`/`FileChooserNative`) và đồng nhất behaviour với portal.
4. Cập nhật script Docker/Makefile để cài gói GTK4 khi bật flag.

### Phase 3 – Kiểm thử & Packaging (1–2 tuần)
1. Kiểm thử tự động: `make ci-linux-local`, `cargo test`, smoke test GUI với GTK4.
2. Kiểm thử thủ công: Xorg + Wayland, AppImage, `.deb`; ghi log vào `logs/qa/gtk4-<date>.md`.
3. Điều chỉnh Flatpak manifest để giữ portal khi ở sandbox.

### Phase 4 – Rollout
1. Tổng hợp kết quả, cập nhật CHANGELOG/README/docs/OPERATIONS.md.
2. Chuẩn bị PR, liên kết Issue #39, cung cấp log kiểm thử và hướng dẫn cài đặt.
3. Sau merge, theo dõi regressions (đặc biệt trên Xorg với renderer GL).

## Deliverables
- Nhật ký research (`logs/gtk4/20251022-research.md`).
- Prototype/PR minh chứng.
- Cập nhật tài liệu (README, docs/DEPENDENCY_MIGRATION.md, CHANGELOG, guides).
- Bảng kiểm thử và gói packaging đã thông qua.
