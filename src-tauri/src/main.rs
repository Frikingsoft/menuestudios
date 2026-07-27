// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands {
    pub mod firefox;
    pub mod terminal;
    pub mod vscode;
    pub mod apagar;
    pub mod reiniciar;
    pub mod cerrar;
}

use tauri::{Manager, PhysicalPosition, PhysicalSize};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "linux")]
            {
                let _ = window.set_always_on_top(true);

                if let Some(monitor) = window.current_monitor()? {
                    let screen_size = monitor.size();
                    let width = screen_size.width;
                    let height = 48u32;
                    let x = 0i32;
                    let y = (screen_size.height - height) as i32;

                    let _ = window.set_size(PhysicalSize::new(width, height));
                    let _ = window.set_position(PhysicalPosition::new(x, y));
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::firefox::launch_firefox,
            commands::terminal::launch_terminal,
            commands::vscode::launch_vscode,
            commands::apagar::apagar_sistema,
            commands::reiniciar::reiniciar_sistema,
            commands::cerrar::cerrar_sesion,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}