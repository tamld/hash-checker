#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use std::{
    collections::VecDeque,
    env,
    ffi::OsStr,
    fs::{self, remove_file, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{
        mpsc::{self, Receiver, TryRecvError},
        Arc,
    },
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use eframe::{egui, App, Frame, NativeOptions};
use egui::{
    output::OutputCommand, vec2, Align, Color32, FontFamily, FontId, Frame as EguiFrame, IconData,
    Id, Key, KeyboardShortcut, Layout, Modifiers, Popup, RichText, StrokeKind, TextStyle,
    ViewportBuilder, ViewportId,
};
use hash_checker::{
    compute_hash, detect_format_from_extension, generate_manifest, read_manifest, resolve_root,
    supported_algorithms, verify_hash, verify_manifest, write_manifest, Manifest, ManifestEntry,
    ManifestFormat, VerificationReport,
};
use image::RgbaImage;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
mod dialog;
use serde::Serialize;
use serde_json::{to_value, to_writer_pretty, Value};
mod comparator;

const BASE_SPACING: f32 = 8.0;
const LARGE_SPACING: f32 = 16.0;
const BUTTON_HEIGHT: f32 = 40.0;
const TABLE_ROW_HEIGHT: f32 = 40.0;
const TELEMETRY_DIR: &str = "logs/gui-manifest";
const TELEMETRY_FILE: &str = "telemetry.log";
const MAX_CONTENT_WIDTH: f32 = 960.0;
const ACTION_BUTTON_MIN_WIDTH: f32 = 160.0;
const SECONDARY_BUTTON_MIN_WIDTH: f32 = 184.0;
const SNAPSHOT_DEFAULT_WIDTH: u32 = 1280;
const SNAPSHOT_DEFAULT_HEIGHT: u32 = 800;
const README_SNAPSHOT_WIDTH: u32 = 1440;
const README_SNAPSHOT_HEIGHT: u32 = 900;

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn canonicalize_or(path: PathBuf) -> PathBuf {
    path.canonicalize().unwrap_or(path)
}

fn fixture_path(relative: &str) -> PathBuf {
    canonicalize_or(project_root().join(relative))
}

#[cfg(target_os = "windows")]
fn demo_storage_dir() -> PathBuf {
    let base = PathBuf::from(r"C:\Temp\hash-checker-gui");
    if let Err(err) = fs::create_dir_all(&base) {
        eprintln!(
            "Failed to create demo storage directory {}: {err}",
            base.display()
        );
    }
    base
}

#[cfg(not(target_os = "windows"))]
fn demo_storage_dir() -> PathBuf {
    let base = PathBuf::from("/tmp/hash-checker-gui");
    if let Err(err) = fs::create_dir_all(&base) {
        eprintln!(
            "Failed to create demo storage directory {}: {err}",
            base.display()
        );
    }
    base
}

const DEMO_SAMPLE_CONTENT: &str = "Hash Checker GUI demo sample\n";

fn ensure_demo_sample_file() -> PathBuf {
    let path = demo_storage_dir().join("sample.txt");
    if !path.exists() {
        if let Some(parent) = path.parent() {
            if let Err(err) = fs::create_dir_all(parent) {
                eprintln!(
                    "Failed to create demo sample parent directory {}: {err}",
                    parent.display()
                );
            }
        }
        if let Err(err) = fs::write(&path, DEMO_SAMPLE_CONTENT) {
            eprintln!("Failed to write demo sample file {}: {err}", path.display());
        }
    }
    path
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else if file_type.is_file() {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

fn ensure_demo_manifest_dir() -> PathBuf {
    let dest = demo_storage_dir().join("gui-deep");
    if !dest.exists() {
        let src = fixture_path("test-fixtures/gui-deep");
        if let Err(err) = copy_dir_recursive(&src, &dest) {
            eprintln!(
                "Failed to prepare demo manifest directory {}: {err}",
                dest.display()
            );
        }
    }
    dest
}

fn responsive_multiplier(width: f32) -> f32 {
    if width <= 640.0 {
        0.9
    } else if width <= 1024.0 {
        1.0
    } else if width <= 1440.0 {
        1.08
    } else {
        1.15
    }
}

fn margin_symmetric(x: f32, y: f32) -> egui::Margin {
    egui::Margin::from(vec2(x, y))
}

fn margin_all(v: f32) -> egui::Margin {
    egui::Margin::from(v)
}

fn configure_typography(ctx: &egui::Context, scale: f32) {
    ctx.style_mut(|style| {
        style.spacing.item_spacing = vec2(BASE_SPACING * scale, BASE_SPACING * scale);
        style.spacing.button_padding = vec2(14.0 * scale, 10.0 * scale);
        style.spacing.window_margin = margin_symmetric(18.0 * scale, 16.0 * scale);
        style.spacing.menu_margin = margin_all(10.0 * scale);
        style.spacing.interact_size = vec2(0.0, 38.0 * scale);

        style.text_styles.insert(
            TextStyle::Heading,
            FontId::new(18.0 * scale, FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Body,
            FontId::new(15.0 * scale, FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Button,
            FontId::new(15.0 * scale, FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Small,
            FontId::new(13.0 * scale, FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Monospace,
            FontId::new(13.0 * scale, FontFamily::Monospace),
        );
    });
}

fn with_constrained_ui<R>(
    ui: &mut egui::Ui,
    max_width: f32,
    body: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    ui.vertical_centered(|ui| {
        let width = max_width.min(ui.available_width());
        ui.set_min_width(width);
        ui.set_max_width(width);
        ui.with_layout(egui::Layout::top_down(Align::Min), |ui| body(ui))
            .inner
    })
    .inner
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActiveTab {
    File,
    DirectoryManifest,
}

impl ActiveTab {
    fn label(self) -> &'static str {
        match self {
            ActiveTab::File => "File Hash",
            ActiveTab::DirectoryManifest => "Folder Scan",
        }
    }
}

struct HashCheckerApp {
    active_tab: ActiveTab,
    file_path: String,
    expected_hash: String,
    algorithm: AlgorithmChoice,
    computed_hash: Option<String>,
    status: Option<StatusMessage>,
    theme: ThemeChoice,
    last_algorithm_used: Option<String>,
    manifest: ManifestView,
    snapshot_jobs: VecDeque<SnapshotRequest>,
    force_open_algorithm_popup: bool,
    snapshot_config: Option<SnapshotOptions>,
    snapshot_capture: Option<SnapshotCaptureState>,
}

#[derive(Clone, Copy)]
enum SnapshotScenario {
    FileDefault,
    FileMatch,
    FileMismatch,
    FileAlgorithmDropdown,
    FileHighContrast,
    ManifestSummary,
    ManifestDetails,
}

impl SnapshotScenario {
    fn active_tab(self) -> ActiveTab {
        match self {
            SnapshotScenario::FileDefault
            | SnapshotScenario::FileMatch
            | SnapshotScenario::FileMismatch
            | SnapshotScenario::FileAlgorithmDropdown
            | SnapshotScenario::FileHighContrast => ActiveTab::File,
            SnapshotScenario::ManifestSummary | SnapshotScenario::ManifestDetails => {
                ActiveTab::DirectoryManifest
            }
        }
    }

    fn label(&self) -> &'static str {
        match self {
            SnapshotScenario::FileDefault => "file-default",
            SnapshotScenario::FileMatch => "file-match",
            SnapshotScenario::FileMismatch => "file-mismatch",
            SnapshotScenario::FileAlgorithmDropdown => "file-algorithm-dropdown",
            SnapshotScenario::FileHighContrast => "file-high-contrast",
            SnapshotScenario::ManifestSummary => "manifest-summary",
            SnapshotScenario::ManifestDetails => "manifest-details",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SnapshotPreset {
    Default,
    Readme,
}

impl SnapshotPreset {
    fn label(&self) -> &'static str {
        match self {
            SnapshotPreset::Default => "default",
            SnapshotPreset::Readme => "readme",
        }
    }
}

struct SnapshotRequest {
    path: PathBuf,
    width: u32,
    height: u32,
    stage: SnapshotStage,
    scenario: SnapshotScenario,
}

impl SnapshotRequest {
    fn new(path: PathBuf, width: u32, height: u32, scenario: SnapshotScenario) -> SnapshotRequest {
        SnapshotRequest {
            path,
            width,
            height,
            stage: SnapshotStage::Configure,
            scenario,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SnapshotStage {
    Configure,
    AwaitPaint,
    RequestCapture,
    AwaitImage,
    Done,
}

impl HashCheckerApp {
    fn new() -> Self {
        let mut algorithms: Vec<String> = supported_algorithms()
            .iter()
            .map(|s| s.to_string())
            .collect();
        algorithms.insert(0, "auto".to_owned());
        Self {
            active_tab: ActiveTab::File,
            file_path: String::new(),
            expected_hash: String::new(),
            algorithm: AlgorithmChoice {
                algorithms,
                selected_index: 0,
            },
            computed_hash: None,
            status: None,
            theme: ThemeChoice::default(),
            last_algorithm_used: None,
            manifest: ManifestView::new(),
            snapshot_jobs: VecDeque::new(),
            force_open_algorithm_popup: false,
            snapshot_config: None,
            snapshot_capture: None,
        }
    }

    fn process_input(&mut self, ctx: &egui::Context) {
        let dropped = ctx.input(|i| i.raw.dropped_files.clone());
        if let Some(path) = dropped.iter().find_map(|file| file.path.as_ref()) {
            match self.active_tab {
                ActiveTab::File => {
                    self.file_path = path.to_string_lossy().into();
                    self.set_status("File selected from drag-and-drop.", StatusKind::Info);
                }
                ActiveTab::DirectoryManifest => {
                    if path.is_dir() {
                        self.manifest.selected_dir = Some(path.to_path_buf());
                        self.manifest.banner =
                            Some(ManifestBanner::info("Folder selected from drag-and-drop."));
                    }
                }
            }
        }

        if self.active_tab == ActiveTab::File
            && ctx.input(|i| i.key_pressed(Key::Enter) && !i.modifiers.any())
            && !ctx.wants_keyboard_input()
        {
            self.calculate();
        }
    }

    fn render_top_bar(&mut self, ui: &mut egui::Ui, scale: f32, display_tab: ActiveTab) {
        ui.horizontal_wrapped(|ui| {
            for tab in [ActiveTab::File, ActiveTab::DirectoryManifest] {
                let selected = display_tab == tab;
                if ui.selectable_label(selected, tab.label()).clicked() {
                    self.active_tab = tab;
                }
            }

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                self.theme_selector(ui);
            });
        });
        ui.add_space(BASE_SPACING * scale);
        ui.separator();
        ui.add_space(BASE_SPACING * scale);
    }

    fn ui_file_tab(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, scale: f32) {
        self.process_input(ctx);
        let spacing = BASE_SPACING * scale;
        let button_size = vec2(SECONDARY_BUTTON_MIN_WIDTH * scale, BUTTON_HEIGHT);
        let action_size = vec2(ACTION_BUTTON_MIN_WIDTH * scale, BUTTON_HEIGHT);
        let combo_width = 170.0 * scale;

        ui.heading("Hash Checker (Rust GUI)");
        ui.add_space(spacing);

        egui::Frame::new()
            .inner_margin(margin_symmetric(12.0 * scale, 0.0))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = vec2(spacing, spacing);
                ui.horizontal(|ui| {
                    ui.label("File:");
                    let available = ui.available_width();
                    let edit_width = (available - button_size.x - spacing).max(280.0);
                    let response = egui::TextEdit::singleline(&mut self.file_path)
                        .hint_text("Select a file to hash")
                        .font(TextStyle::Body)
                        .desired_width(edit_width)
                        .show(ui)
                        .response;
                    if response.changed() {
                        self.computed_hash = None;
                    }
                    if ui
                        .add_sized(button_size, egui::Button::new("Browse…"))
                        .clicked()
                    {
                        if let Some(path) = dialog::pick_file() {
                            if let Some(path_str) = path.to_str() {
                                self.file_path = path_str.to_owned();
                                self.computed_hash = None;
                                self.set_status("File selected.", StatusKind::Info);
                            }
                        }
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Algorithm:");
                    let current = self
                        .algorithm
                        .algorithms
                        .get(self.algorithm.selected_index)
                        .cloned()
                        .unwrap_or_else(|| "auto".to_string());
                    let combo_id = Id::new("file_algorithm_combo");
                    if self.force_open_algorithm_popup {
                        Popup::open_id(ctx, combo_id);
                        self.force_open_algorithm_popup = false;
                    }
                    egui::ComboBox::from_id_salt(combo_id)
                        .width(combo_width)
                        .selected_text(current)
                        .show_ui(ui, |combo| {
                            for (idx, label) in self.algorithm.algorithms.iter().enumerate() {
                                combo.selectable_value(
                                    &mut self.algorithm.selected_index,
                                    idx,
                                    label,
                                );
                            }
                        });
                });

                ui.label("Expected hash (optional):");
                let expected_response = egui::TextEdit::singleline(&mut self.expected_hash)
                    .hint_text("Paste the expected hash (prefix auto-detected)")
                    .desired_width(f32::INFINITY)
                    .font(TextStyle::Body)
                    .show(ui)
                    .response;
                if expected_response.changed() {
                    self.handle_expected_hash_change();
                }

                ui.horizontal(|ui| {
                    if ui
                        .add_sized(
                            action_size,
                            egui::Button::new("Calculate").fill(Color32::from_rgb(30, 102, 245)),
                        )
                        .clicked()
                    {
                        self.calculate();
                    }
                    if ui
                        .add_sized(button_size, egui::Button::new("Clear"))
                        .clicked()
                    {
                        self.file_path.clear();
                        self.expected_hash.clear();
                        self.computed_hash = None;
                        self.status = None;
                    }
                });

                if self.file_path.is_empty() {
                    ui.separator();
                    ui.label(
                        RichText::new("Drop a file here or use Browse… to select one.").italics(),
                    );
                }
            });

        ui.add_space(spacing);

        if let Some(status) = &self.status {
            let color = match status.kind {
                StatusKind::Info => Color32::from_rgb(120, 125, 136),
                StatusKind::Success => Color32::from_rgb(0, 170, 0),
                StatusKind::Warning => Color32::YELLOW,
                StatusKind::Error => Color32::RED,
            };
            ui.colored_label(color, RichText::new(&status.text).strong());
            ui.add_space(spacing);
        }

        egui::Frame::group(&ctx.style())
            .fill(ui.visuals().extreme_bg_color.gamma_multiply(1.05))
            .stroke(egui::Stroke::new(0.0, Color32::TRANSPARENT))
            .inner_margin(margin_symmetric(16.0 * scale, 12.0 * scale))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.spacing_mut().item_spacing = vec2(spacing, spacing);
                ui.label(
                    RichText::new("Computed hash")
                        .text_style(TextStyle::Button)
                        .strong(),
                );
                ui.horizontal(|ui| {
                    let available = ui.available_width();
                    let button_width = SECONDARY_BUTTON_MIN_WIDTH * scale;
                    let mut display_hash = self.computed_hash.clone().unwrap_or_default();
                    ui.add_enabled(
                        false,
                        egui::TextEdit::singleline(&mut display_hash)
                            .desired_width((available - button_width - spacing).max(220.0))
                            .font(TextStyle::Monospace),
                    );
                    let response = ui.add_enabled(
                        self.computed_hash.is_some(),
                        egui::Button::new("Copy hash").min_size(vec2(button_width, BUTTON_HEIGHT)),
                    );
                    if response.clicked() {
                        let value = self.computed_hash.clone().unwrap_or_default();
                        let algo = self
                            .last_algorithm_used
                            .clone()
                            .unwrap_or_else(|| self.resolve_algorithm().to_ascii_uppercase());
                        let formatted = format!("{algo}:{value}");
                        ctx.output_mut(|out| out.commands.push(OutputCommand::CopyText(formatted)));
                        self.set_status("Hash copied to clipboard.", StatusKind::Info);
                    }
                });
            });
    }
}

fn poll_screenshot_image(ctx: &egui::Context) -> Option<Arc<egui::ColorImage>> {
    ctx.input(|input| {
        input.events.iter().rev().find_map(|event| match event {
            egui::Event::Screenshot { image, .. } => Some(image.clone()),
            _ => None,
        })
    })
}

fn save_color_image(path: &Path, image: &Arc<egui::ColorImage>) -> Result<PathBuf, String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create snapshot directory: {e}"))?;
    }

    let width = image.width() as u32;
    let height = image.height() as u32;
    let mut buffer = Vec::with_capacity((width * height * 4) as usize);
    for pixel in &image.pixels {
        buffer.extend_from_slice(&pixel.to_array());
    }

    RgbaImage::from_vec(width, height, buffer)
        .ok_or_else(|| "Failed to create RGBA buffer for snapshot.".to_owned())?
        .save(path)
        .map_err(|e| format!("Failed to save snapshot: {e}"))?;

    Ok(path.to_path_buf())
}

fn write_manifest_report(path: &Path, ready: &ManifestReady) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create report directory: {e}"))?;
    }

    let entries = ready
        .entries
        .iter()
        .map(|row| ManifestReportEntryData {
            path: row.path.clone(),
            state: row.state.as_str(),
            expected: row.expected.clone(),
            actual: row.actual.clone(),
        })
        .collect();

    let report = ManifestReportData {
        operation: match ready.operation {
            ManifestOperation::Scan => "scan",
            ManifestOperation::Verify => "verify",
        },
        algorithm: ready.manifest.algorithm.clone(),
        root: ready.root.display().to_string(),
        manifest_path: ready
            .manifest_path
            .as_ref()
            .map(|p| p.display().to_string()),
        summary: ready.summary.clone(),
        duration_ms: ready.duration.as_secs_f64() * 1000.0,
        entries,
    };

    let mut file = File::create(path).map_err(|e| format!("Failed to create report file: {e}"))?;
    to_writer_pretty(&mut file, &report).map_err(|e| format!("Failed to serialize report: {e}"))?;
    file.write_all(b"\n")
        .map_err(|e| format!("Failed to finish report: {e}"))?;
    Ok(())
}

impl App for HashCheckerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        self.paint(ctx);
        self.process_snapshot(ctx, frame);
    }
}

