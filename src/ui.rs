use eframe::egui;
use crate::app::CodeEditorApp;
use crate::file_ops;
use crate::file_tree;
use crate::terminal;
use crate::editor;

// Custom color scheme (Dracula-inspired)
const COLOR_BG: egui::Color32 = egui::Color32::from_rgb(40, 42, 54);
const COLOR_DARK_BG: egui::Color32 = egui::Color32::from_rgb(28, 30, 43);
const COLOR_ACCENT: egui::Color32 = egui::Color32::from_rgb(139, 233, 253);
const COLOR_SUCCESS: egui::Color32 = egui::Color32::from_rgb(80, 250, 123);
const COLOR_ERROR: egui::Color32 = egui::Color32::from_rgb(255, 121, 198);
const COLOR_TEXT: egui::Color32 = egui::Color32::from_rgb(248, 248, 242);
const COLOR_MUTED: egui::Color32 = egui::Color32::from_rgb(98, 114, 164);

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
    visuals.override_text_color = Some(COLOR_TEXT);
    visuals.panel_fill = COLOR_DARK_BG;
    visuals.window_fill = COLOR_BG;
    ctx.set_visuals(visuals);
}

fn render_menu_bar(app: &mut CodeEditorApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("menu_bar")
        .frame(egui::Frame::none().fill(COLOR_DARK_BG).stroke(egui::Stroke::new(1.0, COLOR_MUTED)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("🧬 Codonn").color(COLOR_ACCENT).size(18.0));
                ui.separator();
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("📁 File", |ui| {
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
                    ui.menu_button("👁️ View", |ui| {
                        if ui.button(if app.show_sidebar { "👁️ Hide Sidebar" } else { "👁️ Show Sidebar" }).clicked() {
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
        .frame(egui::Frame::none().fill(COLOR_DARK_BG).stroke(egui::Stroke::new(1.0, COLOR_MUTED)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("✓").color(COLOR_SUCCESS));
                ui.label(egui::RichText::new("Ready").color(COLOR_TEXT));
                ui.separator();
                ui.label(egui::RichText::new(format!("Lines: {}", app.text.lines().count())).color(COLOR_ACCENT));
                ui.separator();
                ui.label(egui::RichText::new("UTF-8").color(COLOR_MUTED));
                ui.separator();
                if let Some(path) = &app.file_path {
                    ui.label(egui::RichText::new(format!("📄 {}", path.file_name().unwrap().to_string_lossy())).color(COLOR_SUCCESS));
                }
            });
        });
}

fn render_breadcrumb(app: &mut CodeEditorApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("breadcrumb")
        .frame(egui::Frame::none().fill(COLOR_BG).stroke(egui::Stroke::new(1.0, COLOR_MUTED)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(root) = &app.root_folder {
                    ui.label(egui::RichText::new("📍").color(COLOR_ACCENT));
                    let root_clone = root.clone();
                    if ui.button(egui::RichText::new(root.file_name().unwrap().to_string_lossy().to_string()).color(COLOR_ACCENT)).clicked() {
                        app.breadcrumb_path = vec![root_clone];
                    }
                    let breadcrumb_copy = app.breadcrumb_path.clone();
                    for (idx, path) in breadcrumb_copy.iter().skip(1).enumerate() {
                        ui.label(egui::RichText::new(">").color(COLOR_MUTED));
                        if ui.button(egui::RichText::new(path.file_name().unwrap().to_string_lossy().to_string()).color(COLOR_ACCENT)).clicked() {
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
            .min_width(300.0)
            .frame(egui::Frame::none().fill(COLOR_DARK_BG).stroke(egui::Stroke::new(1.0, COLOR_MUTED)))
            .show(ctx, |ui| {
                ui.heading(egui::RichText::new("📂 File Explorer").color(COLOR_ACCENT).size(16.0));
                ui.separator();
                
                if let Some(root) = &app.root_folder.clone() {
                    egui::ScrollArea::vertical()
                        .max_height(400.0)
                        .show(ui, |ui| {
                            file_tree::render_tree(app, ui, root);
                        });
                } else {
                    ui.colored_label(COLOR_MUTED, "📂 Open a folder to see files");
                }
                
                ui.separator();
                ui.heading(egui::RichText::new("📋 Open Files").color(COLOR_ACCENT).size(16.0));
                
                let open_files = app.open_files.clone();
                let current_index = app.current_file_index;
                let mut selected_index = None;
                let mut close_index = None;
                
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for (i, path) in open_files.iter().enumerate() {
                            ui.horizontal(|ui| {
                                let file_name = path.file_name().unwrap().to_string_lossy();
                                let is_active = i == current_index;
                                
                                let label_color = if is_active { COLOR_SUCCESS } else { COLOR_TEXT };
                                
                                if ui.selectable_label(
                                    is_active,
                                    egui::RichText::new(format!("  {}", file_name)).color(label_color)
                                ).clicked() {
                                    selected_index = Some(i);
                                }
                                
                                if ui.button(egui::RichText::new("✕").color(COLOR_ERROR)).clicked() {
                                    close_index = Some(i);
                                }
                            });
                        }
                    });
                
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
            .min_height(280.0)
            .frame(egui::Frame::none().fill(COLOR_DARK_BG).stroke(egui::Stroke::new(1.0, COLOR_MUTED)))
            .show(ctx, |ui| {
                // Header with buttons
                ui.horizontal(|ui| {
                    ui.heading(egui::RichText::new("🖥️ Terminal").color(COLOR_ACCENT).size(16.0));
                    if ui.button(egui::RichText::new("⚡ PowerShell").color(COLOR_SUCCESS)).clicked() {
                        terminal::open_powershell(app);
                    }
                    if ui.button(egui::RichText::new("⚡ CMD").color(COLOR_SUCCESS)).clicked() {
                        terminal::open_cmd(app);
                    }
                    if ui.button(egui::RichText::new("🗑️ Clear").color(COLOR_ERROR)).clicked() {
                        app.terminal_output.lock().unwrap().clear();
                        app.terminal_output.lock().unwrap().push_str("Terminal Cleared\n> ");
                    }
                });
                
                ui.separator();
                
                // Terminal output display
                let output = app.terminal_output.lock().unwrap().clone();
                egui::ScrollArea::vertical()
                    .max_height(130.0)
                    .auto_shrink([false; 2])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new(&output)
                            .monospace()
                            .color(COLOR_SUCCESS)
                            .size(11.0));
                    });
                
                ui.separator();
                
                // Terminal input section
                ui.label(egui::RichText::new("Command Input:").color(COLOR_ACCENT));
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("$ ").color(COLOR_ACCENT).size(14.0));
                    ui.text_edit_singleline(&mut app.terminal_input);
                    
                    if ui.button(egui::RichText::new("Execute ▶").size(13.0).color(COLOR_TEXT)).clicked() {
                        if !app.terminal_input.is_empty() {
                            let cmd = app.terminal_input.clone();
                            terminal::execute_command(app, &cmd);
                            app.terminal_input.clear();
                        }
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Examples:").color(COLOR_MUTED).size(10.0).italics());
                    ui.label(egui::RichText::new("dir | ls | echo test | python script.py").color(COLOR_MUTED).size(10.0).italics());
                });
            });
    }
}



fn render_editor(app: &mut CodeEditorApp, ctx: &egui::Context) {
    render_breadcrumb(app, ctx);
    
    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(COLOR_BG))
        .show(ctx, |ui| {
            if !app.open_files.is_empty() {
                ui.horizontal(|ui| {
                    let open_files = app.open_files.clone();
                    let current_index = app.current_file_index;
                    let mut selected_index = None;
                    let mut close_index = None;
                    
                    for (i, path) in open_files.iter().enumerate() {
                        let tab_name = path.file_name().unwrap().to_string_lossy();
                        let is_active = i == current_index;
                        let tab_color = if is_active { COLOR_ACCENT } else { COLOR_MUTED };
                        
                        ui.horizontal(|ui| {
                            if ui.selectable_label(
                                is_active,
                                egui::RichText::new(format!("  {} ", tab_name)).color(tab_color)
                            ).clicked() {
                                selected_index = Some(i);
                            }
                            if ui.button(egui::RichText::new("✕").color(COLOR_ERROR).size(12.0)).clicked() {
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

            if let Some(path) = &app.file_path.clone() {
                ui.horizontal(|ui| {
                    if ui.button(egui::RichText::new("▶️ Run").color(COLOR_SUCCESS).size(14.0)).clicked() {
                        terminal::run_file(app, path);
                    }
                    if ui.button(egui::RichText::new("🔨 Build").color(COLOR_ACCENT).size(14.0)).clicked() {
                        terminal::build_file(app, path);
                    }
                });
                ui.separator();
            }

            editor::render_editor(app, ui);
        });
}
