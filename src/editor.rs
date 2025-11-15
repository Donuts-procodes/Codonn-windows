use eframe::egui;
use crate::app::CodeEditorApp;

pub fn render_editor(app: &mut CodeEditorApp, ui: &mut egui::Ui) {
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
                    .desired_rows(25);
                
                let _response = ui.add(text_edit);
                
                // Auto-completion on text change
                auto_complete_brackets(&mut app.text);
            });
        });
}

fn auto_complete_brackets(text: &mut String) {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    
    if len < 1 {
        return;
    }
    
    let last_char = chars[len - 1];
    
    match last_char {
        '(' => {
            if len >= text.len() || chars.get(len).map_or(true, |&c| c != ')') {
                text.push(')');
            }
        }
        '[' => {
            if len >= text.len() || chars.get(len).map_or(true, |&c| c != ']') {
                text.push(']');
            }
        }
        '{' => {
            if len >= text.len() || chars.get(len).map_or(true, |&c| c != '}') {
                text.push('}');
            }
        }
        '"' => {
            let quote_count = text.matches('"').count();
            if quote_count % 2 == 1 {
                text.push('"');
            }
        }
        '\'' => {
            let quote_count = text.matches('\'').count();
            if quote_count % 2 == 1 {
                text.push('\'');
            }
        }
        _ => {}
    }
}
