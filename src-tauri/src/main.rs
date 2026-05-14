// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use tauri::Builder;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::provision::download_and_import_kali_ova,
            commands::provision::get_kali_download_path,
            // Add other commands here (vm_control, etc.)
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}