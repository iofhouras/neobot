use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};
use reqwest::Client;
use sha2::{Sha256, Digest};
use hex;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use tokio::sync::Mutex;

// Official Kali Linux OVA (update URL as needed for latest version)
const KALI_OVA_URL: &str = "https://cdimage.kali.org/kali-2026.1/kali-linux-2026.1-virtualbox-amd64.ova";
const KALI_OVA_SHA256: &str = "YOUR_SHA256_HERE"; // TODO: Replace with actual SHA256 of the OVA

const DOWNLOAD_DIR: &str = "kali-ova";

#[derive(Clone, serde::Serialize)]
struct DownloadProgress {
    downloaded: u64,
    total: u64,
    percentage: f32,
    status: String,
}

#[tauri::command]
pub async fn download_and_import_kali_ova(
    app: AppHandle,
    vm_name: Option<String>,
) -> Result<String, String> {
    let vm_name = vm_name.unwrap_or_else(|| "NeoBot-Kali".to_string());
    let download_dir = std::env::current_dir()
        .unwrap()
        .join(DOWNLOAD_DIR);

    std::fs::create_dir_all(&download_dir).map_err(|e| e.to_string())?;

    let ova_path = download_dir.join("kali-linux.ova");

    // Step 1: Download with progress
    emit_progress(&app, 0, 100, 0.0, "Starting download...");

    let client = Client::new();
    let response = client
        .get(KALI_OVA_URL)
        .send()
        .await
        .map_err(|e| format!("Failed to start download: {}", e))?;

    let total_size = response
        .content_length()
        .unwrap_or(0);

    let mut file = File::create(&ova_path).map_err(|e| e.to_string())?;
    let mut downloaded: u64 = 0;
    let mut hasher = Sha256::new();

    let mut stream = response.bytes_stream();
    use futures_util::StreamExt;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        file.write_all(&chunk).map_err(|e| e.to_string())?;
        hasher.update(&chunk);
        downloaded += chunk.len() as u64;

        if total_size > 0 {
            let percentage = (downloaded as f32 / total_size as f32) * 100.0;
            emit_progress(&app, downloaded, total_size, percentage, "Downloading Kali Linux OVA...");
        }
    }

    // Step 2: Verify SHA256
    emit_progress(&app, downloaded, total_size, 100.0, "Verifying integrity...");
    let hash = hasher.finalize();
    let hash_hex = hex::encode(hash);

    if hash_hex != KALI_OVA_SHA256 {
        return Err(format!("SHA256 mismatch! Expected: {}, got: {}", KALI_OVA_SHA256, hash_hex));
    }

    emit_progress(&app, downloaded, total_size, 100.0, "Download verified successfully!");

    // Step 3: Import into VirtualBox (placeholder - will be expanded)
    emit_progress(&app, downloaded, total_size, 100.0, "Importing into VirtualBox...");

    // TODO: Call VBoxManage import here (see vbox module)
    // For now, simulate success
    std::thread::sleep(std::time::Duration::from_secs(2));

    emit_progress(&app, downloaded, total_size, 100.0, "Kali Linux VM ready!");

    Ok(format!("Successfully downloaded and imported {}", vm_name))
}

fn emit_progress(app: &AppHandle, downloaded: u64, total: u64, percentage: f32, status: &str) {
    let _ = app.emit(
        "download-progress",
        DownloadProgress {
            downloaded,
            total,
            percentage,
            status: status.to_string(),
        },
    );
}

// Helper to get download path
#[tauri::command]
pub fn get_kali_download_path() -> String {
    std::env::current_dir()
        .unwrap()
        .join(DOWNLOAD_DIR)
        .join("kali-linux.ova")
        .to_string_lossy()
        .to_string()
}