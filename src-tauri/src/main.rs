#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands {
    pub mod firefox;
    pub mod terminal;
    pub mod vscode;
    pub mod apagar;
    pub mod reiniciar;
    pub mod cerrar;
    pub mod ram;
    pub mod cpu;
    pub mod disco;
}

use tauri::{Manager, PhysicalPosition, PhysicalSize, Emitter};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Code, Modifiers, Shortcut, ShortcutState};

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        let _ = app.emit("toggle-nav", ());
                    }
                })
                .build(),
        )
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

            // Alt + Espacio -> Toggle nav
           let shortcut = Shortcut::new(Some(Modifiers::SUPER), Code::Space);
            app.global_shortcut().register(shortcut).unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::firefox::launch_firefox,
            commands::terminal::launch_terminal,
            commands::vscode::launch_vscode,
            commands::apagar::apagar_sistema,
            commands::reiniciar::reiniciar_sistema,
            commands::cerrar::cerrar_sesion,
            commands::ram::get_ram_percentage,
            commands::cpu::get_cpu_percentage_int,
            commands::disco::get_disk_percentage,      
            commands::disco::get_disk_percentage_int,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}