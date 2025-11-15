use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use eframe::egui;

pub struct CodeEditorApp {
    pub text: String,
    pub file_path: Option<PathBuf>,
    pub open_files: Vec<PathBuf>,
    pub current_file_index: usize,
    pub show_sidebar: bool,
    pub show_terminal: bool,
    pub terminal_output: Arc<Mutex<String>>,
    pub terminal_input: String,
    pub root_folder: Option<PathBuf>,
    pub file_tree_expanded: std::collections::HashMap<PathBuf, bool>,
    pub breadcrumb_path: Vec<PathBuf>,
}

impl Default for CodeEditorApp {
    fn default() -> Self {
        Self {
            text: String::new(),
            file_path: None,
            open_files: Vec::new(),
            current_file_index: 0,
            show_sidebar: true,
            show_terminal: true,
            terminal_output: Arc::new(Mutex::new("🖥️ Terminal Ready\nType commands below\n> ".to_string())),
            terminal_input: String::new(),
            root_folder: None,
            file_tree_expanded: std::collections::HashMap::new(),
            breadcrumb_path: Vec::new(),
        }
    }
}

impl eframe::App for CodeEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::ui::render(self, ctx);
    }
}
