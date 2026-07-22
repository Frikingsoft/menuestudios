// src-tauri/src/commands/firefox.rs
use std::process::Command;
use tauri::command;

#[command]
pub fn launch_firefox() -> Result<String, String> {
    match Command::new("firefox")
        .spawn() {
            Ok(_) => Ok("Firefox lanzado exitosamente".to_string()),
            Err(e) => Err(format!("Error al lanzar Firefox: {}", e))
        }
}

#[command]
pub fn launch_firefox_with_url(url: String) -> Result<String, String> {
    match Command::new("firefox")
        .arg(&url)
        .spawn() {
            Ok(_) => Ok(format!("Firefox abrió: {}", url)),
            Err(e) => Err(format!("Error al lanzar Firefox: {}", e))
        }
}

#[command]
pub fn launch_firefox_private() -> Result<String, String> {
    match Command::new("firefox")
        .arg("--private-window")
        .spawn() {
            Ok(_) => Ok("Firefox en modo privado lanzado".to_string()),
            Err(e) => Err(format!("Error: {}", e))
        }
}

#[command]
pub fn launch_firefox_kiosk() -> Result<String, String> {
    match Command::new("firefox")
        .arg("--kiosk")
        .spawn() {
            Ok(_) => Ok("Firefox en modo kiosko lanzado".to_string()),
            Err(e) => Err(format!("Error: {}", e))
        }
}

// ELIMINAR esta función porque no se usa:
// pub fn launch_firefox_profile(profile: String) -> Result<String, String> { ... }

#[command]
pub fn check_firefox_installed() -> Result<bool, String> {
    match Command::new("which")
        .arg("firefox")
        .output() {
            Ok(output) => Ok(output.status.success()),
            Err(e) => Err(format!("Error verificando Firefox: {}", e))
        }
}

#[command]
pub fn get_firefox_version() -> Result<String, String> {
    match Command::new("firefox")
        .arg("--version")
        .output() {
            Ok(output) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout);
                    Ok(version.trim().to_string())
                } else {
                    Err("No se pudo obtener la versión".to_string())
                }
            },
            Err(e) => Err(format!("Error: {}", e))
        }
}