// src-tauri/src/commands/vscode.rs
use std::process::Command;
use tauri::command;

#[command]
pub fn launch_vscode() -> Result<(), String> {
    Command::new("code")
        .arg("--new-window")  // Forzar nueva ventana
        .spawn()
        .map_err(|e| format!("Error al abrir VSCode: {}", e))?;
    Ok(())
}