use eframe::egui;
use std::fs;
use std::path::PathBuf;

struct VSCodeEditor {
    text: String,
    file_path: Option<PathBuf>,
    open_files: Vec<PathBuf>,
    current_file_index: usize,
    show_sidebar: bool,
}

impl Default for VSCodeEditor {
    fn default() -> Self {
        Self {
            text: String::new(),
            file_path: None,
            open_files: Vec::new(),
            current_file_index: 0,
            show_sidebar: true,
        }
    }
}

impl eframe::App for VSCodeEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        self.open_file();
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {
                        self.save_file();
                        ui.close_menu();
                    }
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                });
                ui.menu_button("Edit", |ui| {
                    ui.label("Cut");
                    ui.label("Copy");
                    ui.label("Paste");
                });
                ui.menu_button("View", |ui| {
                    if ui.button("Toggle Sidebar").clicked() {
                        self.show_sidebar = !self.show_sidebar;
                        ui.close_menu();
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Ln 1, Col 1");
                ui.separator();
                ui.label("UTF-8");
                ui.separator();
                ui.label("Rust");
            });
        });

        if self.show_sidebar {
            egui::SidePanel::left("sidebar")
                .min_width(200.0)
                .show(ctx, |ui| {
                    ui.heading("Explorer");
                    ui.separator();
                    
                    // Clone to avoid borrow issues
                    let open_files = self.open_files.clone();
                    let current_index = self.current_file_index;
                    let mut selected_index = None;
                    
                    for (i, path) in open_files.iter().enumerate() {
                        let file_name = path.file_name().unwrap().to_string_lossy();
                        if ui.selectable_label(i == current_index, file_name.to_string()).clicked() {
                            selected_index = Some(i);
                        }
                    }
                    
                    if let Some(i) = selected_index {
                        self.current_file_index = i;
                        self.load_file(open_files[i].clone());
                    }
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.open_files.is_empty() {
                ui.horizontal(|ui| {
                    let open_files = self.open_files.clone();
                    let current_index = self.current_file_index;
                    let mut selected_index = None;
                    
                    for (i, path) in open_files.iter().enumerate() {
                        let tab_name = path.file_name().unwrap().to_string_lossy();
                        if ui.selectable_label(i == current_index, tab_name.to_string()).clicked() {
                            selected_index = Some(i);
                        }
                    }
                    
                    if let Some(i) = selected_index {
                        self.current_file_index = i;
                        self.load_file(open_files[i].clone());
                    }
                });
                ui.separator();
            }

            let lines: Vec<&str> = self.text.lines().collect();
            let max_line_number = lines.len().max(1).to_string().len();

            let line_numbers = lines
                .iter()
                .enumerate()
                .map(|(i, _)| format!("{: >width$}", i + 1, width = max_line_number))
                .collect::<Vec<_>>()
                .join("\n");

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::Label::new(line_numbers).wrap(false));
                    ui.add_space(10.0);

                    ui.add(
                        egui::TextEdit::multiline(&mut self.text)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY),
                    );
                });
            });
        });
    }
}

impl VSCodeEditor {
    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                self.text = content;
                self.file_path = Some(path.clone());
                if !self.open_files.contains(&path) {
                    self.open_files.push(path);
                    self.current_file_index = self.open_files.len() - 1;
                }
            }
        }
    }

    fn save_file(&mut self) {
        if let Some(path) = &self.file_path {
            fs::write(path, &self.text).ok();
        }
    }

    fn load_file(&mut self, path: PathBuf) {
        if let Ok(content) = fs::read_to_string(&path) {
            self.text = content;
            self.file_path = Some(path);
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Codonn",
        options,
        Box::new(|_cc| Box::new(VSCodeEditor::default())),
    )
}
