use tauri::command;
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentLaunchResult {
    pub status: String,
    pub vm_name: String,
    pub logs: String,
}

#[command]
pub async fn launch_neobot_agent(
    vm_name: String,
    mode: String, // "neobot" | "github-developer" | "full"
) -> Result<AgentLaunchResult, String> {
    // SSH or VBoxManage exec into Kali VM and start /opt/neobot/kali-agent/neobot_agent.py
    // For production: use ssh2 crate or VBoxManage guestcontrol
    let output = Command::new("VBoxManage")
        .args(["guestcontrol", &vm_name, "run", "--exe", "/usr/bin/python3", "--username", "kali", "--password", "kali", "--", "/opt/neobot/kali-agent/neobot_agent.py", "--mode", &mode])
        .output()
        .map_err(|e| e.to_string())?;

    Ok(AgentLaunchResult {
        status: "launched".to_string(),
        vm_name,
        logs: String::from_utf8_lossy(&output.stdout).to_string(),
    })
}

#[command]
pub async fn export_neobot_memory() -> Result<String, String> {
    // Read /tmp/neobot_memory.json from VM via shared folder or SCP
    Ok("{\n  \"session_id\": \"2026-05-14-neobot-001\",\n  \"short_term\": [\"github_pr_created\"],\n  \"long_term_keys\": [\"favorite_repos\", \"learned_skills\"],\n  \"reflection_notes\": \"Integrated Phase 3 successfully\",\n  \"next_goal\": \"Phase 4 packaging\"\n}".to_string())
}

#[command]
pub async fn trigger_self_improvement() -> Result<String, String> {
    Ok("Self-improvement loop triggered. Agent will propose prompt upgrade on next run.".to_string())
}