impl HashCheckerApp {
    fn paint(&mut self, ctx: &egui::Context) {
        self.manifest.poll_job();
        let screen_width = ctx.viewport_rect().width();
        let spacing_scale = responsive_multiplier(screen_width);
        let visuals = self.theme.selected_theme().visuals();
        ctx.set_visuals(visuals);
        configure_typography(ctx, spacing_scale);

        let snapshot_tab = self
            .snapshot_jobs
            .front()
            .map(|req| req.scenario.active_tab());
        let display_tab = snapshot_tab.unwrap_or(self.active_tab);

        egui::CentralPanel::default().show(ctx, |panel_ui| {
            let horizontal_margin = 24.0 * spacing_scale;
            egui::Frame::new()
                .inner_margin(margin_symmetric(horizontal_margin, 0.0))
                .show(panel_ui, |inner_ui| {
                    with_constrained_ui(inner_ui, MAX_CONTENT_WIDTH * spacing_scale, |ui| {
                        ui.set_width(ui.available_width());
                        self.render_top_bar(ui, spacing_scale, display_tab);
                        match display_tab {
                            ActiveTab::File => self.ui_file_tab(ctx, ui, spacing_scale),
                            ActiveTab::DirectoryManifest => {
                                self.manifest.ui(ctx, ui, spacing_scale)
                            }
                        }
                    });
                });
        });
    }

