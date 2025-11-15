use eframe::egui;
use crate::app::CodeEditorApp;
use crate::file;
use crate::terminal;

pub fn render(app: &mut CodeEditorApp, ctx: &egui::Context) {
    set_theme(ctx);

    render_menu_bar(app, ctx);
    render_status_bar(app, ctx);
    render_sidebar(app, ctx);
    render_terminal(app, ctx);
    render_editor(app, ctx);
}

fn set_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.override_text_color = Some(egui::Color32::from_rgb(229, 229, 229));
    ctx.set_visuals(visuals);
}

fn render_menu_bar(app: &mut CodeEditorApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("menu_bar")
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(30, 30, 46)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("📝 Code Editor");
                ui.separator();
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("➕ Open File").clicked() {
                            file::open_file(app);
                            ui.close_menu();
                        }
                        if ui.button("💾 Save").clicked() {
                            file::save_file(app);
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("❌ Exit").clicked() {
                            std::process::exit(0);
                        }
                    });
                    ui.menu_button("View", |ui| {
                        if ui.button(if app.show_sidebar { "📁 Hide Sidebar" } else { "📁 Show Sidebar" }).clicked() {
                            app.show_sidebar = !app.show_sidebar;
                            ui.close_menu();
                        }
                        if ui.button(if app.show_terminal { "🖥️ Hide Terminal" } else { "🖥️ Show Terminal" }).clicked() {
                            app.show_terminal = !app.show_terminal;
                            ui.close_menu();
                        }
                    });
                });
            });
        });
}

fn render_status_bar(app: &CodeEditorApp, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("status_bar")
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(30, 30, 46)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("✓ Ready");
                ui.separator();
                ui.label(format!("Lines: {}", app.text.lines().count()));
                ui.separator();
                ui.label("UTF-8");
                ui.separator();
                if let Some(path) = &app.file_path {
                    ui.label(format!("📄 {}", path.file_name().unwrap().to_string_lossy()));
                }
            });
        });
}

fn render_sidebar(app: &mut CodeEditorApp, ctx: &egui::Context) {
    if app.show_sidebar {
        egui::SidePanel::left("sidebar")
            .min_width(250.0)
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(24, 24, 37)))
            .show(ctx, |ui| {
                ui.heading("📂 Explorer");
                ui.separator();
                
                let open_files = app.open_files.clone();
                let current_index = app.current_file_index;
                let mut selected_index = None;
                
                for (i, path) in open_files.iter().enumerate() {
                    let file_name = path.file_name().unwrap().to_string_lossy();
                    let is_active = i == current_index;
                    
                    if ui.selectable_label(is_active, format!("  {}", file_name)).clicked() {
                        selected_index = Some(i);
                    }
                }
                
                if let Some(i) = selected_index {
                    app.current_file_index = i;
                    file::load_file(app, open_files[i].clone());
                }
            });
    }
}

fn render_terminal(app: &mut CodeEditorApp, ctx: &egui::Context) {
    if app.show_terminal {
        egui::TopBottomPanel::bottom("terminal")
            .min_height(200.0)
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(24, 24, 37)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("🖥️ Terminal");
                    if ui.button("⚡ PowerShell").clicked() {
                        terminal::run_command(app, "powershell", "-NoExit", "-Command", "clear");
                    }
                    if ui.button("⚡ CMD").clicked() {
                        terminal::run_command(app, "cmd", "/K", "cls", "");
                    }
                    if ui.button("🗑️ Clear").clicked() {
                        app.terminal_output.lock().unwrap().clear();
                    }
                });
                
                ui.separator();
                
                let output = app.terminal_output.lock().unwrap();
                egui::ScrollArea::vertical()
                    .max_height(150.0)
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new(output.as_str())
                            .monospace()
                            .color(egui::Color32::from_rgb(0, 255, 0)));
                    });
            });
    }
}

fn render_editor(app: &mut CodeEditorApp, ctx: &egui::Context) {
    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(30, 30, 46)))
        .show(ctx, |ui| {
            // Tab bar
            if !app.open_files.is_empty() {
                ui.horizontal(|ui| {
                    let open_files = app.open_files.clone();
                    let current_index = app.current_file_index;
                    let mut selected_index = None;
                    
                    for (i, path) in open_files.iter().enumerate() {
                        let tab_name = path.file_name().unwrap().to_string_lossy();
                        let is_active = i == current_index;
                        
                        if ui.selectable_label(is_active, format!("  {} ", tab_name)).clicked() {
                            selected_index = Some(i);
                        }
                    }
                    
                    if let Some(i) = selected_index {
                        app.current_file_index = i;
                        file::load_file(app, open_files[i].clone());
                    }
                });
                ui.separator();
            }

            // Code editor with line numbers
            let lines: Vec<&str> = app.text.lines().collect();
            let max_line_number = lines.len().max(1).to_string().len();

            let line_numbers = lines
                .iter()
                .enumerate()
                .map(|(i, _)| format!("{: >width$}", i + 1, width = max_line_number))
                .collect::<Vec<_>>()
                .join("\n");

            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(line_numbers)
                                    .monospace()
                                    .color(egui::Color32::from_rgb(89, 98, 120))
                                    .size(13.0)
                            )
                            .wrap(false)
                        );
                        ui.add_space(10.0);

                        let text_edit = egui::TextEdit::multiline(&mut app.text)
                            .font(egui::TextStyle::Monospace)
                            .text_color(egui::Color32::from_rgb(229, 229, 229))
                            .desired_width(f32::INFINITY)
                            .desired_rows(20);
                        
                        ui.add(text_edit);
                    });
                });
        });
}
