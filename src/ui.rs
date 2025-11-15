use eframe::egui;
use crate::app::CodeEditorApp;
use crate::file_ops;
use crate::file_tree;
use crate::terminal;
use crate::editor;

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
                ui.heading("🔥 Code Editor");
                ui.separator();
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("📂 Open Folder").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                app.root_folder = Some(path.clone());
                                app.breadcrumb_path = vec![path];
                                ui.close_menu();
                            }
                        }
                        if ui.button("📄 Open File").clicked() {
                            file_ops::open_file(app);
                            ui.close_menu();
                        }
                        if ui.button("💾 Save").clicked() {
                            file_ops::save_file(app);
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
                ui.label("UTF-8 | Rust");
                ui.separator();
                if let Some(path) = &app.file_path {
                    ui.label(format!("📄 {}", path.file_name().unwrap().to_string_lossy()));
                }
            });
        });
}

fn render_breadcrumb(app: &mut CodeEditorApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("breadcrumb")
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(24, 24, 37)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(root) = &app.root_folder {
                    ui.label("📍 ");
                    let root_clone = root.clone();
                    if ui.button(root.file_name().unwrap().to_string_lossy().to_string()).clicked() {
                        app.breadcrumb_path = vec![root_clone];
                    }
                    let breadcrumb_copy = app.breadcrumb_path.clone();
                    for (idx, path) in breadcrumb_copy.iter().skip(1).enumerate() {
                        ui.label(">");
                        if ui.button(path.file_name().unwrap().to_string_lossy().to_string()).clicked() {
                            app.breadcrumb_path.truncate(idx + 2);
                        }
                    }
                }
            });
        });
}

fn render_sidebar(app: &mut CodeEditorApp, ctx: &egui::Context) {
    if app.show_sidebar {
        egui::SidePanel::left("sidebar")
            .min_width(280.0)
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(24, 24, 37)))
            .show(ctx, |ui| {
                ui.heading("📂 Explorer");
                ui.separator();
                
                if let Some(root) = &app.root_folder.clone() {
                    file_tree::render_tree(app, ui, root);
                } else {
                    ui.label("Open a folder to see file tree");
                }
                
                ui.separator();
                ui.heading("📋 Open Files");
                
                let open_files = app.open_files.clone();
                let current_index = app.current_file_index;
                let mut selected_index = None;
                let mut close_index = None;
                
                for (i, path) in open_files.iter().enumerate() {
                    ui.horizontal(|ui| {
                        let file_name = path.file_name().unwrap().to_string_lossy();
                        if ui.selectable_label(i == current_index, format!("  {}", file_name)).clicked() {
                            selected_index = Some(i);
                        }
                        if ui.button("✕").clicked() {
                            close_index = Some(i);
                        }
                    });
                }
                
                if let Some(i) = selected_index {
                    app.current_file_index = i;
                    file_ops::load_file(app, open_files[i].clone());
                }
                
                if let Some(i) = close_index {
                    app.open_files.remove(i);
                    if app.current_file_index >= app.open_files.len() {
                        app.current_file_index = app.open_files.len().saturating_sub(1);
                    }
                }
            });
    }
}

fn render_terminal(app: &mut CodeEditorApp, ctx: &egui::Context) {
    if app.show_terminal {
        egui::TopBottomPanel::bottom("terminal")
            .min_height(220.0)
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(24, 24, 37)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("🖥️ Terminal");
                    if ui.button("⚡ PowerShell").clicked() {
                        terminal::open_powershell(app);
                    }
                    if ui.button("⚡ CMD").clicked() {
                        terminal::open_cmd(app);
                    }
                    if ui.button("🗑️ Clear").clicked() {
                        app.terminal_output.lock().unwrap().clear();
                        app.terminal_output.lock().unwrap().push_str("Terminal Cleared\n> ");
                    }
                });
                
                ui.separator();
                
                let output = app.terminal_output.lock().unwrap();
                egui::ScrollArea::vertical()
                    .max_height(150.0)
                    .auto_shrink([false; 2])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new(output.as_str())
                            .monospace()
                            .color(egui::Color32::from_rgb(0, 255, 0))
                            .size(12.0));
                    });
            });
    }
}

fn render_editor(app: &mut CodeEditorApp, ctx: &egui::Context) {
    render_breadcrumb(app, ctx);
    
    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(30, 30, 46)))
        .show(ctx, |ui| {
            // Tab bar with close buttons
            if !app.open_files.is_empty() {
                ui.horizontal(|ui| {
                    let open_files = app.open_files.clone();
                    let current_index = app.current_file_index;
                    let mut selected_index = None;
                    let mut close_index = None;
                    
                    for (i, path) in open_files.iter().enumerate() {
                        let tab_name = path.file_name().unwrap().to_string_lossy();
                        let is_active = i == current_index;
                        
                        ui.horizontal(|ui| {
                            if ui.selectable_label(is_active, format!("  {} ", tab_name)).clicked() {
                                selected_index = Some(i);
                            }
                            if ui.button("✕").clicked() {
                                close_index = Some(i);
                            }
                        });
                    }
                    
                    if let Some(i) = selected_index {
                        app.current_file_index = i;
                        file_ops::load_file(app, open_files[i].clone());
                    }
                    
                    if let Some(i) = close_index {
                        app.open_files.remove(i);
                        if app.current_file_index >= app.open_files.len() {
                            app.current_file_index = app.open_files.len().saturating_sub(1);
                        }
                    }
                });
                ui.separator();
            }

            // Run buttons based on file type
            if let Some(path) = &app.file_path.clone() {
                ui.horizontal(|ui| {
                    if ui.button("▶️ Run").clicked() {
                        terminal::run_file(app, path);
                    }
                    if ui.button("🔨 Build").clicked() {
                        terminal::build_file(app, path);
                    }
                });
                ui.separator();
            }

            // Code editor with line numbers and syntax highlighting
            editor::render_editor(app, ui);
        });
}
