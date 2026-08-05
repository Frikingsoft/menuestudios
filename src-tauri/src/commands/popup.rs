use tauri::{Manager, WebviewWindowBuilder};

#[tauri::command]
pub fn open_audio_popup(app: tauri::AppHandle) -> Result<(), String> {
    // Verificar si la ventana ya existe
    if let Some(window) = app.get_webview_window("audio-popup") {
        let _ = window.close();
    }
    
    // Usar una URL que funcione en desarrollo
    // El archivo debe estar en la carpeta public/ del frontend
    let url = "http://localhost:5173/audio.html";
    
    println!("🔊 Abriendo popup desde: {}", url);
    
    let window = WebviewWindowBuilder::new(
        &app,
        "audio-popup",
        tauri::WebviewUrl::External(url.parse().unwrap())
    )
    .inner_size(400.0, 300.0)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .focused(true)
    .resizable(false)
    .build()
    .map_err(|e| {
        println!("❌ Error: {}", e);
        e.to_string()
    })?;
    
    let _ = window.set_decorations(false);
    let _ = window.set_title("");
    
    // Posicionar encima de la barra
    if let Some(main_window) = app.get_webview_window("main") {
        if let Ok(main_position) = main_window.outer_position() {
            let x = main_position.x;
            let y = main_position.y - 300;
            let _ = window.set_position(tauri::PhysicalPosition::new(x, y));
        }
    }
    
    Ok(())
}

#[tauri::command]
pub fn close_audio_popup(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("audio-popup") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}