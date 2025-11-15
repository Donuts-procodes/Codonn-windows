use std::process::{Command, Stdio};
use std::sync::Arc;
// use std::io::BufRead;
use std::thread;
use crate::app::CodeEditorApp;
use std::path::Path;

pub fn open_powershell(app: &mut CodeEditorApp) {
    let output_clone = Arc::clone(&app.terminal_output);
    
    thread::spawn(move || {
        match Command::new("powershell")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(_child) => {
                let mut output = output_clone.lock().unwrap();
                output.push_str("PowerShell Started\n");
                output.push_str("Type 'exit' to close\n");
                output.push_str("> ");
            }
            Err(e) => {
                let mut output = output_clone.lock().unwrap();
                output.push_str(&format!("Error: {}\n> ", e));
            }
        }
    });
}

pub fn open_cmd(app: &mut CodeEditorApp) {
    let output_clone = Arc::clone(&app.terminal_output);
    
    thread::spawn(move || {
        match Command::new("cmd")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(_child) => {
                let mut output = output_clone.lock().unwrap();
                output.push_str("CMD Started\n");
                output.push_str("Type 'exit' to close\n");
                output.push_str("> ");
            }
            Err(e) => {
                let mut output = output_clone.lock().unwrap();
                output.push_str(&format!("Error: {}\n> ", e));
            }
        }
    });
}

pub fn run_file(app: &mut CodeEditorApp, path: &Path) {
    let output_clone = Arc::clone(&app.terminal_output);
    let path_buf = path.to_path_buf();
    
    thread::spawn(move || {
        let mut output = output_clone.lock().unwrap();
        output.push_str(&format!("▶️ Running: {}\n", path_buf.display()));
        drop(output);
        
        let extension = path_buf.extension().and_then(|s| s.to_str()).unwrap_or("");
        
        let result = match extension {
            "rs" => {
                let mut output = output_clone.lock().unwrap();
                output.push_str("Compiling Rust...\n");
                drop(output);
                Command::new("rustc").arg(&path_buf).output()
            }
            "py" => {
                Command::new("python").arg(&path_buf).output()
            }
            "js" => {
                Command::new("node").arg(&path_buf).output()
            }
            _ => {
                let mut output = output_clone.lock().unwrap();
                output.push_str("❌ Unsupported file type\n");
                output.push_str("> ");
                return;
            }
        };
        
        match result {
            Ok(output_result) => {
                let mut output = output_clone.lock().unwrap();
                let stdout = String::from_utf8_lossy(&output_result.stdout);
                if !stdout.is_empty() {
                    output.push_str(&stdout);
                }
                if !output_result.stderr.is_empty() {
                    let stderr = String::from_utf8_lossy(&output_result.stderr);
                    output.push_str("❌ Error:\n");
                    output.push_str(&stderr);
                }
                output.push_str("\n> ");
            }
            Err(e) => {
                let mut output = output_clone.lock().unwrap();
                output.push_str(&format!("❌ Error: {}\n> ", e));
            }
        }
    });
}

pub fn build_file(app: &mut CodeEditorApp, path: &Path) {
    let output_clone = Arc::clone(&app.terminal_output);
    let path_buf = path.to_path_buf();
    
    thread::spawn(move || {
        let mut output = output_clone.lock().unwrap();
        output.push_str(&format!("🔨 Building: {}\n", path_buf.display()));
        drop(output);
        
        match Command::new("cargo")
            .args(&["build", "--release"])
            .output()
        {
            Ok(result) => {
                let mut output = output_clone.lock().unwrap();
                output.push_str(&String::from_utf8_lossy(&result.stdout));
                if !result.stderr.is_empty() {
                    output.push_str(&String::from_utf8_lossy(&result.stderr));
                }
                output.push_str("\n> ");
            }
            Err(e) => {
                let mut output = output_clone.lock().unwrap();
                output.push_str(&format!("Build Error: {}\n> ", e));
            }
        }
    });
}

pub fn execute_command(app: &mut CodeEditorApp, command: &str) {
    let output_clone = Arc::clone(&app.terminal_output);
    let command = command.to_string();
    
    thread::spawn(move || {
        let mut output = output_clone.lock().unwrap();
        output.push_str(&format!("$ {}\n", command));
        drop(output);
        
        let result = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &command])
                .output()
        } else {
            Command::new("sh")
                .args(&["-c", &command])
                .output()
        };
        
        match result {
            Ok(output_result) => {
                let mut output = output_clone.lock().unwrap();
                let stdout = String::from_utf8_lossy(&output_result.stdout);
                if !stdout.is_empty() {
                    output.push_str(&stdout);
                }
                if !output_result.stderr.is_empty() {
                    let stderr = String::from_utf8_lossy(&output_result.stderr);
                    output.push_str(&stderr);
                }
                output.push_str("\n> ");
            }
            Err(e) => {
                let mut output = output_clone.lock().unwrap();
                output.push_str(&format!("Error: {}\n> ", e));
            }
        }
    });
}
