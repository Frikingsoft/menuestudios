/// Obtiene el volumen actual del sistema (0-100)
#[tauri::command]
pub fn get_volume() -> u8 {
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        
        if let Ok(output) = Command::new("pactl")
            .args(["get-sink-volume", "@DEFAULT_SINK@"])
            .output() 
        {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                for line in output_str.lines() {
                    if let Some(percent) = line.split('%').next() {
                        if let Some(last_word) = percent.split_whitespace().last() {
                            if let Ok(vol) = last_word.parse::<u8>() {
                                return vol;
                            }
                        }
                    }
                }
            }
        }
        
        return 50;
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        50
    }
}

/// Cambia el volumen del sistema
#[tauri::command]
pub fn set_volume(volume: u8) -> Result<String, String> {
    let vol = volume.min(100);
    
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        
        let result = Command::new("pactl")
            .args(["set-sink-volume", "@DEFAULT_SINK@", &format!("{}%", vol)])
            .output();
        
        if result.is_ok() {
            return Ok(format!("Volumen establecido a {}%", vol));
        }
        
        return Err("No se pudo cambiar el volumen".to_string());
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        Err("Cambio de volumen no soportado".to_string())
    }
}

/// Alterna el mute del sistema
#[tauri::command]
pub fn toggle_mute() -> Result<String, String> {
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        
        let result = Command::new("pactl")
            .args(["set-sink-mute", "@DEFAULT_SINK@", "toggle"])
            .output();
        
        if result.is_ok() {
            return Ok("Mute toggled".to_string());
        }
        
        return Err("No se pudo cambiar el mute".to_string());
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        Err("Mute no soportado".to_string())
    }
}