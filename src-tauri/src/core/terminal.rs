use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use tauri::{AppHandle, Emitter};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TerminalCommand {
    pub command: String,
    pub args: Vec<String>,
    pub working_dir: Option<String>,
    pub timeout_seconds: Option<u64>,
    pub env_vars: Option<HashMap<String, String>>,
}

pub struct TerminalExecutor {
    pub logs: Arc<Mutex<Vec<String>>>,
}

impl TerminalExecutor {
    pub fn new() -> Self {
        Self {
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn execute(&self, app: &AppHandle, cmd: TerminalCommand) -> Result<CommandResult, String> {
        let mut command = Command::new(&cmd.command);
        command.args(&cmd.args);
        
        if let Some(dir) = &cmd.working_dir {
            command.current_dir(dir);
        }
        
        if let Some(envs) = &cmd.env_vars {
            for (key, value) in envs {
                command.env(key, value);
            }
        }
        
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        
        let mut child = command.spawn().map_err(|e| format!("Failed to spawn command: {}", e))?;
        
        let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
        let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;
        
        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);
        
        let mut stdout_lines = String::new();
        let mut stderr_lines = String::new();
        
        // Stream output in real-time
        for line in stdout_reader.lines() {
            if let Ok(l) = line {
                stdout_lines.push_str(&format!("{}\n", l));
                let mut logs = self.logs.lock().await;
                logs.push(l.clone());
                let _ = app.emit("terminal-output", l);
            }
        }
        
        for line in stderr_reader.lines() {
            if let Ok(l) = line {
                stderr_lines.push_str(&format!("{}\n", l));
                let mut logs = self.logs.lock().await;
                logs.push(format!("[ERROR] {}", l));
                let _ = app.emit("terminal-output", format!("[ERROR] {}", l));
            }
        }
        
        let status = child.wait().map_err(|e| format!("Failed to wait for command: {}", e))?;
        
        Ok(CommandResult {
            success: status.success(),
            stdout: stdout_lines,
            stderr: stderr_lines,
            exit_code: status.code().unwrap_or(-1),
        })
    }

    pub async fn execute_with_retry(
        &self,
        app: &AppHandle,
        cmd: TerminalCommand,
        max_retries: u32,
    ) -> Result<CommandResult, String> {
        let mut last_error = String::new();
        
        for attempt in 1..=max_retries {
            match self.execute(app, cmd.clone()).await {
                Ok(result) if result.success => return Ok(result),
                Ok(result) => {
                    last_error = format!("Command failed with exit code {}: {}", result.exit_code, result.stderr);
                }
                Err(e) => {
                    last_error = e;
                }
            }
            
            if attempt < max_retries {
                let _ = app.emit("terminal-output", format!("[RETRY] Attempt {}/{} failed. Retrying...", attempt, max_retries));
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
        
        Err(format!("Command failed after {} retries: {}", max_retries, last_error))
    }
}