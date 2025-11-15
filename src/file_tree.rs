use eframe::egui;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::app::CodeEditorApp;
use crate::file_ops;

pub fn render_tree(app: &mut CodeEditorApp, ui: &mut egui::Ui, path: &Path) {
    let mut entries: Vec<_> = WalkDir::new(path)
        .min_depth(1)
        .max_depth(4)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect();
    
    entries.sort_by(|a, b| {
        let a_is_dir = a.file_type().is_dir();
        let b_is_dir = b.file_type().is_dir();
        
        if a_is_dir != b_is_dir {
            b_is_dir.cmp(&a_is_dir)
        } else {
            a.file_name().cmp(b.file_name())
        }
    });
    
    for entry in entries {
        let path_buf = entry.path().to_path_buf();
        let depth = entry.depth();
        
        if entry.file_type().is_dir() {
            render_folder(app, ui, &path_buf, depth);
        } else {
            render_file(app, ui, &path_buf, depth);
        }
    }
}

fn render_folder(app: &mut CodeEditorApp, ui: &mut egui::Ui, path: &PathBuf, depth: usize) {
    let is_expanded = app.file_tree_expanded.get(path).copied().unwrap_or(false);
    let indent = "    ".repeat(depth - 1);
    let arrow = if is_expanded { "▼" } else { "▶" };
    let folder_icon = "📁";
    let folder_name = path.file_name().unwrap().to_string_lossy();
    
    let button_text = format!("{}{} {} {}", indent, arrow, folder_icon, folder_name);
    
    if ui.button(button_text).clicked() {
        app.file_tree_expanded.insert(path.clone(), !is_expanded);
    }
}

fn render_file(app: &mut CodeEditorApp, ui: &mut egui::Ui, path: &PathBuf, depth: usize) {
    let indent = "    ".repeat(depth - 1);
    let icon = get_file_icon(&path.file_name().unwrap().to_string_lossy());
    let file_name = path.file_name().unwrap().to_string_lossy();
    
    let button_text = format!("{}  {} {}", indent, icon, file_name);
    
    if ui.button(button_text).clicked() {
        file_ops::load_file(app, path.clone());
        if !app.open_files.contains(path) {
            app.open_files.push(path.clone());
            app.current_file_index = app.open_files.len() - 1;
        }
    }
}

fn get_file_icon(filename: &str) -> &'static str {
    match Path::new(filename).extension().and_then(|s| s.to_str()) {
        Some("rs") => "🦀",
        Some("py") => "🐍",
        Some("js") | Some("jsx") | Some("ts") | Some("tsx") => "📜",
        Some("html") => "🌐",
        Some("css") | Some("scss") => "🎨",
        Some("json") => "📦",
        Some("md") => "📝",
        Some("txt") => "📄",
        Some("toml") | Some("yaml") | Some("yml") => "⚙️",
        Some("lock") => "🔒",
        _ => "📋",
    }
}
