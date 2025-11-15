use eframe::egui;
use std::path::Path;
use walkdir::WalkDir;
use crate::app::CodeEditorApp;
use crate::file_ops;

pub fn render_tree(app: &mut CodeEditorApp, ui: &mut egui::Ui, path: &Path) {
    for entry in WalkDir::new(path)
        .min_depth(1)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path_buf = entry.path().to_path_buf();
        let depth = entry.depth();
        let indent = "  ".repeat(depth);
        
        if entry.file_type().is_dir() {
            let is_expanded = app.file_tree_expanded.get(&path_buf).copied().unwrap_or(false);
            let label = if is_expanded { "📁 " } else { "📂 " };
            
            if ui.button(format!("{}{}",indent, label)).clicked() {
                app.file_tree_expanded.insert(path_buf.clone(), !is_expanded);
            }
        } else {
            let file_name = entry.file_name().to_string_lossy();
            let icon = get_file_icon(&file_name);
            
            if ui.button(format!("{}{} {}", indent, icon, file_name)).clicked() {
                let path_buf_clone = path_buf.clone();
                file_ops::load_file(app, path_buf_clone.clone());
                if !app.open_files.contains(&path_buf_clone) {
                    app.open_files.push(path_buf_clone);
                    app.current_file_index = app.open_files.len() - 1;
                }
            }
        }
    }
}

fn get_file_icon(filename: &str) -> &'static str {
    match std::path::Path::new(filename).extension().and_then(|s| s.to_str()) {
        Some("rs") => "🦀",
        Some("py") => "🐍",
        Some("js") => "📜",
        Some("html") => "🌐",
        Some("css") => "🎨",
        Some("json") => "📦",
        Some("md") => "📝",
        Some("txt") => "📄",
        _ => "📋",
    }
}
