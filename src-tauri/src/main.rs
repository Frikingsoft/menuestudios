use tauri::{Manager, PhysicalPosition, PhysicalSize};

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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}