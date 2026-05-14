use tauri::{AppHandle, Emitter};
use reqwest::Client;
use sha2::{Sha256, Digest};
use hex;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::path::PathBuf;

// Configuration
const KALI_OVA_URL: &str = "https://cdimage.kali.org/kali-2026.1/kali-linux-2026.1-virtualbox-amd64.ova";
const KALI_OVA_SHA256: &str = "PLACEHOLDER_SHA256"; // Replace with real value
const DOWNLOAD_DIR: &str = "kali-ova";

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    pub percentage: f32,
    pub status: String,
}

#[derive(serde::Deserialize)]
pub struct VmConfig {
    pub vm_name: String,
    pub cpus: u32,
    pub memory_mb: u32,
    pub headless: bool,
}

#[tauri::command]
pub async fn setup_kali_vm(app: AppHandle, config: VmConfig) -> Result<String, String> {
    let vm_name = config.vm_name;
    let download_dir = std::env::current_dir().unwrap().join(DOWNLOAD_DIR);
    std::fs::create_dir_all(&download_dir).map_err(|e| e.to_string())?;

    let ova_path = download_dir.join("kali-linux.ova");

    // 1. Check/Install VirtualBox
    ensure_virtualbox(&app)?;

    // 2. Download Kali OVA
    if !ova_path.exists() {
        download_kali_ova(&app, &ova_path).await?;
    }

    // 3. Import & Configure VM
    import_and_configure_vm(&app, &ova_path, &vm_name, config.cpus, config.memory_mb)?;

    // 4. Start VM
    start_vm(&vm_name, config.headless)?;

    Ok(format!("Kali VM '{}' created and launched!", vm_name))
}

fn ensure_virtualbox(app: &AppHandle) -> Result<(), String> {
    if Command::new("VBoxManage").arg("--version").output().is_ok() {
        emit_progress(app, 0, 100, 100.0, "VirtualBox ready");
        return Ok(());
    }
    emit_progress(app, 0, 100, 0.0, "Please install VirtualBox from virtualbox.org");
    Err("VirtualBox not found. Please install it manually.".to_string())
}

async fn download_kali_ova(app: &AppHandle, ova_path: &PathBuf) -> Result<(), String> {
    emit_progress(app, 0, 100, 0.0, "Downloading Kali Linux OVA...");

    let client = Client::new();
    let response = client.get(KALI_OVA_URL).send().await.map_err(|e| e.to_string())?;
    let total = response.content_length().unwrap_or(0);
    let mut file = File::create(ova_path).map_err(|e| e.to_string())?;
    let mut downloaded = 0u64;
    let mut hasher = Sha256::new();

    use futures_util::StreamExt;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        file.write_all(&chunk).map_err(|e| e.to_string())?;
        hasher.update(&chunk);
        downloaded += chunk.len() as u64;

        if total > 0 {
            let pct = (downloaded as f32 / total as f32) * 100.0;
            emit_progress(app, downloaded, total, pct, "Downloading Kali Linux...");
        }
    }

    // Verify
    let hash = hasher.finalize();
    if hex::encode(hash) != KALI_OVA_SHA256 {
        return Err("Kali OVA verification failed".to_string());
    }

    emit_progress(app, downloaded, total, 100.0, "Download verified");
    Ok(())
}

fn import_and_configure_vm(app: &AppHandle, ova_path: &PathBuf, vm_name: &str, cpus: u32, memory: u32) -> Result<(), String> {
    emit_progress(app, 0, 100, 100.0, "Importing into VirtualBox...");

    let _ = Command::new("VBoxManage")
        .args(["import", ova_path.to_str().unwrap(), "--vmname", vm_name])
        .output();

    let _ = Command::new("VBoxManage")
        .args(["modifyvm", vm_name, "--cpus", &cpus.to_string(), "--memory", &memory.to_string()])
        .output();

    emit_progress(app, 0, 100, 100.0, "VM configured");
    Ok(())
}

fn start_vm(vm_name: &str, headless: bool) -> Result<(), String> {
    let mode = if headless { "headless" } else { "gui" };
    Command::new("VBoxManage")
        .args(["startvm", vm_name, "--type", mode])
        .output()
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn emit_progress(app: &AppHandle, downloaded: u64, total: u64, percentage: f32, status: &str) {
    let _ = app.emit("download-progress", DownloadProgress {
        downloaded,
        total,
        percentage,
        status: status.to_string(),
    });
}

#[tauri::command]
pub fn get_kali_path() -> String {
    std::env::current_dir().unwrap().join(DOWNLOAD_DIR).join("kali-linux.ova").to_string_lossy().to_string()
}