use eframe::egui;
use std::fs;

struct CodeEditor {
    text: String,
    file_path: String,
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self {
            text: String::new(),
            file_path: String::new(),
        }
    }
}

impl eframe::App for CodeEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Code Editor");
            ui.text_edit_multiline(&mut self.text);
            ui.horizontal(|ui| {
                if ui.button("Open").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        if let Ok(content) = fs::read_to_string(&path) {
                            self.text = content;
                            self.file_path = path.to_string_lossy().to_string();
                        }
                    }
                }
                if ui.button("Save").clicked() {
                    if !self.file_path.is_empty() {
                        fs::write(&self.file_path, &self.text).ok();
                    }
                }
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Code Editor",
        options,
        Box::new(|_cc| Box::new(CodeEditor::default())),
    )
}
