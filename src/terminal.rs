use std::process::Command;
use std::sync::Arc;
use crate::app::CodeEditorApp;

pub fn run_command(app: &mut CodeEditorApp, program: &str, arg1: &str, arg2: &str, _arg3: &str) {
    let output_clone = Arc::clone(&app.terminal_output);
    let program = program.to_string();
    let arg1 = arg1.to_string();
    let arg2 = arg2.to_string();
    
    std::thread::spawn(move || {
        let mut output = output_clone.lock().unwrap();
        output.push_str(&format!("$ {} {} {}\n", program, arg1, arg2));
        
        match Command::new(&program)
            .arg(&arg1)
            .arg(&arg2)
            .output()
        {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let stderr = String::from_utf8_lossy(&out.stderr);
                if !stdout.is_empty() {
                    output.push_str(&stdout);
                }
                if !stderr.is_empty() {
                    output.push_str(&format!("Error: {}\n", stderr));
                }
            }
            Err(e) => {
                output.push_str(&format!("Failed to run: {}\n", e));
            }
        }
    });
}
