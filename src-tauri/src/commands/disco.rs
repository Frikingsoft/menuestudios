use sysinfo::Disks;

/// Obtiene el porcentaje de uso del primer disco disponible
#[tauri::command]
pub fn get_disk_percentage() -> f32 {
    let disks = Disks::new_with_refreshed_list();

    if let Some(disk) = disks.list().first() {
        let total = disk.total_space() as f32;
        let available = disk.available_space() as f32;
        let used = total - available;

        if total > 0.0 {
            return (used / total) * 100.0;
        }
    }

    0.0
}

/// Obtiene el porcentaje de uso del disco en entero
#[tauri::command]
pub fn get_disk_percentage_int() -> u8 {
    get_disk_percentage().round() as u8
}