    fn process_snapshot(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let mut pop_front = false;
        if let Some((stage, scenario)) = self
            .snapshot_jobs
            .front()
            .map(|req| (req.stage, req.scenario))
        {
            if matches!(stage, SnapshotStage::Configure) {
                self.prepare_snapshot_scenario(scenario, ctx);
                self.record_snapshot_state(scenario);
            }
        }
        if let Some(request) = self.snapshot_jobs.front_mut() {
            match request.stage {
                SnapshotStage::Configure => {
                    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(vec2(
                        request.width as f32,
                        request.height as f32,
                    )));
                    ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(vec2(
                        request.width as f32,
                        request.height as f32,
                    )));
                    ctx.send_viewport_cmd(egui::ViewportCommand::MaxInnerSize(vec2(
                        request.width as f32,
                        request.height as f32,
                    )));
                    request.stage = SnapshotStage::AwaitPaint;
                    ctx.request_repaint();
                }
                SnapshotStage::AwaitPaint => {
                    request.stage = SnapshotStage::RequestCapture;
                    ctx.request_repaint();
                }
                SnapshotStage::RequestCapture => {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                    request.stage = SnapshotStage::AwaitImage;
                    ctx.request_repaint();
                }
                SnapshotStage::AwaitImage => {
                    if let Some(image) = poll_screenshot_image(ctx) {
                        match save_color_image(&request.path, &image) {
                            Ok(path) => println!("Snapshot saved to {}", path.display()),
                            Err(err) => eprintln!("Failed to save snapshot: {err}"),
                        }
                        request.stage = SnapshotStage::Done;
                        ctx.request_repaint();
                    } else {
                        ctx.request_repaint();
                    }
                }
                SnapshotStage::Done => {
                    pop_front = true;
                }
            }
        }

        if pop_front {
            self.snapshot_jobs.pop_front();
            if self.snapshot_jobs.is_empty() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                self.finalize_snapshot_capture();
            } else {
                ctx.request_repaint();
            }
        }
    }

    fn prepare_snapshot_scenario(&mut self, scenario: SnapshotScenario, ctx: &egui::Context) {
        self.reset_snapshot_state();
        match scenario {
            SnapshotScenario::FileDefault => {
                self.active_tab = ActiveTab::File;
            }
            SnapshotScenario::FileMatch => {
                self.configure_file_match();
            }
            SnapshotScenario::FileMismatch => {
                self.configure_file_mismatch();
            }
            SnapshotScenario::FileAlgorithmDropdown => {
                self.configure_file_algorithm_dropdown();
                self.force_open_algorithm_popup = true;
            }
            SnapshotScenario::FileHighContrast => {
                self.configure_file_high_contrast();
            }
            SnapshotScenario::ManifestSummary => {
                self.configure_manifest_snapshot(false);
            }
            SnapshotScenario::ManifestDetails => {
                self.configure_manifest_snapshot(true);
            }
        }
        self.active_tab = scenario.active_tab();
        if matches!(scenario, SnapshotScenario::FileAlgorithmDropdown) {
            Popup::open_id(ctx, Id::new("file_algorithm_combo"));
        }
    }

    fn reset_snapshot_state(&mut self) {
        self.file_path.clear();
        self.expected_hash.clear();
        self.computed_hash = None;
        self.status = None;
        self.last_algorithm_used = None;
        self.active_tab = ActiveTab::File;
        self.theme = ThemeChoice::default();
        self.force_open_algorithm_popup = false;
        self.manifest = ManifestView::new();
        self.manifest.telemetry_enabled = false;
        self.manifest.force_embed_details = false;
        self.manifest.show_details = false;
    }

    fn configure_file_match(&mut self) {
        let sample_path = ensure_demo_sample_file();
        let digest = compute_hash(&sample_path, "sha256")
            .unwrap_or_else(|_| "3da541559918a808c2402bba5012f6c60b27661c".to_owned());
        self.file_path = sample_path.display().to_string();
        self.expected_hash = digest.clone();
        self.select_algorithm("sha256");
        self.computed_hash = Some(digest.clone());
        self.last_algorithm_used = Some("SHA256".to_owned());
        self.set_status("Hashes match.", StatusKind::Success);
    }

    fn configure_file_mismatch(&mut self) {
        let sample_path = ensure_demo_sample_file();
        let digest = compute_hash(&sample_path, "sha256")
            .unwrap_or_else(|_| "3da541559918a808c2402bba5012f6c60b27661c".to_owned());
        self.file_path = sample_path.display().to_string();
        self.expected_hash = "ff00ff00ff00ff00ff00ff00ff00ff00".to_owned();
        self.select_algorithm("sha256");
        self.computed_hash = Some(digest);
        self.last_algorithm_used = Some("SHA256".to_owned());
        self.set_status("Hashes do not match.", StatusKind::Error);
    }

    fn configure_file_algorithm_dropdown(&mut self) {
        let sample_path = ensure_demo_sample_file();
        let digest = compute_hash(&sample_path, "sha512").unwrap_or_else(|_| {
            "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce".to_owned()
        });
        self.file_path = sample_path.display().to_string();
        self.algorithm.selected_index = 0; // auto
        self.expected_hash = format!("sha512:{digest}");
        self.handle_expected_hash_change();
    }

    fn configure_file_high_contrast(&mut self) {
        self.configure_file_match();
        if let Some(index) = self
            .theme
            .presets
            .iter()
            .position(|preset| matches!(preset, ThemePreset::HighContrast))
        {
            self.theme.selected_index = index;
        }
        self.set_status("High contrast mode enabled.", StatusKind::Info);
    }

    fn configure_manifest_snapshot(&mut self, embed_details: bool) {
        let root = ensure_demo_manifest_dir();
        let ready = match generate_manifest(&root, "sha256", true) {
            Ok(manifest) => {
                if embed_details {
                    self.build_manifest_verify_ready(manifest, root.clone())
                } else {
                    ManifestReady::from_scan(manifest, root.clone(), Duration::from_millis(340))
                }
            }
            Err(err) => {
                eprintln!("Failed to build manifest snapshot: {err}");
                return;
            }
        };

        self.manifest.selected_dir = Some(root);
        self.manifest.recursive = true;
        self.manifest.force_embed_details = embed_details;
        self.manifest.telemetry_enabled = false;
        self.manifest.set_ready_state(ready);
        if embed_details {
            self.manifest.show_details = true;
        }
    }

    fn build_manifest_verify_ready(&self, manifest: Manifest, root: PathBuf) -> ManifestReady {
        let mut ready =
            ManifestReady::from_scan(manifest.clone(), root, Duration::from_millis(340));
        if ready.entries.is_empty() {
            return ready;
        }

        ready.operation = ManifestOperation::Verify;
        if let Some(first) = ready.entries.get_mut(0) {
            let reversed = first
                .expected
                .clone()
                .unwrap_or_default()
                .chars()
                .rev()
                .collect::<String>();
            first.actual = Some(reversed);
            first.state = ManifestRowState::Mismatch;
        }
        if ready.entries.len() > 1 {
            if let Some(second) = ready.entries.get_mut(1) {
                second.actual = None;
                second.state = ManifestRowState::Missing;
            }
        }
        ready.entries.push(ManifestRow {
            path: "extras/cache-report.json".to_owned(),
            expected: None,
            actual: Some(
                "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3e4b8b7f4d3e0d0f6f7c5a6".to_owned(),
            ),
            state: ManifestRowState::Extra,
        });

        let missing_count = if manifest.entries.len() > 1 { 1 } else { 0 };
        ready.summary.mismatched = 1;
        ready.summary.missing = missing_count;
        ready.summary.extra = 1;
        ready.summary.matched = manifest
            .entries
            .len()
            .saturating_sub(ready.summary.mismatched + ready.summary.missing);
        ready.summary.recorded = manifest.entries.len();
        ready.summary.total = ready.summary.matched
            + ready.summary.mismatched
            + ready.summary.missing
            + ready.summary.extra;
        ready
    }

    fn select_algorithm(&mut self, name: &str) {
        if let Some(index) = self
            .algorithm
            .algorithms
            .iter()
            .position(|label| label.eq_ignore_ascii_case(name))
        {
            self.algorithm.selected_index = index;
        }
    }

    fn algorithm_label(&self, index: usize) -> &str {
        &self.algorithm.algorithms[index]
    }

    fn selected_algorithm(&self) -> Option<&str> {
        let label = self.algorithm_label(self.algorithm.selected_index);
        if label == "auto" {
            None
        } else {
            Some(label)
        }
    }

    fn resolve_algorithm(&self) -> String {
        self.selected_algorithm()
            .unwrap_or("sha256")
            .to_ascii_lowercase()
    }

    fn selected_theme(&self) -> ThemePreset {
        self.theme.presets[self.theme.selected_index]
    }

    fn calculate(&mut self) {
        let path = PathBuf::from(self.file_path.trim());
        if self.file_path.trim().is_empty() {
            self.set_status(
                "Please choose a file before calculating.",
                StatusKind::Warning,
            );
            return;
        }
        if !path.exists() {
            self.set_status("Selected file does not exist.", StatusKind::Error);
            return;
        }

        let expected = self.expected_hash.trim();
        if expected.is_empty() {
            let algorithm = self.resolve_algorithm();
            match compute_hash(&path, &algorithm) {
                Ok(digest) => {
                    self.computed_hash = Some(digest.clone());
                    self.last_algorithm_used = Some(algorithm.to_ascii_uppercase());
                    self.set_status("Hash computed successfully.", StatusKind::Success);
                }
                Err(err) => {
                    self.computed_hash = None;
                    self.set_status(&format!("Failed to compute hash: {err}"), StatusKind::Error);
                }
            }
        } else {
            let algorithm = self.resolve_algorithm();
            match verify_hash(&path, expected, self.selected_algorithm()) {
                Ok((matches, digest)) => {
                    self.computed_hash = Some(digest.clone());
                    self.last_algorithm_used = Some(algorithm.to_ascii_uppercase());
                    if matches {
                        self.set_status("Hashes match.", StatusKind::Success);
                    } else {
                        self.set_status("Hashes do not match.", StatusKind::Error);
                    }
                }
                Err(err) => {
                    self.computed_hash = None;
                    self.set_status(&format!("Verification failed: {err}"), StatusKind::Error);
                }
            }
        }
    }

    fn set_status(&mut self, text: &str, kind: StatusKind) {
        self.status = Some(StatusMessage {
            text: text.to_owned(),
            kind,
        });
    }

    fn theme_selector(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_id_salt("theme_selector")
            .width(120.0)
            .selected_text(self.selected_theme().name())
            .show_ui(ui, |combo| {
                for (idx, preset) in self.theme.presets.iter().enumerate() {
                    combo.selectable_value(&mut self.theme.selected_index, idx, preset.name());
                }
            });
    }

    fn handle_expected_hash_change(&mut self) {
        let trimmed = self.expected_hash.trim().to_owned();
        if let Some((algo, digest)) = parse_prefixed_hash(&trimmed) {
            if let Some(index) = self
                .algorithm
                .algorithms
                .iter()
                .position(|label| label.eq_ignore_ascii_case(&algo))
            {
                self.algorithm.selected_index = index;
            }
            self.expected_hash = digest.to_string();
            self.last_algorithm_used = Some(algo.to_ascii_uppercase());
            self.set_status(
                &format!("Detected {} from pasted hash.", algo.to_ascii_uppercase()),
                StatusKind::Info,
            );
        } else {
            if let Some(prefix) = extract_prefix(&trimmed) {
                let supported = supported_prefixes().join(", ");
                self.set_status(
                    &format!(
                        "Unsupported hash prefix '{prefix}'. Supported prefixes: {supported}."
                    ),
                    StatusKind::Warning,
                );
            }
            self.expected_hash = trimmed;
        }
    }

    fn preload_manifest_scan(
        &mut self,
        dir: &Path,
        recursive: bool,
        algorithm: &str,
    ) -> Result<ManifestReady, String> {
        let algorithm = if algorithm.eq_ignore_ascii_case("auto") {
            "sha256"
        } else {
            algorithm
        };
        if let Some(index) = self
            .manifest
            .algorithms
            .iter()
            .position(|a| a.eq_ignore_ascii_case(algorithm))
        {
            self.manifest.algorithm_index = index;
        }
        self.manifest.recursive = recursive;
        let start = Instant::now();
        let manifest =
            generate_manifest(dir, algorithm, recursive).map_err(|err| err.to_string())?;
        let ready = ManifestReady::from_scan(manifest, dir.to_path_buf(), start.elapsed());
        self.manifest.selected_dir = Some(dir.to_path_buf());
        self.manifest.set_ready_state(ready.clone());
        self.active_tab = ActiveTab::DirectoryManifest;
        Ok(ready)
    }

    fn enable_snapshot(&mut self, options: SnapshotOptions) {
        let (png_parent, stem, json_path) = resolve_snapshot_targets(&options.path);
        if let Err(err) = fs::create_dir_all(&png_parent) {
            eprintln!(
                "Failed to prepare snapshot directory {}: {err}",
                png_parent.display()
            );
        }

        self.snapshot_capture = Some(SnapshotCaptureState {
            json_path,
            states: Vec::new(),
        });
        self.snapshot_config = Some(options.clone());

        let mut jobs = VecDeque::new();
        match options.preset {
            SnapshotPreset::Default => {
                let width = options.width;
                let height = options.height;
                jobs.push_back(SnapshotRequest::new(
                    png_parent.join(format!("{stem}-file.png")),
                    width,
                    height,
                    SnapshotScenario::FileDefault,
                ));
                jobs.push_back(SnapshotRequest::new(
                    png_parent.join(format!("{stem}-manifest.png")),
                    width,
                    height,
                    SnapshotScenario::ManifestSummary,
                ));
            }
            SnapshotPreset::Readme => {
                let output_dir = png_parent.clone();
                if let Err(err) = fs::create_dir_all(&output_dir) {
                    eprintln!("Failed to create snapshot directory: {err}");
                }
                let width = if options.width == SNAPSHOT_DEFAULT_WIDTH {
                    README_SNAPSHOT_WIDTH
                } else {
                    options.width
                };
                let height = if options.height == SNAPSHOT_DEFAULT_HEIGHT {
                    README_SNAPSHOT_HEIGHT
                } else {
                    options.height
                };
                let scenarios = [
                    (SnapshotScenario::FileDefault, "gui-main.png"),
                    (SnapshotScenario::FileAlgorithmDropdown, "gui-algorithm.png"),
                    (SnapshotScenario::FileMatch, "gui-match.png"),
                    (SnapshotScenario::FileMismatch, "gui-mismatch.png"),
                    (SnapshotScenario::FileHighContrast, "gui-high-contrast.png"),
                    (SnapshotScenario::ManifestSummary, "gui-folder-scan.png"),
                    (SnapshotScenario::ManifestDetails, "gui-folder-details.png"),
                ];
                for (scenario, filename) in scenarios {
                    jobs.push_back(SnapshotRequest::new(
                        output_dir.join(filename),
                        width,
                        height,
                        scenario,
                    ));
                }
            }
        }
        self.snapshot_jobs = jobs;
    }

    fn record_snapshot_state(&mut self, scenario: SnapshotScenario) {
        let config = match self.snapshot_config.clone() {
            Some(cfg) => cfg,
            None => return,
        };
        let state = SnapshotState::from_app(self, &config, Some(scenario.label().to_string()));
        if let Some(capture) = self.snapshot_capture.as_mut() {
            capture.states.push(state);
        }
    }

    fn finalize_snapshot_capture(&mut self) {
        if let Some(capture) = self.snapshot_capture.take() {
            if capture.states.is_empty() {
                self.snapshot_config = None;
                return;
            }
            let document = SnapshotDocument {
                version: "1.0.0".to_owned(),
                platform: env::consts::OS.to_owned(),
                captures: capture.states,
            };
            if let Err(err) = write_snapshot_json(&capture.json_path, &document) {
                eprintln!(
                    "Failed to write snapshot state JSON {}: {err}",
                    capture.json_path.display()
                );
            } else {
                println!("Snapshot state written to {}", capture.json_path.display());
            }
        }
        self.snapshot_config = None;
    }
}

#[derive(Default)]
struct AlgorithmChoice {
    algorithms: Vec<String>,
    selected_index: usize,
}

impl AlgorithmChoice {
    fn current_label(&self) -> &str {
        self.algorithms
            .get(self.selected_index)
            .map(|s| s.as_str())
            .unwrap_or("auto")
    }
}

