use std::fs;
use std::path::PathBuf;
use rfd::FileDialog;
use crate::app::CodeEditorApp;

pub fn open_file(app: &mut CodeEditorApp) {
    if let Some(path) = FileDialog::new().pick_file() {
        if let Ok(content) = fs::read_to_string(&path) {
            app.text = content;
            app.file_path = Some(path.clone());
            if !app.open_files.contains(&path) {
                app.open_files.push(path);
                app.current_file_index = app.open_files.len() - 1;
            }
        }
    }
}

pub fn save_file(app: &mut CodeEditorApp) {
    if let Some(path) = &app.file_path {
        if fs::write(path, &app.text).is_ok() {
            let mut output = app.terminal_output.lock().unwrap();
            output.push_str(&format!("✓ Saved: {}\n", path.display()));
        }
    }
}

pub fn load_file(app: &mut CodeEditorApp, path: PathBuf) {
    if let Ok(content) = fs::read_to_string(&path) {
        app.text = content;
        app.file_path = Some(path);
    }
}
