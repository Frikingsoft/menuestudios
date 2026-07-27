use std::process::Command;

#[tauri::command]
pub fn reiniciar_sistema() -> Result<String, String> {
    let output = Command::new("systemctl")
        .args(["reboot"])
        .output()
        .map_err(|e| format!("❌ Error al ejecutar systemctl: {}", e))?;

    if output.status.success() {
        Ok("✅ Sistema reiniciado".to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("❌ No se pudo reiniciar: {}", error))
    }
}