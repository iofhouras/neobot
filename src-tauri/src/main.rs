// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod core;

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
            commands::macos::download_virtualbox_for_macos,
            commands::macos::download_neobot_installer_for_macos,
            commands::macos::get_macos_download_path,
            commands::terminal::execute_terminal_command,
            commands::terminal::execute_with_retry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}