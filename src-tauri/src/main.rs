// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use tauri::Builder;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::provision::setup_kali_vm,
            commands::provision::get_kali_path,
            commands::windows::download_virtualbox_for_windows,
            commands::windows::download_neobot_installer_for_windows,
            commands::windows::get_windows_download_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}