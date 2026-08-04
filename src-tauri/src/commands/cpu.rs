use sysinfo::System;

#[tauri::command]
pub fn get_cpu_percentage() -> f32 {
    let mut sys = System::new();
    sys.refresh_cpu_all();
    std::thread::sleep(std::time::Duration::from_millis(100));
    sys.refresh_cpu_all();
    
    sys.global_cpu_usage()
}

#[tauri::command]
pub fn get_cpu_percentage_int() -> u8 {
    get_cpu_percentage().round() as u8
}