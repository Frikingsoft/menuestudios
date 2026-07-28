use std::fs::File;
use std::io::Read;
use tauri::command;

#[command]
pub fn get_ram_percentage() -> Result<f64, String> {
    let mut file = File::open("/proc/meminfo")
        .map_err(|e| format!("Error al abrir /proc/meminfo: {}", e))?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Error al leer /proc/meminfo: {}", e))?;
    
    let total = get_mem_value(&contents, "MemTotal:")?;
    let available = get_mem_value(&contents, "MemAvailable:")?;
    
    let used = total - available;
    let percentage = (used as f64 / total as f64) * 100.0;
    
    Ok(percentage)
}

fn get_mem_value(contents: &str, key: &str) -> Result<u64, String> {
    contents
        .lines()
        .find(|line| line.starts_with(key))
        .ok_or_else(|| format!("No se encontró '{}' en /proc/meminfo", key))?
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| format!("Error al parsear '{}'", key))?
        .parse::<u64>()
        .map_err(|e| format!("Error al convertir valor: {}", e))
}