// src-tauri/src/commands/terminal.rs
use std::process::Command;
use tauri::command;

#[command]
pub fn launch_terminal() -> Result<String, String> {
    // Lanzar Kitty terminal
    match Command::new("kitty")
        .spawn() {
            Ok(_) => Ok("Kitty terminal lanzada exitosamente".to_string()),
            Err(e) => Err(format!("Error al lanzar Kitty: {}", e))
        }
}

#[command]
pub fn launch_terminal_with_directory(directory: String) -> Result<String, String> {
    // Lanzar Kitty en un directorio específico
    match Command::new("kitty")
        .arg("--directory")
        .arg(&directory)
        .spawn() {
            Ok(_) => Ok(format!("Kitty abierta en: {}", directory)),
            Err(e) => Err(format!("Error al lanzar Kitty: {}", e))
        }
}

#[command]
pub fn launch_terminal_with_command(command: String) -> Result<String, String> {
    // Lanzar Kitty ejecutando un comando específico
    match Command::new("kitty")
        .arg("--")
        .arg(&command)
        .spawn() {
            Ok(_) => Ok(format!("Kitty ejecutando: {}", command)),
            Err(e) => Err(format!("Error al lanzar Kitty: {}", e))
        }
}

#[command]
pub fn launch_terminal_fullscreen() -> Result<String, String> {
    // Lanzar Kitty en pantalla completa
    match Command::new("kitty")
        .arg("--fullscreen")
        .spawn() {
            Ok(_) => Ok("Kitty en pantalla completa".to_string()),
            Err(e) => Err(format!("Error: {}", e))
        }
}

#[command]
pub fn launch_terminal_with_title(title: String) -> Result<String, String> {
    // Lanzar Kitty con un título personalizado
    match Command::new("kitty")
        .arg("--title")
        .arg(&title)
        .spawn() {
            Ok(_) => Ok(format!("Kitty con título: {}", title)),
            Err(e) => Err(format!("Error: {}", e))
        }
}

#[command]
pub fn launch_terminal_with_profile(profile: String) -> Result<String, String> {
    // Lanzar Kitty con un perfil específico
    match Command::new("kitty")
        .arg("--config")
        .arg(&format!("/path/to/{}.conf", profile))
        .spawn() {
            Ok(_) => Ok(format!("Kitty con perfil: {}", profile)),
            Err(e) => Err(format!("Error: {}", e))
        }
}

#[command]
pub fn launch_terminal_with_size(width: u32, height: u32) -> Result<String, String> {
    // Lanzar Kitty con tamaño específico
    match Command::new("kitty")
        .arg("--size")
        .arg(&format!("{}x{}", width, height))
        .spawn() {
            Ok(_) => Ok(format!("Kitty con tamaño: {}x{}", width, height)),
            Err(e) => Err(format!("Error: {}", e))
        }
}

#[command]
pub fn check_kitty_installed() -> Result<bool, String> {
    // Verificar si Kitty está instalado
    match Command::new("which")
        .arg("kitty")
        .output() {
            Ok(output) => Ok(output.status.success()),
            Err(e) => Err(format!("Error verificando Kitty: {}", e))
        }
}

#[command]
pub fn get_kitty_version() -> Result<String, String> {
    // Obtener la versión de Kitty
    match Command::new("kitty")
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