use tauri::AppHandle;
use crate::core::terminal::{TerminalExecutor, TerminalCommand, CommandResult};
use std::sync::Arc;
use tokio::sync::Mutex;

static TERMINAL: once_cell::sync::Lazy<Arc<Mutex<TerminalExecutor>>> = once_cell::sync::Lazy::new(|| {
    Arc::new(Mutex::new(TerminalExecutor::new()))
});

#[tauri::command]
pub async fn execute_terminal_command(app: AppHandle, cmd: TerminalCommand) -> Result<CommandResult, String> {
    let executor = TERMINAL.lock().await;
    executor.execute(&app, cmd).await
}

#[tauri::command]
pub async fn execute_with_retry(app: AppHandle, cmd: TerminalCommand, max_retries: u32) -> Result<CommandResult, String> {
    let executor = TERMINAL.lock().await;
    executor.execute_with_retry(&app, cmd, max_retries).await
}