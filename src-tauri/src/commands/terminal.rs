// src-tauri/src/commands/terminal.rs
use std::process::Command;
use tauri::command;

#[command]
pub fn launch_terminal() -> Result<(), String> {
    match Command::new("/usr/bin/kitty")
        .spawn() {
            Ok(_) => {
                println!("✅ Kitty abierto");
                Ok(())
            },
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                Err(format!("Error al abrir Kitty: {}", e))
            }
        }
}