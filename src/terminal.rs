use std::process::Command;
use std::sync::Arc;
use crate::app::CodeEditorApp;
use std::path::Path;

pub fn open_powershell(app: &mut CodeEditorApp) {
    let output_clone = Arc::clone(&app.terminal_output);
    std::thread::spawn(move || {
        let mut output = output_clone.lock().unwrap();
        output.push_str("PowerShell Started\n");
    });
}

pub fn open_cmd(app: &mut CodeEditorApp) {
    let output_clone = Arc::clone(&app.terminal_output);
    std::thread::spawn(move || {
        let mut output = output_clone.lock().unwrap();
        output.push_str("CMD Started\n");
    });
}

pub fn run_file(app: &mut CodeEditorApp, path: &Path) {
    let output_clone = Arc::clone(&app.terminal_output);
    let path_buf = path.to_path_buf();
    
    std::thread::spawn(move || {
        let mut output = output_clone.lock().unwrap();
        output.push_str(&format!("▶️ Running: {}\n", path_buf.display()));
        
        let extension = path_buf.extension().and_then(|s| s.to_str()).unwrap_or("");
        
        let result = match extension {
            "rs" => {
                output.push_str("Compiling Rust...\n");
                Command::new("rustc").arg(&path_buf).output()
            }
            "py" => {
                Command::new("python").arg(&path_buf).output()
            }
            "js" => {
                Command::new("node").arg(&path_buf).output()
            }
            _ => {
                output.push_str("Unsupported file type\n");
                return;
            }
        };
        
        match result {
            Ok(output_result) => {
                output.push_str(&String::from_utf8_lossy(&output_result.stdout));
                if !output_result.stderr.is_empty() {
                    output.push_str(&String::from_utf8_lossy(&output_result.stderr));
                }
            }
            Err(e) => {
                output.push_str(&format!("Error: {}\n", e));
            }
        }
        output.push_str("\n> ");
    });
}

pub fn build_file(app: &mut CodeEditorApp, path: &Path) {
    let output_clone = Arc::clone(&app.terminal_output);
    let path_buf = path.to_path_buf();
    
    std::thread::spawn(move || {
        let mut output = output_clone.lock().unwrap();
        output.push_str(&format!("🔨 Building: {}\n", path_buf.display()));
        
        match Command::new("cargo").args(&["build", "--release"]).output() {
            Ok(result) => {
                output.push_str(&String::from_utf8_lossy(&result.stdout));
                if !result.stderr.is_empty() {
                    output.push_str(&String::from_utf8_lossy(&result.stderr));
                }
            }
            Err(e) => {
                output.push_str(&format!("Build Error: {}\n", e));
            }
        }
        output.push_str("\n> ");
    });
}