#[derive(Debug, Clone)]
struct StatusMessage {
    text: String,
    kind: StatusKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum StatusKind {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl StatusKind {
    fn as_str(&self) -> &'static str {
        match self {
            StatusKind::Info => "info",
            StatusKind::Success => "success",
            StatusKind::Warning => "warning",
            StatusKind::Error => "error",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
enum ThemePreset {
    SoftLight,
    Slate,
    HighContrast,
}

impl ThemePreset {
    fn name(&self) -> &'static str {
        match self {
            ThemePreset::SoftLight => "Soft Light",
            ThemePreset::Slate => "Slate",
            ThemePreset::HighContrast => "High Contrast",
        }
    }

    fn visuals(&self) -> egui::Visuals {
        match self {
            ThemePreset::SoftLight => {
                let mut visuals = egui::Visuals::light();
                visuals.override_text_color = Some(Color32::from_rgb(40, 44, 54));
                visuals.panel_fill = Color32::from_rgb(222, 226, 233);
                visuals.window_fill = Color32::from_rgb(214, 218, 226);
                visuals.extreme_bg_color = Color32::from_rgb(206, 210, 219);
                visuals.hyperlink_color = Color32::from_rgb(52, 109, 170);
                visuals.widgets.inactive.bg_fill = Color32::from_rgb(209, 214, 224);
                visuals.widgets.inactive.fg_stroke.color = Color32::from_rgb(63, 68, 82);
                visuals.widgets.hovered.bg_fill = Color32::from_rgb(198, 204, 216);
                visuals.widgets.active.bg_fill = Color32::from_rgb(188, 195, 209);
                visuals.selection.bg_fill = Color32::from_rgb(76, 120, 184);
                visuals
            }
            ThemePreset::Slate => {
                let mut visuals = egui::Visuals::dark();
                visuals.override_text_color = Some(Color32::from_rgb(232, 236, 244));
                visuals.panel_fill = Color32::from_rgb(30, 35, 43);
                visuals.window_fill = Color32::from_rgb(26, 30, 38);
                visuals.extreme_bg_color = Color32::from_rgb(22, 26, 32);
                visuals.hyperlink_color = Color32::from_rgb(118, 176, 246);
                visuals.widgets.inactive.bg_fill = Color32::from_rgb(44, 50, 60);
                visuals.widgets.inactive.fg_stroke.color = Color32::from_rgb(226, 230, 238);
                visuals.widgets.hovered.bg_fill = Color32::from_rgb(56, 62, 74);
                visuals.widgets.active.bg_fill = Color32::from_rgb(64, 72, 86);
                visuals.selection.bg_fill = Color32::from_rgb(86, 142, 244);
                visuals
            }
            ThemePreset::HighContrast => egui::Visuals::dark(),
        }
    }
}

struct ThemeChoice {
    presets: Vec<ThemePreset>,
    selected_index: usize,
}

impl Default for ThemeChoice {
    fn default() -> Self {
        Self {
            presets: vec![ThemePreset::Slate, ThemePreset::HighContrast],
            selected_index: 0,
        }
    }
}

impl ThemeChoice {
    fn selected_theme(&self) -> ThemePreset {
        self.presets[self.selected_index]
    }
}

struct ManifestView {
    selected_dir: Option<PathBuf>,
    recursive: bool,
    algorithm_index: usize,
    algorithms: Vec<String>,
    export_format_index: usize,
    state: ManifestState,
    banner: Option<ManifestBanner>,
    job_receiver: Option<Receiver<ManifestJobResult>>,
    telemetry_enabled: bool,
    show_details: bool,
    force_embed_details: bool,
}

impl ManifestView {
    fn new() -> Self {
        let algorithms: Vec<String> = supported_algorithms()
            .iter()
            .map(|s| s.to_string())
            .collect();
        let algorithm_index = algorithms
            .iter()
            .position(|a| a.eq_ignore_ascii_case("sha256"))
            .unwrap_or(0);
        Self {
            selected_dir: None,
            recursive: true,
            algorithm_index,
            algorithms,
            export_format_index: 0,
            state: ManifestState::Empty,
            banner: None,
            job_receiver: None,
            telemetry_enabled: true,
            show_details: false,
            force_embed_details: false,
        }
    }

    fn poll_job(&mut self) {
        if let Some(rx) = &self.job_receiver {
            match rx.try_recv() {
                Ok(result) => {
                    match result {
                        ManifestJobResult::Success(data) => {
                            self.set_ready_state(*data);
                        }
                        ManifestJobResult::Failure { message, details } => {
                            self.state = ManifestState::Error(ManifestError { message, details });
                            self.banner = None;
                        }
                    }
                    self.job_receiver = None;
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    self.state = ManifestState::Error(ManifestError {
                        message: "Manifest operation was interrupted.".to_owned(),
                        details: None,
                    });
                    self.job_receiver = None;
                }
            }
        }
    }

    fn set_ready_state(&mut self, data: ManifestReady) {
        if self.telemetry_enabled {
            self.log_telemetry(&data);
        }
        self.banner = Some(self.banner_for_ready(&data));
        self.state = ManifestState::Ready(Box::new(data));
        self.job_receiver = None;
    }

    fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, scale: f32) {
        self.handle_shortcuts(ctx);

        with_constrained_ui(ui, MAX_CONTENT_WIDTH * scale, |ui| {
            ui.heading("Scan a folder");
            ui.add_space(BASE_SPACING * scale);

            self.render_controls(ctx, ui, scale);

            ui.add_space(LARGE_SPACING * scale);

            if let Some(banner) = &self.banner {
                self.render_banner(ui, banner);
                ui.add_space(BASE_SPACING);
            }

            match self.state.clone() {
                ManifestState::Empty => self.render_empty(ui),
                ManifestState::Loading(op) => self.render_loading(ui, op),
                ManifestState::Ready(data) => {
                    self.render_ready(ctx, ui, data.as_ref(), scale);
                }
                ManifestState::Error(err) => self.render_error(ui, &err),
            }
        });
    }

    fn render_controls(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, scale: f32) {
        let spacing_scale = scale;
        let padding = margin_symmetric(16.0 * spacing_scale, 14.0 * spacing_scale);

        egui::Frame::group(&ctx.style())
            .fill(ui.visuals().extreme_bg_color)
            .stroke(egui::Stroke::new(
                1.0,
                ui.visuals().widgets.inactive.bg_fill.gamma_multiply(0.6),
            ))
            .inner_margin(padding)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.spacing_mut().item_spacing =
                    vec2(BASE_SPACING * spacing_scale, BASE_SPACING * spacing_scale);
                let primary_size = vec2(ACTION_BUTTON_MIN_WIDTH * spacing_scale, BUTTON_HEIGHT);
                let folder_display = self
                    .selected_dir
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "No folder selected".to_owned());

                ui.horizontal_wrapped(|row| {
                    row.label("Folder:");
                    row.add_space(BASE_SPACING * spacing_scale);
                    let button_width = SECONDARY_BUTTON_MIN_WIDTH * spacing_scale;
                    let path_width =
                        (row.available_width() - button_width - BASE_SPACING * spacing_scale)
                            .max(240.0);
                    row.add_sized(
                        vec2(path_width, 0.0),
                        egui::Label::new(RichText::new(folder_display.clone()).monospace()).wrap(),
                    );
                    row.add_space(BASE_SPACING * spacing_scale);
                    if row
                        .add_sized(
                            vec2(button_width, BUTTON_HEIGHT),
                            egui::Button::new("Choose folder…"),
                        )
                        .clicked()
                    {
                        if let Some(path) = dialog::pick_folder() {
                            self.selected_dir = Some(path);
                        }
                    }
                });

                ui.add_space(BASE_SPACING * spacing_scale);
                ui.horizontal_wrapped(|row| {
                    row.label("Algorithm:");
                    row.add_space(BASE_SPACING * spacing_scale);
                    let dropdown_width = 180.0 * spacing_scale;
                    egui::ComboBox::from_id_salt("manifest_algorithm_selector")
                        .width(dropdown_width)
                        .selected_text(&self.algorithms[self.algorithm_index])
                        .show_ui(row, |combo| {
                            for (idx, label) in self.algorithms.iter().enumerate() {
                                combo.selectable_value(&mut self.algorithm_index, idx, label);
                            }
                        });
                    row.add_space(BASE_SPACING * spacing_scale);
                    row.checkbox(&mut self.recursive, "Include subfolders (Alt+R)");
                });

                ui.add_space(BASE_SPACING * spacing_scale);
                let loading = matches!(self.state, ManifestState::Loading(_));
                let scan_enabled = self.selected_dir.is_some() && !loading;
                let export_enabled = matches!(self.state, ManifestState::Ready(_)) && !loading;

                let accent_color = Color32::from_rgb(30, 102, 245);
                let scan_button = |ui: &mut egui::Ui, label: &str| {
                    ui.add_enabled(
                        scan_enabled,
                        egui::Button::new(label)
                            .min_size(primary_size)
                            .fill(accent_color)
                            .stroke(egui::Stroke::NONE),
                    )
                };
                let secondary_button = |ui: &mut egui::Ui, label: &str, enabled: bool| {
                    ui.add_enabled(
                        enabled,
                        egui::Button::new(label).min_size(vec2(
                            SECONDARY_BUTTON_MIN_WIDTH * spacing_scale,
                            BUTTON_HEIGHT,
                        )),
                    )
                };

                ui.horizontal_wrapped(|row| {
                    if scan_button(row, "Scan").clicked() {
                        self.trigger_scan(ctx);
                    }
                    row.add_space(BASE_SPACING * spacing_scale);
                    if secondary_button(row, "Export manifest…", export_enabled).clicked() {
                        self.trigger_export(ctx);
                    }
                    row.add_space(BASE_SPACING * spacing_scale);
                    if secondary_button(row, "Verify manifest…", !loading).clicked() {
                        self.trigger_verify(ctx);
                    }
                });

                ui.add_space(BASE_SPACING * spacing_scale);
                ui.horizontal_wrapped(|row| {
                    row.label("Format:");
                    row.add_space(BASE_SPACING * spacing_scale);
                    let dropdown_width = 180.0 * spacing_scale;
                    egui::ComboBox::from_id_salt("manifest_format_selector")
                        .width(dropdown_width)
                        .selected_text(Self::format_label(self.export_format_index))
                        .show_ui(row, |combo| {
                            for (idx, label) in Self::format_labels().iter().enumerate() {
                                combo.selectable_value(&mut self.export_format_index, idx, *label);
                            }
                        });
                });
            });
    }
    fn render_empty(&self, ui: &mut egui::Ui) {
        EguiFrame::group(ui.style())
            .fill(Color32::from_rgb(34, 38, 46))
            .stroke(egui::Stroke::new(1.0, Color32::from_rgb(70, 78, 92)))
            .inner_margin(margin_all(LARGE_SPACING))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(LARGE_SPACING);
                    ui.label(RichText::new("📁 Ready to scan").size(20.0));
                    ui.add_space(BASE_SPACING * 1.5);
                    ui.label("Drop a folder here or pick one with 'Choose folder…'.");
                    ui.label("Turn on \"Include subfolders\" if you need the whole tree.");
                    ui.add_space(BASE_SPACING * 2.0);
                    ui.label(RichText::new("Shortcuts").strong());
                    ui.label("Enter — Scan | ⌘/Ctrl+S — Save list | ⌘/Ctrl+O — Check list");
                    ui.add_space(LARGE_SPACING);
                });
            });
    }

    fn render_loading(&self, ui: &mut egui::Ui, op: ManifestOperation) {
        let message = match op {
            ManifestOperation::Scan => "Scanning folder…",
            ManifestOperation::Verify => "Verifying manifest…",
        };
        ui.label(RichText::new(message).strong());
        ui.add_space(BASE_SPACING);
        self.render_skeleton(ui);
    }

    fn render_ready(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        data: &ManifestReady,
        scale: f32,
    ) {
        self.render_summary(ctx, ui, data, scale);
    }

    fn render_error(&self, ui: &mut egui::Ui, err: &ManifestError) {
        EguiFrame::new()
            .fill(Color32::from_rgb(90, 34, 34))
            .stroke(egui::Stroke::new(2.0, Color32::from_rgb(200, 80, 80)))
            .inner_margin(margin_all(LARGE_SPACING))
            .show(ui, |ui| {
                ui.label(RichText::new("There was a problem").strong());
                ui.label(&err.message);
                if let Some(details) = &err.details {
                    ui.add_space(BASE_SPACING);
                    ui.label(RichText::new("Details:").strong());
                    ui.monospace(details);
                }
            });
    }

    fn render_banner(&self, ui: &mut egui::Ui, banner: &ManifestBanner) {
        let (fill, stroke) = match banner.kind {
            StatusKind::Success => (
                Color32::from_rgb(32, 94, 61),
                Color32::from_rgb(70, 140, 100),
            ),
            StatusKind::Warning => (
                Color32::from_rgb(125, 92, 18),
                Color32::from_rgb(190, 140, 40),
            ),
            StatusKind::Error => (
                Color32::from_rgb(120, 36, 32),
                Color32::from_rgb(190, 80, 70),
            ),
            StatusKind::Info => (
                Color32::from_rgb(56, 62, 90),
                Color32::from_rgb(96, 110, 150),
            ),
        };
        EguiFrame::new()
            .fill(fill)
            .stroke(egui::Stroke::new(1.0, stroke))
            .inner_margin(margin_all(BASE_SPACING))
            .show(ui, |ui| {
                ui.label(RichText::new(&banner.text).strong());
            });
    }

    fn render_skeleton(&self, ui: &mut egui::Ui) {
        let rows = 5;
        EguiFrame::new()
            .fill(Color32::from_rgb(30, 34, 40))
            .inner_margin(margin_all(BASE_SPACING))
            .show(ui, |ui| {
                for _ in 0..rows {
                    ui.horizontal(|ui| {
                        self.skeleton_rect(ui, 220.0);
                        self.skeleton_rect(ui, 260.0);
                        self.skeleton_rect(ui, 120.0);
                    });
                    ui.add_space(4.0);
                }
            });
    }

    fn skeleton_rect(&self, ui: &mut egui::Ui, width: f32) {
        let rect = ui.available_rect_before_wrap();
        let rect = egui::Rect::from_min_size(rect.min, vec2(width, TABLE_ROW_HEIGHT * 0.6));
        let painter = ui.painter();
        painter.rect_filled(rect, 2.0, Color32::from_rgb(50, 56, 64));
        painter.rect_stroke(
            rect,
            2.0,
            egui::Stroke::new(1.0, Color32::from_rgb(66, 72, 80)),
            StrokeKind::Outside,
        );
        ui.allocate_space(rect.size());
    }

    fn render_summary(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        data: &ManifestReady,
        scale: f32,
    ) {
        let summary = &data.summary;
        ui.vertical(|column| {
            column.horizontal(|row| {
                row.label(RichText::new(format!("Total: {}", summary.total)).strong());
                row.separator();
                row.colored_label(
                    Color32::from_rgb(96, 130, 200),
                    format!("Recorded: {}", summary.recorded),
                );
                row.separator();
                row.colored_label(
                    Color32::from_rgb(40, 180, 120),
                    format!("Matched: {}", summary.matched),
                );
                row.separator();
                row.colored_label(
                    Color32::from_rgb(230, 170, 60),
                    format!("Mismatched: {}", summary.mismatched),
                );
                row.separator();
                row.colored_label(
                    Color32::from_rgb(230, 150, 50),
                    format!("Missing: {}", summary.missing),
                );
                row.separator();
                row.colored_label(
                    Color32::from_rgb(220, 90, 80),
                    format!("Extra: {}", summary.extra),
                );
                row.add_space(BASE_SPACING * 2.0);
                let details_button = egui::Button::new("View details…")
                    .min_size(vec2(140.0 * scale, BUTTON_HEIGHT))
                    .fill(Color32::from_rgb(30, 102, 245));
                if row.add(details_button).clicked() {
                    self.show_details = true;
                }
            });
            column.add_space(BASE_SPACING * 0.5);
            column.horizontal_wrapped(|row| {
                row.label(format!(
                    "Algorithm: {} | Duration: {:.0} ms",
                    data.manifest.algorithm,
                    data.duration.as_secs_f64() * 1000.0
                ));
                if let Some(path) = &data.manifest_path {
                    row.separator();
                    row.add(
                        egui::Label::new(
                            RichText::new(format!("Saved list: {}", path.display()))
                                .text_style(TextStyle::Small),
                        )
                        .wrap(),
                    );
                }
                row.separator();
                let root_text = format!("Root: {}", data.root.display());
                let root_label = egui::Label::new(RichText::new(root_text.clone()).monospace())
                    .wrap()
                    .sense(egui::Sense::click());
                let response = row.add(root_label);
                if response.clicked() {
                    ctx.output_mut(|out| {
                        out.commands
                            .push(OutputCommand::CopyText(root_text.clone()))
                    });
                }
                response.on_hover_text("Click to copy full root path");
            });
        });
        if self.show_details {
            if self.force_embed_details {
                self.render_details_embedded(ctx, ui, data, scale);
            } else {
                self.render_details_window(ctx, data, scale);
            }
        }
    }

    fn render_details_embedded(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        data: &ManifestReady,
        scale: f32,
    ) {
        egui::Frame::group(&ctx.style())
            .fill(ui.visuals().extreme_bg_color)
            .stroke(egui::Stroke::new(
                1.0,
                ui.visuals().widgets.inactive.bg_fill.gamma_multiply(0.6),
            ))
            .inner_margin(margin_all(BASE_SPACING * scale))
            .show(ui, |panel| {
                panel.set_width(panel.available_width());
                panel.vertical(|col| {
                    col.horizontal(|row| {
                        row.label(RichText::new("Directory manifest details").strong());
                        row.with_layout(Layout::right_to_left(Align::Center), |controls| {
                            if controls.button("Close").clicked() {
                                self.show_details = false;
                            }
                        });
                    });
                    col.add_space(BASE_SPACING * scale);
                    col.label(RichText::new(format!("Root: {}", data.root.display())).monospace());
                    col.add_space(BASE_SPACING * scale);
                    self.render_table(col, data, scale);
                });
            });
        self.force_embed_details = false;
    }

    fn render_details_window(&mut self, ctx: &egui::Context, data: &ManifestReady, scale: f32) {
        let viewport_id = ViewportId::from_hash_of("directory_manifest_details");
        let mut viewport_should_close = false;
        ctx.show_viewport_immediate(
            viewport_id,
            ViewportBuilder::default()
                .with_title("Directory manifest details")
                .with_inner_size([900.0, 560.0])
                .with_min_inner_size([700.0, 420.0])
                .with_resizable(true),
            |ctx, _class| {
                viewport_should_close = ctx.input(|i| i.viewport().close_requested());
                egui::TopBottomPanel::top("details_header").show(ctx, |ui| {
                    ui.add_space(BASE_SPACING * scale);
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("Root: {}", data.root.display())).monospace(),
                        );
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui.button("Close").clicked() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        });
                    });
                    ui.add_space(BASE_SPACING * scale);
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    self.render_table(ui, data, scale);
                });
            },
        );
        if viewport_should_close {
            self.show_details = false;
        }
    }

    fn render_table(&self, ui: &mut egui::Ui, data: &ManifestReady, scale: f32) {
        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .show(ui, |scroll| {
                let total_width = scroll.available_width().max(480.0);
                let path_width = (total_width * 0.38).max(200.0);
                let hash_width = (total_width * 0.26).max(160.0);
                let state_width =
                    (total_width - path_width - 2.0 * hash_width - 2.0 * BASE_SPACING).max(120.0);

                scroll.vertical(|list| {
                    list.set_width(total_width);
                    list.spacing_mut().item_spacing = vec2(BASE_SPACING * 0.5, BASE_SPACING * 0.75);
                    list.set_min_height(320.0 * scale);

                    list.horizontal(|row| {
                        row.set_width(total_width);
                        row.spacing_mut().item_spacing =
                            vec2(BASE_SPACING * 0.5, BASE_SPACING * 0.5);
                        row.add_sized(
                            vec2(path_width, 0.0),
                            egui::Label::new(RichText::new("Path").strong()),
                        );
                        row.add_sized(
                            vec2(hash_width, 0.0),
                            egui::Label::new(RichText::new("Expected").strong()),
                        );
                        row.add_sized(
                            vec2(hash_width, 0.0),
                            egui::Label::new(RichText::new("Actual").strong()),
                        );
                        row.add_sized(
                            vec2(state_width, 0.0),
                            egui::Label::new(RichText::new("State").strong()),
                        );
                    });

                    list.add_space(BASE_SPACING * 0.75);

                    for row in &data.entries {
                        list.horizontal(|row_ui| {
                            row_ui.set_width(total_width);
                            row_ui.spacing_mut().item_spacing =
                                vec2(BASE_SPACING * 0.5, BASE_SPACING * 0.5);

                            row_ui.add_sized(
                                vec2(path_width, 0.0),
                                egui::Label::new(RichText::new(&row.path).monospace().size(14.0))
                                    .wrap(),
                            );

                            let expected_label = match row.expected.as_ref() {
                                Some(expected) => RichText::new(expected).monospace(),
                                None => RichText::new("—").italics(),
                            };
                            row_ui.add_sized(
                                vec2(hash_width, 0.0),
                                egui::Label::new(expected_label).wrap(),
                            );

                            let actual_label = match row.actual.as_ref() {
                                Some(actual) => RichText::new(actual).monospace(),
                                None => RichText::new("—").italics(),
                            };
                            row_ui.add_sized(
                                vec2(hash_width, 0.0),
                                egui::Label::new(actual_label).wrap(),
                            );

                            let (text, color) = row.state.label_and_color();
                            row_ui.add_sized(
                                vec2(state_width, 0.0),
                                egui::Label::new(text.color(color)).wrap(),
                            );
                        });
                    }
                });
            });
    }

    fn trigger_scan(&mut self, ctx: &egui::Context) {
        let Some(dir) = self.selected_dir.clone() else {
            self.banner = Some(ManifestBanner::warning("Pick a folder before scanning."));
            return;
        };

        let algorithm = self.algorithms[self.algorithm_index].clone();
        let recursive = self.recursive;
        let (tx, rx) = mpsc::channel();
        self.state = ManifestState::Loading(ManifestOperation::Scan);
        self.banner = None;
        self.job_receiver = Some(rx);

        thread::spawn(move || {
            let start = Instant::now();
            let result = generate_manifest(&dir, &algorithm, recursive);
            match result {
                Ok(manifest) => {
                    let duration = start.elapsed();
                    let ready = ManifestReady::from_scan(manifest, dir.clone(), duration);
                    let _ = tx.send(ManifestJobResult::Success(Box::new(ready)));
                }
                Err(err) => {
                    let _ = tx.send(ManifestJobResult::Failure {
                        message: "Failed to scan folder.".to_owned(),
                        details: Some(err.to_string()),
                    });
                }
            }
        });
        ctx.request_repaint();
    }

    fn trigger_export(&mut self, ctx: &egui::Context) {
        let ManifestState::Ready(data) = &self.state else {
            self.banner = Some(ManifestBanner::warning(
                "Nothing to export yet. Run a scan or verification first.",
            ));
            return;
        };

        let format = Self::format_from_index(self.export_format_index);
        let suggested_name = match format {
            ManifestFormat::Json => "hash-checker-manifest.json",
            ManifestFormat::Csv => "hash-checker-manifest.csv",
            ManifestFormat::Plain => "hash-checker-manifest.txt",
        };

        if let Some(path) = dialog::save_file_with_name(suggested_name) {
            let file_result = File::create(&path);
            match file_result {
                Ok(file) => {
                    if let Err(err) = write_manifest(&data.manifest, format, file) {
                        self.banner = Some(ManifestBanner::error(&format!(
                            "Failed to write manifest: {err}"
                        )));
                    } else {
                        self.banner = Some(ManifestBanner::success(&format!(
                            "Saved manifest to {}",
                            path.display()
                        )));
                    }
                }
                Err(err) => {
                    self.banner = Some(ManifestBanner::error(&format!(
                        "Failed to create file: {err}"
                    )));
                }
            }
        }
        ctx.request_repaint();
    }

    fn trigger_verify(&mut self, ctx: &egui::Context) {
        if matches!(self.state, ManifestState::Loading(_)) {
            return;
        }
        let manifest_path = match dialog::pick_manifest_file() {
            Some(path) => path,
            None => return,
        };

        let selected_root = self.selected_dir.clone();
        let (tx, rx) = mpsc::channel();
        self.state = ManifestState::Loading(ManifestOperation::Verify);
        self.banner = None;
        self.job_receiver = Some(rx);

        thread::spawn(move || {
            let format =
                detect_format_from_extension(&manifest_path).unwrap_or(ManifestFormat::Json);
            let file = File::open(&manifest_path);
            let manifest_result = file.and_then(|f| {
                read_manifest(f, format).map_err(|e| std::io::Error::other(e.to_string()))
            });

            let manifest = match manifest_result {
                Ok(manifest) => manifest,
                Err(err) => {
                    let _ = tx.send(ManifestJobResult::Failure {
                        message: "Failed to read manifest file.".to_owned(),
                        details: Some(err.to_string()),
                    });
                    return;
                }
            };

            let root = match resolve_root(&manifest, selected_root.as_deref(), &manifest_path) {
                root if root.exists() => root,
                other => {
                    let _ = tx.send(ManifestJobResult::Failure {
                        message: "Saved list root not found.".to_owned(),
                        details: Some(other.display().to_string()),
                    });
                    return;
                }
            };

            let start = Instant::now();
            match verify_manifest(&manifest, &root) {
                Ok(report) => {
                    let duration = start.elapsed();
                    let ready = ManifestReady::from_verify(
                        manifest,
                        Some(manifest_path),
                        root,
                        report,
                        duration,
                    );
                    match ready {
                        Ok(ready) => {
                            let _ = tx.send(ManifestJobResult::Success(Box::new(ready)));
                        }
                        Err(err) => {
                            let _ = tx.send(ManifestJobResult::Failure {
                                message: "Verification failed while preparing results.".to_owned(),
                                details: Some(err),
                            });
                        }
                    }
                }
                Err(err) => {
                    let _ = tx.send(ManifestJobResult::Failure {
                        message: "Verification failed.".to_owned(),
                        details: Some(err.to_string()),
                    });
                }
            }
        });
        ctx.request_repaint();
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        let trigger_scan = ctx.input_mut(|input| {
            input.consume_shortcut(&KeyboardShortcut::new(Modifiers::NONE, Key::Enter))
        });
        if trigger_scan {
            self.trigger_scan(ctx);
        }
        let trigger_export = ctx.input_mut(|input| {
            input.consume_shortcut(&KeyboardShortcut::new(Modifiers::COMMAND, Key::S))
                || input.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::S))
        });
        if trigger_export {
            self.trigger_export(ctx);
        }
        let trigger_verify = ctx.input_mut(|input| {
            input.consume_shortcut(&KeyboardShortcut::new(Modifiers::COMMAND, Key::O))
                || input.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::O))
        });
        if trigger_verify {
            self.trigger_verify(ctx);
        }
        let toggle_recursive = ctx.input_mut(|input| {
            input.consume_shortcut(&KeyboardShortcut::new(Modifiers::ALT, Key::R))
        });
        if toggle_recursive {
            self.recursive = !self.recursive;
        }
    }

    fn banner_for_ready(&self, data: &ManifestReady) -> ManifestBanner {
        match data.operation {
            ManifestOperation::Scan => ManifestBanner::info(&format!(
                "Recorded {} items in {} ms. Use Check list to compare with a saved file.",
                data.summary.recorded,
                (data.duration.as_secs_f64() * 1000.0).round()
            )),
            ManifestOperation::Verify => {
                if data.summary.mismatched == 0
                    && data.summary.missing == 0
                    && data.summary.extra == 0
                {
                    ManifestBanner::success(&format!(
                        "Folder matches saved list – all {} items ok ({} ms).",
                        data.summary.total,
                        (data.duration.as_secs_f64() * 1000.0).round()
                    ))
                } else {
                    ManifestBanner::warning(&format!(
                        "Folder check found issues ({} mismatch, {} missing, {} extra).",
                        data.summary.mismatched, data.summary.missing, data.summary.extra
                    ))
                }
            }
        }
    }

    fn log_telemetry(&self, data: &ManifestReady) {
        if let Err(err) = fs::create_dir_all(TELEMETRY_DIR) {
            eprintln!("Failed to create telemetry directory: {err}");
            return;
        }
        let path = Path::new(TELEMETRY_DIR).join(TELEMETRY_FILE);
        let mut file = match OpenOptions::new().create(true).append(true).open(&path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open telemetry file: {err}");
                return;
            }
        };
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let line = format!(
            "{timestamp},operation={:?},duration_ms={:.2},total={},recorded={},matched={},mismatched={},missing={},extra={}\n",
            data.operation,
            data.duration.as_secs_f64() * 1000.0,
            data.summary.total,
            data.summary.recorded,
            data.summary.matched,
            data.summary.mismatched,
            data.summary.missing,
            data.summary.extra
        );
        let _ = file.write_all(line.as_bytes());
    }

    fn format_from_index(index: usize) -> ManifestFormat {
        match index {
            0 => ManifestFormat::Json,
            1 => ManifestFormat::Csv,
            _ => ManifestFormat::Plain,
        }
    }

    fn format_label(index: usize) -> &'static str {
        Self::format_labels()
            .get(index)
            .copied()
            .unwrap_or("JSON (.json)")
    }

    fn format_labels() -> &'static [&'static str] {
        &["JSON (.json)", "CSV (.csv)", "TXT (.txt)"]
    }
}

