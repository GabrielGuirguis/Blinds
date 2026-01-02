pub mod unified_window;
pub mod unified_window_utils;
pub mod swiftc_screen_capture_interface;

use unified_window_utils::get_all_windows;
use swiftc_screen_capture_interface::check_access;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    let res = check_access();
    format!("Hello, {}! You've been greeted from Rust!, did it work? {}", name, res)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_all_windows])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
