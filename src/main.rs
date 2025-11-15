mod app;
mod ui;
mod editor;
mod terminal;
mod file;

use eframe::egui;
use app::CodeEditorApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1400.0, 900.0)),
        ..Default::default()
    };
    eframe::run_native(
        "🔥 Code Editor - Professional Edition",
        options,
        Box::new(|_cc| Box::new(CodeEditorApp::default())),
    )
}
