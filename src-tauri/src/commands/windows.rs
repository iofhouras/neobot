// Windows-specific download and installation module
// Handles downloading VirtualBox, tools, and Windows-specific setup for NeoBot

use std::path::PathBuf;
use tauri::{AppHandle, Emitter};
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::process::Command;

// Windows download URLs (update as needed)
const VIRTUALBOX_WINDOWS_URL: &str = "https://download.virtualbox.org/virtualbox/7.0.18/VirtualBox-7.0.18-162988-Win.exe";
const NEOBOT_WINDOWS_INSTALLER: &str = "https://github.com/iofhouras/neobot/releases/latest/download/NeoBot-Setup.exe";

const WINDOWS_DOWNLOAD_DIR: &str = "windows-downloads";

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsDownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    pub percentage: f32,
    pub status: String,
}

#[tauri::command]
pub async fn download_virtualbox_for_windows(app: AppHandle) -> Result<String, String> {
    let download_dir = std::env::current_dir().unwrap().join(WINDOWS_DOWNLOAD_DIR);
    std::fs::create_dir_all(&download_dir).map_err(|e| e.to_string())?;

    let vbox_path = download_dir.join("VirtualBox-Windows.exe");

    if vbox_path.exists() {
        return Ok("VirtualBox already downloaded".to_string());
    }

    emit_windows_progress(&app, 0, 100, 0.0, "Downloading VirtualBox for Windows...");

    let client = Client::new();
    let response = client.get(VIRTUALBOX_WINDOWS_URL).send().await.map_err(|e| e.to_string())?;
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
            emit_windows_progress(&app, downloaded, total, pct, "Downloading VirtualBox...");
        }
    }

    emit_windows_progress(&app, downloaded, total, 100.0, "VirtualBox download complete");
    Ok(vbox_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn download_neobot_installer_for_windows(app: AppHandle) -> Result<String, String> {
    let download_dir = std::env::current_dir().unwrap().join(WINDOWS_DOWNLOAD_DIR);
    std::fs::create_dir_all(&download_dir).map_err(|e| e.to_string())?;

    let installer_path = download_dir.join("NeoBot-Setup.exe");

    if installer_path.exists() {
        return Ok("NeoBot installer already downloaded".to_string());
    }

    emit_windows_progress(&app, 0, 100, 0.0, "Downloading NeoBot Windows installer...");

    let client = Client::new();
    let response = client.get(NEOBOT_WINDOWS_INSTALLER).send().await.map_err(|e| e.to_string())?;

    let mut file = File::create(&installer_path).map_err(|e| e.to_string())?;
    // Simple download without progress for installer
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;
    file.write_all(&bytes).map_err(|e| e.to_string())?;

    emit_windows_progress(&app, 0, 100, 100.0, "NeoBot Windows installer ready");
    Ok(installer_path.to_string_lossy().to_string())
}

fn emit_windows_progress(app: &AppHandle, downloaded: u64, total: u64, percentage: f32, status: &str) {
    let _ = app.emit("windows-download-progress", WindowsDownloadProgress {
        downloaded,
        total,
        percentage,
        status: status.to_string(),
    });
}

#[tauri::command]
pub fn get_windows_download_path() -> String {
    std::env::current_dir()
        .unwrap()
        .join(WINDOWS_DOWNLOAD_DIR)
        .to_string_lossy()
        .to_string()
}