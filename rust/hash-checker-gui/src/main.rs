#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use std::{
    env,
    fs::{remove_file, File},
    io::Write,
    path::PathBuf,
};

use eframe::{egui, App, Frame, NativeOptions};
use egui::{vec2, Color32, IconData, Key, RichText};
use hash_checker::{compute_hash, supported_algorithms, verify_hash};
use rfd::FileDialog;

#[derive(Default)]
struct HashCheckerApp {
    file_path: String,
    expected_hash: String,
    algorithm: AlgorithmChoice,
    computed_hash: Option<String>,
    status: Option<StatusMessage>,
    theme: ThemeChoice,
    last_algorithm_used: Option<String>,
}

#[derive(Default)]
struct AlgorithmChoice {
    algorithms: Vec<String>,
    selected_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            ThemePreset::HighContrast => "High Contrast Dark",
        }
    }

    fn visuals(&self) -> egui::Visuals {
        match self {
            ThemePreset::SoftLight => {
                let mut visuals = egui::Visuals::light();
                visuals.override_text_color = Some(Color32::from_rgb(40, 44, 52));
                visuals.panel_fill = Color32::from_rgb(246, 247, 250);
                visuals.widgets.inactive.bg_fill = Color32::from_rgb(236, 239, 244);
                visuals.widgets.inactive.fg_stroke.color = Color32::from_rgb(54, 58, 68);
                visuals.widgets.hovered.bg_fill = Color32::from_rgb(223, 227, 236);
                visuals.widgets.active.bg_fill = Color32::from_rgb(209, 216, 229);
                visuals.selection.bg_fill = Color32::from_rgb(86, 110, 157);
                visuals.button_frame = true;
                visuals
            }
            ThemePreset::Slate => {
                let mut visuals = egui::Visuals::dark();
                visuals.override_text_color = Some(Color32::from_rgb(225, 228, 235));
                visuals.panel_fill = Color32::from_rgb(38, 44, 53);
                visuals.widgets.inactive.bg_fill = Color32::from_rgb(48, 56, 66);
                visuals.widgets.hovered.bg_fill = Color32::from_rgb(62, 72, 83);
                visuals.widgets.active.bg_fill = Color32::from_rgb(72, 84, 96);
                visuals.selection.bg_fill = Color32::from_rgb(86, 132, 172);
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
            presets: vec![
                ThemePreset::SoftLight,
                ThemePreset::Slate,
                ThemePreset::HighContrast,
            ],
            selected_index: 0,
        }
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

impl HashCheckerApp {
    fn new() -> Self {
        let mut algorithms: Vec<String> = supported_algorithms()
            .iter()
            .map(|s| s.to_string())
            .collect();
        algorithms.insert(0, "auto".to_owned());
        Self {
            algorithm: AlgorithmChoice {
                algorithms,
                selected_index: 0,
            },
            theme: ThemeChoice::default(),
            last_algorithm_used: None,
            ..Default::default()
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

    fn pick_file(&mut self) {
        if let Some(path) = FileDialog::new().pick_file() {
            if let Some(path_str) = path.to_str() {
                self.file_path = path_str.to_owned();
            }
        }
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
        ui.horizontal(|ui| {
            ui.label("Theme:");
            egui::ComboBox::from_label("")
                .selected_text(self.selected_theme().name())
                .show_ui(ui, |combo| {
                    for (idx, preset) in self.theme.presets.iter().enumerate() {
                        combo.selectable_value(&mut self.theme.selected_index, idx, preset.name());
                    }
                });
        });
    }

    fn process_input(&mut self, ctx: &egui::Context) {
        let dropped = ctx.input(|i| i.raw.dropped_files.clone());
        if let Some(path) = dropped.iter().find_map(|file| file.path.as_ref()) {
            self.file_path = path.to_string_lossy().into();
            self.set_status("File selected from drag-and-drop.", StatusKind::Info);
        }

        if ctx.input(|i| i.key_pressed(Key::Enter)) {
            self.calculate();
        }
    }
}

impl App for HashCheckerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        self.process_input(ctx);
        ctx.set_visuals(self.selected_theme().visuals());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hash Checker (Rust GUI)");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("File:");
                ui.add(egui::TextEdit::singleline(&mut self.file_path).desired_width(320.0));
                if ui.button("Browse…").clicked() {
                    self.pick_file();
                }
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("Algorithm:");
                egui::ComboBox::from_label("")
                    .selected_text(self.algorithm_label(self.algorithm.selected_index))
                    .show_ui(ui, |combo| {
                        for (idx, label) in self.algorithm.algorithms.iter().enumerate() {
                            combo.selectable_value(&mut self.algorithm.selected_index, idx, label);
                        }
                    });
            });

            ui.add_space(8.0);
            ui.label("Expected hash (optional):");
            let response = ui.add(
                egui::TextEdit::singleline(&mut self.expected_hash).desired_width(f32::INFINITY),
            );
            if response.changed() {
                self.handle_expected_hash_change();
            }

            ui.add_space(8.0);
            if ui.button("Calculate").clicked() {
                self.calculate();
            }
            ui.small("Press Enter to calculate. Choose a theme below to adjust the palette.");
            ui.add_space(4.0);
            self.theme_selector(ui);

            ui.add_space(12.0);
            if let Some(status) = &self.status {
                let color = match status.kind {
                    StatusKind::Info => Color32::from_rgb(120, 125, 136),
                    StatusKind::Success => Color32::from_rgb(0, 170, 0),
                    StatusKind::Warning => Color32::YELLOW,
                    StatusKind::Error => Color32::RED,
                };
                ui.colored_label(color, RichText::new(&status.text).strong());
            }

            ui.add_space(12.0);
            ui.label("Computed hash:");
            ui.horizontal(|ui| {
                let available = ui.available_width();
                let button_width = 140.0;
                let text_width =
                    (available - ui.spacing().item_spacing.x - button_width).max(160.0);
                let mut display_hash = self.computed_hash.clone().unwrap_or_default();
                ui.add_enabled(
                    false,
                    egui::TextEdit::singleline(&mut display_hash).desired_width(text_width),
                );
                let copy_button = egui::Button::new("Copy hash")
                    .min_size(vec2(button_width, ui.spacing().interact_size.y * 1.1));
                let response = ui.add_enabled(self.computed_hash.is_some(), copy_button);
                if response.clicked() {
                    let value = self.computed_hash.clone().unwrap_or_default();
                    let algo = self
                        .last_algorithm_used
                        .clone()
                        .unwrap_or_else(|| self.resolve_algorithm().to_ascii_uppercase());
                    let formatted = format!("{}:{}", algo, value);
                    ctx.output_mut(|o| o.copied_text = formatted);
                    self.set_status("Hash copied to clipboard.", StatusKind::Info);
                }
            });
        });
    }
}

impl HashCheckerApp {
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
            self.expected_hash = trimmed;
        }
    }
}

fn parse_prefixed_hash(input: &str) -> Option<(String, String)> {
    let mut parts = input.splitn(2, ':');
    let prefix = parts.next()?.trim();
    let rest = parts.next()?.trim();
    if prefix.is_empty() || rest.is_empty() {
        return None;
    }
    let lowered = prefix.to_ascii_lowercase();
    let known = ["sha1", "sha256", "sha512", "md5", "blake2b", "blake2s"];
    if known.contains(&lowered.as_str()) {
        Some((lowered, rest.to_string()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::parse_prefixed_hash;

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
    if env::args().any(|arg| arg == "--smoke-test") {
        match run_smoke_test() {
            Ok(_) => {
                println!("GUI smoke test passed");
                return Ok(());
            }
            Err(err) => {
                eprintln!("GUI smoke test failed: {err}");
                std::process::exit(1);
            }
        }
    }

    let mut options = NativeOptions::default();
    if let Some(icon) = load_app_icon() {
        options.viewport = options.viewport.with_icon(icon);
    }
    eframe::run_native(
        "Hash Checker",
        options,
        Box::new(|_| Box::new(HashCheckerApp::new())),
    )
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
