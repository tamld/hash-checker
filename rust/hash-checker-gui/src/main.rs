use std::{
    env,
    fs::{remove_file, File},
    io::Write,
    path::PathBuf,
};

use eframe::{egui, App, Frame};
use egui::{Color32, Key, RichText};
use hash_checker::{compute_hash, supported_algorithms, verify_hash};
use rfd::FileDialog;

#[derive(Default)]
struct HashCheckerApp {
    file_path: String,
    expected_hash: String,
    algorithm: AlgorithmChoice,
    computed_hash: Option<String>,
    status: Option<StatusMessage>,
    high_contrast: bool,
}

#[derive(Default)]
struct AlgorithmChoice {
    algorithms: Vec<String>,
    selected_index: usize,
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
            let algorithm = self.selected_algorithm().unwrap_or("sha256");
            match compute_hash(&path, algorithm) {
                Ok(digest) => {
                    self.computed_hash = Some(digest.clone());
                    self.set_status("Hash computed successfully.", StatusKind::Success);
                }
                Err(err) => {
                    self.computed_hash = None;
                    self.set_status(&format!("Failed to compute hash: {err}"), StatusKind::Error);
                }
            }
        } else {
            match verify_hash(&path, expected, self.selected_algorithm()) {
                Ok((matches, digest)) => {
                    self.computed_hash = Some(digest.clone());
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

    fn toggle_contrast(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.high_contrast, "High contrast theme");
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
        if self.high_contrast {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

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
            ui.add(
                egui::TextEdit::singleline(&mut self.expected_hash).desired_width(f32::INFINITY),
            );

            ui.add_space(8.0);
            if ui.button("Calculate").clicked() {
                self.calculate();
            }
            ui.small("Press Enter to calculate or toggle contrast below.");
            ui.add_space(4.0);
            self.toggle_contrast(ui);

            ui.add_space(12.0);
            if let Some(status) = &self.status {
                let color = match status.kind {
                    StatusKind::Info => Color32::LIGHT_GRAY,
                    StatusKind::Success => Color32::from_rgb(0, 170, 0),
                    StatusKind::Warning => Color32::YELLOW,
                    StatusKind::Error => Color32::RED,
                };
                ui.colored_label(color, RichText::new(&status.text).strong());
            }

            ui.add_space(12.0);
            ui.label("Computed hash:");
            ui.horizontal(|ui| {
                let mut display_hash = self.computed_hash.clone().unwrap_or_default();
                ui.add_enabled(
                    false,
                    egui::TextEdit::singleline(&mut display_hash).desired_width(f32::INFINITY),
                );
                if ui.button("Copy").clicked() {
                    let value = self.computed_hash.clone().unwrap_or_default();
                    ctx.output_mut(|o| o.copied_text = value);
                    self.set_status("Hash copied to clipboard.", StatusKind::Info);
                }
            });
        });
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

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hash Checker",
        options,
        Box::new(|_| Box::new(HashCheckerApp::new())),
    )
}
