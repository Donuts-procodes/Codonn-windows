mod app;
mod ui;
mod editor;
mod terminal;
mod file_tree;
mod file_ops;

use eframe::egui;
use app::CodeEditorApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1600.0, 1000.0)),
        ..Default::default()
    };
    
    match eframe::run_native(
        "🧬Codonn",
        options,
        Box::new(|_cc| Box::new(CodeEditorApp::default())),
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Application error: {:?}", e);
            Err(e)
        }
    }
}
