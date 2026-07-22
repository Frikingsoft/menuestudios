// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands {
    pub mod firefox;
    pub mod terminal;  // ← Agregar esta línea
}

use tauri::{Manager, PhysicalPosition, PhysicalSize, command};
use std::process::Command;

// Comando genérico para lanzar cualquier aplicación
#[command]
fn launch_app(command: String) -> Result<String, String> {
    match Command::new(&command)
        .spawn() {
            Ok(_) => Ok(format!("Aplicación '{}' lanzada exitosamente", command)),
            Err(e) => Err(format!("Error al lanzar '{}': {}", command, e))
        }
}

#[command]
fn launch_app_with_args(command: String, args: Vec<String>) -> Result<String, String> {
    match Command::new(&command)
        .args(&args)
        .spawn() {
            Ok(_) => Ok("Aplicación lanzada exitosamente".to_string()),
            Err(e) => Err(format!("Error: {}", e))
        }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "linux")]
            {
                // Mantener la ventana siempre encima
                let _ = window.set_always_on_top(true);

                if let Some(monitor) = window.current_monitor()? {
                    let screen_size = monitor.size();

                    let width = screen_size.width;
                    let height = 48u32;

                    // Posicionar la ventana en la parte inferior
                    let x = 0i32;
                    let y = (screen_size.height - height) as i32;

                    let _ = window.set_size(PhysicalSize::new(width, height));
                    let _ = window.set_position(PhysicalPosition::new(x, y));
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            launch_app,
            launch_app_with_args,
            // Comandos de Firefox
            commands::firefox::launch_firefox,
            commands::firefox::launch_firefox_with_url,
            commands::firefox::launch_firefox_private,
            commands::firefox::launch_firefox_kiosk,
            commands::firefox::check_firefox_installed,
            commands::firefox::get_firefox_version,
            // Comandos de Terminal (Kitty)
            commands::terminal::launch_terminal,
            commands::terminal::launch_terminal_with_directory,
            commands::terminal::launch_terminal_with_command,
            commands::terminal::launch_terminal_fullscreen,
            commands::terminal::launch_terminal_with_title,
            commands::terminal::launch_terminal_with_profile,
            commands::terminal::launch_terminal_with_size,
            commands::terminal::check_kitty_installed,
            commands::terminal::get_kitty_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}