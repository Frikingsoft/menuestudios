use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            
            // Forzar transparencia
            #[cfg(target_os = "linux")]
            {
                // La transparencia debe ser manejada por el compositor
                // Asegurar que la ventana sea transparente
                let _ = window.set_always_on_top(true);
                
                if let Some(monitor) = window.current_monitor().unwrap() {
                    let screen_size = monitor.size();
                    let width = screen_size.width;
                    let height = screen_size.height / 2;
                    let x = 0;
                    let y = (screen_size.height - height) as i32;
                    
                    use tauri::{PhysicalPosition, PhysicalSize};
                    let _ = window.set_size(PhysicalSize { width, height });
                    let _ = window.set_position(PhysicalPosition { x, y });
                }
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}