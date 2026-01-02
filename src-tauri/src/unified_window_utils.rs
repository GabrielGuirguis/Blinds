use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use crate::unified_window::UnifiedWindow;

static ID_TO_APP_NAME: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

pub fn get_app_name(app_id: &str) -> Option<String> {
    let map: &Mutex<HashMap<String, String>> = ID_TO_APP_NAME.get_or_init(|| Mutex::new(HashMap::new()));
    map.lock().unwrap().get(app_id).cloned()
}

#[tauri::command]
#[cfg(target_os = "windows")]
pub fn get_all_windows() -> Vec<UnifiedWindow> {
    let mut windows = Vec::new();
    windows
}

#[tauri::command]
#[cfg(target_os = "macos")]
pub fn get_all_windows() -> Vec<UnifiedWindow> {
    let windows = Vec::new();
    
    return windows;
}