#[derive(Clone)]
struct ManifestReady {
    manifest: Manifest,
    manifest_path: Option<PathBuf>,
    root: PathBuf,
    entries: Vec<ManifestRow>,
    summary: ManifestSummary,
    operation: ManifestOperation,
    duration: Duration,
}

impl ManifestReady {
    fn from_scan(manifest: Manifest, root: PathBuf, duration: Duration) -> Self {
        let entries = manifest
            .entries
            .iter()
            .map(ManifestRow::from_scan_entry)
            .collect::<Vec<_>>();
        let recorded = entries.len();
        let summary = ManifestSummary {
            total: recorded,
            recorded,
            matched: 0,
            mismatched: 0,
            missing: 0,
            extra: 0,
        };
        Self {
            manifest,
            manifest_path: None,
            root,
            entries,
            summary,
            operation: ManifestOperation::Scan,
            duration,
        }
    }

    fn from_verify(
        manifest: Manifest,
        manifest_path: Option<PathBuf>,
        root: PathBuf,
        report: VerificationReport,
        duration: Duration,
    ) -> Result<Self, String> {
        let mut mismatch_map = std::collections::HashMap::new();
        for mismatch in &report.mismatched {
            mismatch_map.insert(mismatch.entry.path.clone(), mismatch.actual.clone());
        }
        let missing_set: std::collections::HashSet<_> = report
            .missing
            .iter()
            .map(|entry| entry.path.clone())
            .collect();

        let mut entries = Vec::new();
        for entry in &manifest.entries {
            if let Some(actual) = mismatch_map.get(&entry.path) {
                entries.push(ManifestRow::mismatch(entry, actual.clone()));
            } else if missing_set.contains(&entry.path) {
                entries.push(ManifestRow::missing(entry));
            } else {
                entries.push(ManifestRow::from_verified_entry(entry));
            }
        }

        for extra in &report.extra {
            let full_path = root.join(extra);
            let actual_hash = compute_hash(&full_path, &manifest.algorithm).ok();
            entries.push(ManifestRow::extra(extra.clone(), actual_hash));
        }

        let recorded = manifest.entries.len();
        let summary = ManifestSummary {
            total: recorded + report.extra.len(),
            recorded,
            matched: report.matched,
            mismatched: report.mismatched.len(),
            missing: report.missing.len(),
            extra: report.extra.len(),
        };

        Ok(Self {
            manifest,
            manifest_path,
            root,
            entries,
            summary,
            operation: ManifestOperation::Verify,
            duration,
        })
    }
}

