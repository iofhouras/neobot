// macOS-specific download and installation module
// Handles downloading VirtualBox, NeoBot .dmg, and macOS-specific setup

use std::path::PathBuf;
use tauri::{AppHandle, Emitter};
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::process::Command;

// macOS download URLs (update as needed)
const VIRTUALBOX_MACOS_URL: &str = "https://download.virtualbox.org/virtualbox/7.0.18/VirtualBox-7.0.18-162988-OSX.dmg";
const NEOBOT_MACOS_INSTALLER: &str = "https://github.com/iofhouras/neobot/releases/latest/download/NeoBot-0.1.0.dmg";

const MACOS_DOWNLOAD_DIR: &str = "macos-downloads";

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MacOSDownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    pub percentage: f32,
    pub status: String,
}

#[tauri::command]
pub async fn download_virtualbox_for_macos(app: AppHandle) -> Result<String, String> {
    let download_dir = std::env::current_dir().unwrap().join(MACOS_DOWNLOAD_DIR);
    std::fs::create_dir_all(&download_dir).map_err(|e| e.to_string())?;

    let vbox_path = download_dir.join("VirtualBox-macOS.dmg");

    if vbox_path.exists() {
        return Ok("VirtualBox already downloaded for macOS".to_string());
    }

    emit_macos_progress(&app, 0, 100, 0.0, "Downloading VirtualBox for macOS...");

    let client = Client::new();
    let response = client.get(VIRTUALBOX_MACOS_URL).send().await.map_err(|e| e.to_string())?;
    let total = response.content_length().unwrap_or(0);
    let mut file = File::create(&vbox_path).map_err(|e| e.to_string())?;
    let mut downloaded = 0u64;

    use futures_util::StreamExt;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        file.write_all(&chunk).map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;

        if total > 0 {
            let pct = (downloaded as f32 / total as f32) * 100.0;
            emit_macos_progress(&app, downloaded, total, pct, "Downloading VirtualBox...");
        }
    }

    emit_macos_progress(&app, downloaded, total, 100.0, "VirtualBox for macOS ready");
    Ok(vbox_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn download_neobot_installer_for_macos(app: AppHandle) -> Result<String, String> {
    let download_dir = std::env::current_dir().unwrap().join(MACOS_DOWNLOAD_DIR);
    std::fs::create_dir_all(&download_dir).map_err(|e| e.to_string())?;

    let installer_path = download_dir.join("NeoBot-macOS.dmg");

    if installer_path.exists() {
        return Ok("NeoBot macOS installer already downloaded".to_string());
    }

    emit_macos_progress(&app, 0, 100, 0.0, "Downloading NeoBot for macOS...");

    let client = Client::new();
    let response = client.get(NEOBOT_MACOS_INSTALLER).send().await.map_err(|e| e.to_string())?;

    let mut file = File::create(&installer_path).map_err(|e| e.to_string())?;
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;
    file.write_all(&bytes).map_err(|e| e.to_string())?;

    emit_macos_progress(&app, 0, 100, 100.0, "NeoBot macOS installer ready");
    Ok(installer_path.to_string_lossy().to_string())
}

fn emit_macos_progress(app: &AppHandle, downloaded: u64, total: u64, percentage: f32, status: &str) {
    let _ = app.emit("macos-download-progress", MacOSDownloadProgress {
        downloaded,
        total,
        percentage,
        status: status.to_string(),
    });
}

#[tauri::command]
pub fn get_macos_download_path() -> String {
    std::env::current_dir()
        .unwrap()
        .join(MACOS_DOWNLOAD_DIR)
        .to_string_lossy()
        .to_string()
}