use tauri::AppHandle;
use crate::provisioning::zero_touch_engine::ZeroTouchProvisioningEngine;
use crate::ai_agent::config::SecureCredentialVault;
use std::sync::Arc;

static VAULT: once_cell::sync::Lazy<Arc<SecureCredentialVault>> = once_cell::sync::Lazy::new(|| {
    Arc::new(SecureCredentialVault::new())
});

#[tauri::command]
pub async fn run_zero_touch_provisioning(app: AppHandle, vm_name: String) -> Result<String, String> {
    let engine = ZeroTouchProvisioningEngine::new(VAULT.clone());
    engine.run_full_provisioning(&app, &vm_name).await
}