#[derive(Clone)]
struct ManifestRow {
    path: String,
    expected: Option<String>,
    actual: Option<String>,
    state: ManifestRowState,
}

impl ManifestRow {
    fn from_scan_entry(entry: &ManifestEntry) -> Self {
        Self {
            path: entry.path.clone(),
            expected: Some(entry.hash.clone()),
            actual: None,
            state: ManifestRowState::Pending,
        }
    }

    fn from_verified_entry(entry: &ManifestEntry) -> Self {
        Self {
            path: entry.path.clone(),
            expected: Some(entry.hash.clone()),
            actual: Some(entry.hash.clone()),
            state: ManifestRowState::Match,
        }
    }

    fn mismatch(entry: &ManifestEntry, actual: String) -> Self {
        Self {
            path: entry.path.clone(),
            expected: Some(entry.hash.clone()),
            actual: Some(actual),
            state: ManifestRowState::Mismatch,
        }
    }

    fn missing(entry: &ManifestEntry) -> Self {
        Self {
            path: entry.path.clone(),
            expected: Some(entry.hash.clone()),
            actual: None,
            state: ManifestRowState::Missing,
        }
    }

    fn extra(path: String, actual: Option<String>) -> Self {
        Self {
            path,
            expected: None,
            actual,
            state: ManifestRowState::Extra,
        }
    }
}

#[derive(Clone, Copy)]
enum ManifestRowState {
    Pending,
    Match,
    Mismatch,
    Missing,
    Extra,
}

impl ManifestRowState {
    fn label_and_color(self) -> (RichText, Color32) {
        match self {
            ManifestRowState::Pending => (
                RichText::new("Pending verify"),
                Color32::from_rgb(96, 130, 200),
            ),
            ManifestRowState::Match => (RichText::new("Match"), Color32::from_rgb(40, 180, 120)),
            ManifestRowState::Mismatch => {
                (RichText::new("Mismatch"), Color32::from_rgb(230, 170, 60))
            }
            ManifestRowState::Missing => {
                (RichText::new("Missing"), Color32::from_rgb(230, 150, 50))
            }
            ManifestRowState::Extra => (RichText::new("Extra"), Color32::from_rgb(220, 90, 80)),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            ManifestRowState::Pending => "pending",
            ManifestRowState::Match => "match",
            ManifestRowState::Mismatch => "mismatch",
            ManifestRowState::Missing => "missing",
            ManifestRowState::Extra => "extra",
        }
    }
}

#[derive(Clone, Serialize)]
struct ManifestSummary {
    total: usize,
    recorded: usize,
    matched: usize,
    mismatched: usize,
    missing: usize,
    extra: usize,
}

#[derive(Serialize)]
struct ManifestReportEntryData {
    path: String,
    state: &'static str,
    expected: Option<String>,
    actual: Option<String>,
}

#[derive(Serialize)]
struct ManifestReportData {
    operation: &'static str,
    algorithm: String,
    root: String,
    manifest_path: Option<String>,
    summary: ManifestSummary,
    duration_ms: f64,
    entries: Vec<ManifestReportEntryData>,
}

#[derive(Clone)]
struct ManifestBanner {
    text: String,
    kind: StatusKind,
}

impl ManifestBanner {
    fn success(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            kind: StatusKind::Success,
        }
    }

    fn warning(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            kind: StatusKind::Warning,
        }
    }

    fn error(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            kind: StatusKind::Error,
        }
    }

    fn info(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            kind: StatusKind::Info,
        }
    }
}

#[derive(Clone)]
struct ManifestError {
    message: String,
    details: Option<String>,
}

#[derive(Clone)]
enum ManifestState {
    Empty,
    Loading(ManifestOperation),
    Ready(Box<ManifestReady>),
    Error(ManifestError),
}

#[derive(Debug, Clone, Copy)]
enum ManifestOperation {
    Scan,
    Verify,
}

enum ManifestJobResult {
    Success(Box<ManifestReady>),
    Failure {
        message: String,
        details: Option<String>,
    },
}

#[derive(Clone)]
struct SnapshotOptions {
    path: PathBuf,
    width: u32,
    height: u32,
    preset: SnapshotPreset,
}

impl SnapshotOptions {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            width: SNAPSHOT_DEFAULT_WIDTH,
            height: SNAPSHOT_DEFAULT_HEIGHT,
            preset: SnapshotPreset::Default,
        }
    }
}

fn resolve_snapshot_targets(path: &Path) -> (PathBuf, String, PathBuf) {
    if path.extension().is_some() {
        let parent = path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));
        let stem = path
            .file_stem()
            .unwrap_or_else(|| OsStr::new("snapshot"))
            .to_string_lossy()
            .to_string();
        (parent, stem, path.to_path_buf())
    } else {
        let parent = path.to_path_buf();
        let stem = "snapshot".to_string();
        let json_path = parent.join(format!("{stem}.json"));
        (parent, stem, json_path)
    }
}

#[derive(Serialize)]
struct SnapshotWindowState {
    width: u32,
    height: u32,
    theme: String,
}

#[derive(Serialize)]
struct SnapshotNavigationState {
    active_tab: String,
    breadcrumb: Vec<String>,
}

#[derive(Serialize)]
struct SnapshotWidgetState {
    id: String,
    kind: String,
    value: Option<String>,
    selected: bool,
    enabled: bool,
    visible: bool,
}

