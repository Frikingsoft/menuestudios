use std::process::Command;

#[tauri::command]
pub fn apagar_sistema() -> Result<String, String> {
    let output = Command::new("systemctl")
        .args(["poweroff"])
        .output()
        .map_err(|e| format!("❌ Error al ejecutar systemctl: {}", e))?;

    if output.status.success() {
        Ok("✅ Sistema apagado".to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("❌ No se pudo apagar: {}", error))
    }
}