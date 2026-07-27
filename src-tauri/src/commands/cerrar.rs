use std::process::Command;

#[tauri::command]
pub fn cerrar_sesion() -> Result<String, String> {
    let output = Command::new("systemctl")
        .args(["stop", "user-1000.slice"])
        .output()
        .map_err(|e| format!("❌ Error: {}", e))?;

    if output.status.success() {
        Ok("✅ Sesión cerrada".to_string())
    } else {
        Err("❌ No se pudo cerrar sesión".to_string())
    }
}