#[derive(Serialize)]
struct SnapshotTelemetryState {
    scan_progress: f32,
    hash_mismatches: u32,
    elapsed_ms: u64,
}

#[derive(Serialize)]
struct SnapshotMetadata {
    app_version: String,
    git_commit: Option<String>,
    cli_args: Vec<String>,
}

#[derive(Serialize)]
struct SnapshotState {
    version: String,
    captured_at: String,
    platform: String,
    scenario: Option<String>,
    window: SnapshotWindowState,
    navigation: SnapshotNavigationState,
    widgets: Vec<SnapshotWidgetState>,
    telemetry: SnapshotTelemetryState,
    metadata: SnapshotMetadata,
}

impl SnapshotState {
    fn from_app(
        app: &HashCheckerApp,
        snapshot_opts: &SnapshotOptions,
        scenario: Option<String>,
    ) -> Self {
        let captured_at = OffsetDateTime::now_utc()
            .format(&Rfc3339)
            .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_owned());
        let theme_name = app.theme.selected_theme().name().to_string();
        let window = SnapshotWindowState {
            width: snapshot_opts.width,
            height: snapshot_opts.height,
            theme: theme_name,
        };
        let breadcrumb = app
            .manifest
            .selected_dir
            .as_ref()
            .map(|dir| vec![dir.display().to_string()])
            .unwrap_or_default();
        let navigation = SnapshotNavigationState {
            active_tab: app.active_tab.label().to_string(),
            breadcrumb,
        };

        let mut widgets = Vec::new();
        widgets.push(SnapshotWidgetState {
            id: "file_path_input".to_owned(),
            kind: "text".to_owned(),
            value: (!app.file_path.is_empty()).then_some(app.file_path.clone()),
            selected: matches!(app.active_tab, ActiveTab::File),
            enabled: true,
            visible: true,
        });
        widgets.push(SnapshotWidgetState {
            id: "algorithm_selector".to_owned(),
            kind: "dropdown".to_owned(),
            value: Some(app.algorithm.current_label().to_owned()),
            selected: false,
            enabled: true,
            visible: true,
        });
        if let Some(hash) = &app.computed_hash {
            widgets.push(SnapshotWidgetState {
                id: "computed_hash".to_owned(),
                kind: "label".to_owned(),
                value: Some(hash.clone()),
                selected: false,
                enabled: true,
                visible: true,
            });
        }
        if let Some(status) = &app.status {
            widgets.push(SnapshotWidgetState {
                id: "status_message".to_owned(),
                kind: format!("status::{}", status.kind.as_str()),
                value: Some(status.text.clone()),
                selected: false,
                enabled: true,
                visible: true,
            });
        }

        let (scan_progress, hash_mismatches, elapsed_ms) = match &app.manifest.state {
            ManifestState::Ready(data) => {
                let total = data.summary.total as f32;
                let progress = if total > 0.0 {
                    data.summary.recorded as f32 / total
                } else {
                    0.0
                };
                (
                    progress,
                    data.summary.mismatched as u32,
                    data.duration.as_millis() as u64,
                )
            }
            _ => (0.0, 0, 0),
        };

        let telemetry = SnapshotTelemetryState {
            scan_progress,
            hash_mismatches,
            elapsed_ms,
        };

        let metadata = SnapshotMetadata {
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            git_commit: option_env!("GIT_COMMIT_HASH").map(|s| s.to_string()),
            cli_args: std::env::args().skip(1).collect(),
        };

        SnapshotState {
            version: "1.0.0".to_string(),
            captured_at,
            platform: env::consts::OS.to_string(),
            scenario,
            window,
            navigation,
            widgets,
            telemetry,
            metadata,
        }
    }
}

#[derive(Serialize)]
struct SnapshotDocument {
    version: String,
    platform: String,
    captures: Vec<SnapshotState>,
}

struct SnapshotCaptureState {
    json_path: PathBuf,
    states: Vec<SnapshotState>,
}

#[derive(Clone)]
struct ManifestCliOptions {
    dir: PathBuf,
    recursive: bool,
    algorithm: Option<String>,
    report_path: Option<PathBuf>,
}

struct CliConfig {
    headless: bool,
    smoke_test: bool,
    snapshot: Option<SnapshotOptions>,
    manifest: Option<ManifestCliOptions>,
    capture_golden: Option<String>,
    compare_golden: Option<String>,
}

impl CliConfig {
    fn parse() -> Result<Self, String> {
        let mut args = env::args().skip(1);
        let mut config = CliConfig {
            headless: false,
            smoke_test: false,
            snapshot: None,
            manifest: None,
            capture_golden: None,
            compare_golden: None,
        };

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--headless" => config.headless = true,
                "--smoke-test" => config.smoke_test = true,
                "--snapshot" => {
                    let path = args
                        .next()
                        .ok_or_else(|| "Expected path after --snapshot".to_owned())?;
                    config.snapshot = Some(SnapshotOptions::new(PathBuf::from(path)));
                }
                "--snapshot-width" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "Expected value after --snapshot-width".to_owned())?;
                    let width = value
                        .parse::<u32>()
                        .map_err(|_| format!("Invalid snapshot width: {value}"))?;
                    let options = config
                        .snapshot
                        .as_mut()
                        .ok_or_else(|| "--snapshot-width must follow --snapshot".to_owned())?;
                    options.width = width.max(1);
                }
                "--snapshot-height" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "Expected value after --snapshot-height".to_owned())?;
                    let height = value
                        .parse::<u32>()
                        .map_err(|_| format!("Invalid snapshot height: {value}"))?;
                    let options = config
                        .snapshot
                        .as_mut()
                        .ok_or_else(|| "--snapshot-height must follow --snapshot".to_owned())?;
                    options.height = height.max(1);
                }
                "--snapshot-preset" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "Expected value after --snapshot-preset".to_owned())?;
                    let preset = match value.to_ascii_lowercase().as_str() {
                        "default" => SnapshotPreset::Default,
                        "readme" => SnapshotPreset::Readme,
                        other => {
                            return Err(format!("Unsupported snapshot preset: {other}"));
                        }
                    };
                    let options = config
                        .snapshot
                        .as_mut()
                        .ok_or_else(|| "--snapshot-preset must follow --snapshot".to_owned())?;
                    options.preset = preset;
                }
                "--manifest-dir" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "Expected directory path after --manifest-dir".to_owned())?;
                    let dir = PathBuf::from(value);
                    config.manifest = Some(ManifestCliOptions {
                        dir,
                        recursive: true,
                        algorithm: None,
                        report_path: None,
                    });
                }
                "--manifest-recursive" => {
                    let options = config.manifest.as_mut().ok_or_else(|| {
                        "--manifest-recursive must follow --manifest-dir".to_owned()
                    })?;
                    options.recursive = true;
                }
                "--manifest-non-recursive" => {
                    let options = config.manifest.as_mut().ok_or_else(|| {
                        "--manifest-non-recursive must follow --manifest-dir".to_owned()
                    })?;
                    options.recursive = false;
                }
                "--manifest-algorithm" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "Expected value after --manifest-algorithm".to_owned())?;
                    let options = config.manifest.as_mut().ok_or_else(|| {
                        "--manifest-algorithm must follow --manifest-dir".to_owned()
                    })?;
                    options.algorithm = Some(value);
                }
                "--manifest-report" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "Expected path after --manifest-report".to_owned())?;
                    let options = config
                        .manifest
                        .as_mut()
                        .ok_or_else(|| "--manifest-report must follow --manifest-dir".to_owned())?;
                    options.report_path = Some(PathBuf::from(value));
                }
                "--capture-golden" => {
                    let scenario = args
                        .next()
                        .ok_or_else(|| "Expected scenario after --capture-golden".to_owned())?;
                    if config.capture_golden.is_some() {
                        return Err("--capture-golden specified multiple times".to_owned());
                    }
                    config.capture_golden = Some(scenario);
                    config.headless = true;
                }
                "--compare-golden" => {
                    let scenario = args
                        .next()
                        .ok_or_else(|| "Expected scenario after --compare-golden".to_owned())?;
                    if config.compare_golden.is_some() {
                        return Err("--compare-golden specified multiple times".to_owned());
                    }
                    config.compare_golden = Some(scenario);
                    config.headless = true;
                }
                "--help" | "-h" => {
                    print_usage();
                    std::process::exit(0);
                }
                other => {
                    return Err(format!("Unrecognized argument: {other}"));
                }
            }
        }

        if config.snapshot.is_some() && config.capture_golden.is_some() {
            return Err("Cannot use --snapshot and --capture-golden together".to_owned());
        }
        if config.compare_golden.is_some() && config.capture_golden.is_some() {
            return Err("Cannot use --capture-golden and --compare-golden together".to_owned());
        }
        if config.compare_golden.is_some() && config.snapshot.is_some() {
            return Err("Cannot use --compare-golden and --snapshot together".to_owned());
        }

        Ok(config)
    }
}

fn print_usage() {
    println!(
        "Usage: hash-checker-gui [OPTIONS]

Options:
  --headless                      Run in non-interactive mode (no GUI window).
  --smoke-test                     Run CLI smoke test and exit.
  --manifest-dir <PATH>            Preload a directory scan before showing the GUI.
  --manifest-recursive             Include subdirectories when scanning (default).
  --manifest-non-recursive         Only scan the top-level directory.
  --manifest-algorithm <NAME>      Hash algorithm for manifest scan (default: sha256).
  --manifest-report <PATH>         Write JSON summary of the manifest view to this file.
  --snapshot <PATH>                Capture a PNG screenshot to the provided path and exit when done.
  --snapshot-width <PX>            Override snapshot width in logical pixels (default: {SNAPSHOT_DEFAULT_WIDTH}).
  --snapshot-height <PX>           Override snapshot height in logical pixels (default: {SNAPSHOT_DEFAULT_HEIGHT}).
  --capture-golden <SCENARIO>      Capture golden master JSON for the specified scenario (implies --headless).
  --compare-golden <SCENARIO>      Compare current state with stored golden master (implies --headless).
  --help, -h                       Show this help message."
    );
}

fn run_smoke_test() -> Result<(), String> {
    let path = env::temp_dir().join("hash_checker_gui_smoke.txt");
    {
        let mut file = File::create(&path).map_err(|e| e.to_string())?;
        writeln!(file, "hash-checker gui smoke test").map_err(|e| e.to_string())?;
    }
    let digest = compute_hash(&path, "sha256").map_err(|e| e.to_string())?;
    let (matches, _) = verify_hash(&path, &digest, Some("sha256")).map_err(|e| e.to_string())?;
    let _ = remove_file(&path);
    if matches {
        Ok(())
    } else {
        Err("verification mismatch".to_owned())
    }
}

