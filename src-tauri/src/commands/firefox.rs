// src-tauri/src/commands/firefox.rs
use std::process::Command;
use tauri::command;

#[command]
pub fn launch_firefox() -> Result<(), String> {
    Command::new("firefox")
        .arg("--fullscreen") 
        .spawn()
        .map_err(|e| format!("Error al abrir Firefox: {}", e))?;
    Ok(())
}