fn main() -> eframe::Result<()> {
    let cli = match CliConfig::parse() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("{err}");
            eprintln!("Use --help for usage details.");
            std::process::exit(2);
        }
    };

    if cli.smoke_test {
        match run_smoke_test() {
            Ok(_) => println!("GUI smoke test passed"),
            Err(err) => {
                eprintln!("GUI smoke test failed: {err}");
                std::process::exit(1);
            }
        }
        if cli.snapshot.is_none() && cli.manifest.is_none() {
            return Ok(());
        }
    }

    if cli.headless {
        return run_headless(&cli);
    }

    let manifest_cli = cli.manifest.clone();
    let snapshot_opts = cli.snapshot.clone();

    let mut options = NativeOptions::default();
    if let Some(icon) = load_app_icon() {
        options.viewport = options.viewport.with_icon(icon);
    }
    if let Some(snapshot_opts) = &snapshot_opts {
        options.viewport = options.viewport.with_inner_size(vec2(
            snapshot_opts.width as f32,
            snapshot_opts.height as f32,
        ));
        options.viewport = options.viewport.with_resizable(true);
    }

    eframe::run_native(
        "Hash Checker",
        options,
        Box::new(move |_cc| {
            let mut app = HashCheckerApp::new();

            if let Some(manifest_cli) = manifest_cli.clone() {
                let algorithm = manifest_cli.algorithm.as_deref().unwrap_or("sha256");
                match app.preload_manifest_scan(
                    &manifest_cli.dir,
                    manifest_cli.recursive,
                    algorithm,
                ) {
                    Ok(ready) => {
                        if let Some(report_path) = &manifest_cli.report_path {
                            match write_manifest_report(report_path, &ready) {
                                Ok(()) => {
                                    println!("Manifest report written to {}", report_path.display())
                                }
                                Err(err) => eprintln!("Failed to write manifest report: {err}"),
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Failed to preload manifest: {err}");
                        std::process::exit(1);
                    }
                }
            }

            if let Some(snapshot_opts) = snapshot_opts.clone() {
                app.enable_snapshot(snapshot_opts);
            }

            Ok::<Box<dyn App>, Box<dyn std::error::Error + Send + Sync>>(
                Box::new(app) as Box<dyn App>
            )
        }),
    )
}

fn run_headless(cli: &CliConfig) -> eframe::Result<()> {
    if let Some(scenario) = &cli.compare_golden {
        return run_headless_compare_golden(scenario);
    }

    if let Some(scenario) = &cli.capture_golden {
        return run_headless_capture_golden(scenario);
    }

    if let Some(snapshot_opts) = &cli.snapshot {
        return run_headless_snapshot(cli, snapshot_opts);
    }

    if cli.manifest.is_some() {
        eprintln!(
            "Headless manifest operations require snapshot output support (Phase 1 pending). \
             Please omit --manifest-* flags or combine with --snapshot."
        );
        std::process::exit(2);
    }

    println!("Headless mode active. No interactive window launched.");
    Ok(())
}

fn run_headless_snapshot(cli: &CliConfig, snapshot_opts: &SnapshotOptions) -> eframe::Result<()> {
    let mut app = HashCheckerApp::new();

    if let Some(manifest_cli) = cli.manifest.clone() {
        let algorithm = manifest_cli.algorithm.as_deref().unwrap_or("sha256");
        if let Err(err) =
            app.preload_manifest_scan(&manifest_cli.dir, manifest_cli.recursive, algorithm)
        {
            eprintln!("Failed to preload manifest: {err}");
            std::process::exit(3);
        }
    } else {
        app.configure_file_match();
    }

    let scenario = Some(snapshot_opts.preset.label().to_string());
    let state = SnapshotState::from_app(&app, snapshot_opts, scenario);
    let (_, _, json_path) = resolve_snapshot_targets(&snapshot_opts.path);
    let document = SnapshotDocument {
        version: "1.0.0".to_owned(),
        platform: env::consts::OS.to_owned(),
        captures: vec![state],
    };

    if let Err(err) = write_snapshot_json(&json_path, &document) {
        eprintln!("Failed to write snapshot JSON: {err}");
        std::process::exit(3);
    }

    println!("Snapshot state written to {}", json_path.display());
    Ok(())
}

fn run_headless_capture_golden(scenario: &str) -> eframe::Result<()> {
    let state = match build_golden_state(scenario) {
        Ok(state) => state,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(2);
        }
    };
    let base_dir = std::env::var("HASH_CHECKER_GOLDEN_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("test-fixtures/golden"));
    let platform = env::consts::OS;
    let target_dir = base_dir.join(platform);
    if let Err(err) = fs::create_dir_all(&target_dir) {
        eprintln!(
            "Failed to prepare golden directory {}: {err}",
            target_dir.display()
        );
        std::process::exit(3);
    }
    let file_path = target_dir.join(format!("{scenario}.json"));
    let document = SnapshotDocument {
        version: "1.0.0".to_owned(),
        platform: platform.to_owned(),
        captures: vec![state],
    };
    if let Err(err) = write_snapshot_json(&file_path, &document) {
        eprintln!(
            "Failed to write golden master {}: {err}",
            file_path.display()
        );
        std::process::exit(3);
    }
    println!("Golden master saved to {}", file_path.display());
    Ok(())
}

fn run_headless_compare_golden(scenario: &str) -> eframe::Result<()> {
    let actual_state = match build_golden_state(scenario) {
        Ok(state) => state,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(2);
        }
    };
    let base_dir = std::env::var("HASH_CHECKER_GOLDEN_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("test-fixtures/golden"));
    let platform = env::consts::OS;
    let golden_path = base_dir.join(platform).join(format!("{scenario}.json"));
    if !golden_path.exists() {
        eprintln!("Golden master not found: {}", golden_path.display());
        std::process::exit(2);
    }
    let golden_file = match File::open(&golden_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!(
                "Failed to open golden master {}: {err}",
                golden_path.display()
            );
            std::process::exit(3);
        }
    };
    let golden_value: Value = match serde_json::from_reader(golden_file) {
        Ok(value) => value,
        Err(err) => {
            eprintln!(
                "Failed to parse golden master {}: {err}",
                golden_path.display()
            );
            std::process::exit(4);
        }
    };
    let actual_document = SnapshotDocument {
        version: "1.0.0".to_owned(),
        platform: platform.to_owned(),
        captures: vec![actual_state],
    };
    let actual_value = match to_value(&actual_document) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to serialize current snapshot: {err}");
            std::process::exit(3);
        }
    };
    match comparator::compare_structural(&golden_value, &actual_value) {
        comparator::ComparisonResult::Match => {
            println!("Golden master matches ({}).", golden_path.display());
            Ok(())
        }
        comparator::ComparisonResult::Diff(diffs) => {
            println!("Golden mismatch detected ({} differences).", diffs.len());
            for entry in diffs.iter().take(10) {
                println!(
                    "- {}\n    expected: {}\n    actual:   {}",
                    entry.path, entry.expected, entry.actual
                );
            }
            if diffs.len() > 10 {
                println!("  … {} more differences", diffs.len() - 10);
            }
            match serde_json::to_string_pretty(&diffs) {
                Ok(diff_json) => println!("Diff JSON:\n{}", diff_json),
                Err(err) => eprintln!("Failed to serialize diff JSON: {err}"),
            }
            std::process::exit(1);
        }
    }
}

fn write_snapshot_json<T: Serialize>(path: &Path, state: &T) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    let mut file = File::create(path)?;
    to_writer_pretty(&mut file, state)?;
    file.write_all(b"\n")?;
    Ok(())
}

fn build_golden_state(scenario: &str) -> Result<SnapshotState, String> {
    let mut app = HashCheckerApp::new();
    let mut options = SnapshotOptions::new(PathBuf::from("golden.json"));
    options.width = SNAPSHOT_DEFAULT_WIDTH;
    options.height = SNAPSHOT_DEFAULT_HEIGHT;

    match scenario {
        "minimal-scan" => {
            app.configure_file_match();
        }
        "deep-tree" => {
            app.configure_manifest_snapshot(false);
        }
        "verify-mismatches" => {
            app.configure_manifest_snapshot(true);
        }
        other => {
            return Err(format!(
                "Unknown golden scenario '{other}'. Expected minimal-scan | deep-tree | verify-mismatches"
            ));
        }
    }

    Ok(SnapshotState::from_app(
        &app,
        &options,
        Some(scenario.to_owned()),
    ))
}

fn load_app_icon() -> Option<IconData> {
    const ICON_BYTES: &[u8] = include_bytes!("../../../docs/assets/icon-hash-checker-512.png");
    let image = image::load_from_memory(ICON_BYTES).ok()?.into_rgba8();
    let width = image.width();
    let height = image.height();
    Some(IconData {
        rgba: image.into_raw(),
        width,
        height,
    })
}

fn parse_prefixed_hash(input: &str) -> Option<(String, String)> {
    let mut parts = input.splitn(2, ':');
    let prefix = parts.next()?.trim();
    let rest = parts.next()?.trim();
    if prefix.is_empty() || rest.is_empty() {
        return None;
    }
    let lowered = prefix.to_ascii_lowercase();
    if supported_prefixes().contains(&lowered.as_str()) {
        Some((lowered, rest.to_string()))
    } else {
        None
    }
}

fn extract_prefix(input: &str) -> Option<String> {
    let mut parts = input.splitn(2, ':');
    let prefix = parts.next()?.trim();
    let rest = parts.next()?.trim();
    if prefix.is_empty() || rest.is_empty() {
        return None;
    }
    let lower = prefix.to_ascii_lowercase();
    if supported_prefixes().contains(&lower.as_str()) {
        None
    } else {
        Some(prefix.to_string())
    }
}

fn supported_prefixes() -> [&'static str; 6] {
    ["sha1", "sha256", "sha512", "md5", "blake2b", "blake2s"]
}

#[cfg(test)]
mod tests {
    use super::*;
    use hash_checker::{Manifest, ManifestEntry};
    use serde_json::Value;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn parses_known_prefix() {
        let parsed = parse_prefixed_hash("sha256: abcdef").expect("should parse");
        assert_eq!(parsed.0, "sha256");
        assert_eq!(parsed.1, "abcdef");
    }

    #[test]
    fn rejects_unknown_prefix() {
        assert!(parse_prefixed_hash("foo:123").is_none());
    }

    #[test]
    fn handles_uppercase_prefix() {
        let parsed = parse_prefixed_hash("SHA1:ABC").expect("upper-case ok");
        assert_eq!(parsed.0, "sha1");
        assert_eq!(parsed.1, "ABC");
    }

    #[test]
    fn manifest_from_scan_records_entries() {
        let manifest = Manifest {
            version: "1".into(),
            algorithm: "sha256".into(),
            generated_at: 0,
            recursive: true,
            root: None,
            entries: vec![
                ManifestEntry {
                    path: "a.txt".into(),
                    hash: "aaa".into(),
                    size: 1,
                    modified: None,
                },
                ManifestEntry {
                    path: "b.txt".into(),
                    hash: "bbb".into(),
                    size: 2,
                    modified: None,
                },
            ],
        };
        let ready =
            ManifestReady::from_scan(manifest, PathBuf::from("/tmp"), Duration::from_millis(25));
        assert_eq!(ready.summary.total, 2);
        assert_eq!(ready.summary.recorded, 2);
        assert_eq!(ready.summary.matched, 0);
        assert!(ready
            .entries
            .iter()
            .all(|row| matches!(row.state, ManifestRowState::Pending)));
        assert!(ready.entries.iter().all(|row| row.actual.is_none()));
    }

    #[test]
    fn manifest_report_serializes_summary() {
        let manifest = Manifest {
            version: "1".into(),
            algorithm: "sha256".into(),
            generated_at: 0,
            recursive: true,
            root: None,
            entries: vec![ManifestEntry {
                path: "sample.txt".into(),
                hash: "abc123".into(),
                size: 42,
                modified: None,
            }],
        };
        let ready = ManifestReady::from_scan(
            manifest,
            PathBuf::from("/tmp/data"),
            Duration::from_millis(5),
        );
        let dir = tempdir().expect("temp dir");
        let report_path = dir.path().join("report.json");
        write_manifest_report(&report_path, &ready).expect("write report");
        let data = fs::read_to_string(&report_path).expect("read report");
        let json: Value = serde_json::from_str(&data).expect("parse json");
        assert_eq!(json["summary"]["total"], 1);
        assert_eq!(json["summary"]["recorded"], 1);
        assert_eq!(json["entries"][0]["state"], "pending");
        assert_eq!(json["operation"], "scan");
    }

    #[test]
    fn manifest_from_verify_tracks_recorded_counts() {
        let dir = tempdir().expect("temp dir");
        let file_a = dir.path().join("a.txt");
        let file_b = dir.path().join("b.txt");
        fs::write(&file_a, b"ok").expect("write a");
        fs::write(&file_b, b"initial").expect("write b");

        let manifest =
            generate_manifest(dir.path(), "sha256", false).expect("initial manifest generation");

        // Mutate one file and add an extra file to trigger mismatch + extra detection.
        fs::write(&file_b, b"changed").expect("mutate b");
        let extra_path = dir.path().join("extra.txt");
        fs::write(&extra_path, b"extra").expect("write extra");

        let report = verify_manifest(&manifest, dir.path()).expect("verification report");

        let ready = ManifestReady::from_verify(
            manifest.clone(),
            None,
            dir.path().to_path_buf(),
            report,
            Duration::from_millis(12),
        )
        .expect("verify state");
        assert_eq!(ready.summary.recorded, manifest.entries.len());
        assert_eq!(ready.summary.total, manifest.entries.len() + 1);
        assert_eq!(ready.summary.matched, 1);
        assert_eq!(ready.summary.mismatched, 1);
        assert_eq!(ready.summary.missing, 0);
        assert_eq!(ready.summary.extra, 1);
    }
}
