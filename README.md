# NeoBot

**Professional Cross-Platform Desktop Application for Automated Kali Linux VM Provisioning with Embedded AI Cybersecurity Agent**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tauri](https://img.shields.io/badge/Tauri-2.x-FFC131?logo=tauri)](https://tauri.app/)
[![SvelteKit](https://img.shields.io/badge/SvelteKit-2.x-FF3E00?logo=svelte)](https://kit.svelte.dev/)
[![GitHub release](https://img.shields.io/github/v/release/iofhouras/neobot?include_prereleases&sort=semver)](https://github.com/iofhouras/neobot/releases)

---

## 🚀 Downloads — Choose Your Platform

**Current Status:** v0.1.0-alpha (Pre-release)  
**Full platform-specific binaries will be attached to the first stable GitHub Release (v0.1.0)**

### Recommended Installers by Operating System

| Platform      | Recommended Download                          | Alternative                  | Approx. Size | Status          |
|---------------|-----------------------------------------------|------------------------------|--------------|-----------------|
| **Linux**     | `NeoBot-0.1.0-x86_64.AppImage`               | `.deb` package              | ~45 MB      | 🔄 Building |
| **Windows**   | `NeoBot-0.1.0-Setup.exe` (NSIS Installer)    | Portable `.zip`             | ~52 MB      | 🔄 Building |
| **macOS**     | `NeoBot-0.1.0.dmg` (Universal Apple Silicon + Intel) | —                     | ~48 MB      | 🔄 Building |

**Quick One-Liner Install (Linux)**
```bash
curl -L https://github.com/iofhouras/neobot/releases/latest/download/NeoBot-0.1.0-x86_64.AppImage -o ~/neobot && \
chmod +x ~/neobot && ~/neobot
```

> **Note:** Once v0.1.0 is released, the links above will automatically point to the latest assets. For now, developers should build from source using the guides below.

---

## 🚀 Overview

NeoBot is a production-grade desktop application built with **Tauri 2** (Rust backend + web frontend) that delivers **one-click automated provisioning** of a fully configured Kali Linux VirtualBox VM, complete with an embedded AI agent (inspired by OpenClaw / CrewAI + LangGraph) specialized for ethical hacking, red teaming, and cybersecurity automation.

### Key Features
- **One-Click VM Setup**: Automatically downloads official Kali OVA, imports into VirtualBox, configures CPU/RAM, starts headless, and pre-installs tools + AI agent.
- **Sleek Cyber-Themed UI**: SvelteKit + TypeScript + Tailwind + shadcn/ui with dark neon aesthetic (tabs: Bot Studio, AI Config, VM Control, Task Monitor, Safety).
- **AI Agent Core**: Runs inside the Kali VM with cybersecurity skills (nmap wrappers, Metasploit automation, custom tool calling).
- **Secure Messaging Bridges**: Control the agent via Telegram, WhatsApp Web, Discord, etc.
- **Professional Installers**: .exe (NSIS), .dmg, .deb/AppImage with one-liner `curl | bash` installer.
- **Security Guardrails**: Rust-based encryption (ring/aes-gcm), user confirmation for risky actions, allowed-target lists.

**Tech Stack** (as recommended):
- **Desktop Framework**: Tauri 2.x (Rust + system webview) — small binaries, high security
- **Frontend**: SvelteKit + TypeScript + TailwindCSS + shadcn/ui
- **Backend (Rust)**: VBoxManage automation, file downloads, encryption, SSH/SCP into VM
- **VM Automation**: VirtualBox + official Kali OVA
- **AI**: Fork of OpenClaw / CrewAI + tool wrappers inside Kali
- **Messaging**: Telegram Bot API, WhatsApp Web.js, Discord.js

---

## 📁 Project Structure

```
neobot/
├── src/                    # Tauri Rust backend
│   ├── main.rs
│   ├── commands/           # vm_control, provision, etc.
│   ├── vbox/               # VirtualBox wrappers
│   └── security/           # Encryption & guardrails
├── src-tauri/             # Tauri config & Cargo.toml
├── frontend/              # SvelteKit app (Bot Studio, Dashboard...)
├── installer/             # NSIS / create-dmg / .deb scripts
├── assets/                # Icons, logos, Kali preconfig
├── kali-preconfig/        # Custom OVA scripts
└── Cargo.toml
```

---

## 🚀 Quick Start (Kali Linux Development Environment)

This repository includes complete setup guides for a fresh Kali Linux 2026.x VM.

### 1. Clone the Repository
```bash
git clone https://github.com/iofhouras/neobot.git
cd neobot
```

### 2. Run the Full Kali Linux Setup (Copy-Paste Blocks)

**Step 1: Update System & Install Basics**
```bash
echo "=== Step 1: Updating Kali and installing basics ===" && \
sudo apt update && sudo apt full-upgrade -y && \
sudo apt install -y curl git build-essential \
  libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev \
  librsvg2-dev libssl-dev pkg-config \
  virtualbox virtualbox-ext-pack p7zip-full && \
echo "✅ Basics installed successfully."
```

**Step 2: Install Rust**
```bash
echo "=== Step 2: Installing Rust (official Tauri one-liner) ===" && \
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
source "$HOME/.cargo/env" && \
rustup default stable && \
rustup update && \
rustup component add rustfmt clippy && \
echo "✅ Rust installed and ready."
```

**Step 3: Install Mise + Node.js LTS + pnpm**
```bash
echo "=== Step 3: Installing mise + Node.js LTS + pnpm ===" && \
curl https://mise.run | sh && \
echo 'eval "$(~/.local/bin/mise activate zsh)"' >> ~/.zshrc && \
source ~/.zshrc && \
mise use --global node@lts && \
mise use --global pnpm@latest && \
node --version && pnpm --version && \
echo "✅ Node.js and pnpm ready."
```

**Step 4: Install Tauri CLI v2**
```bash
echo "=== Step 4: Installing Tauri CLI v2 ===" && \
cargo install tauri-cli --locked --force && \
echo "✅ Tauri CLI v2 installed."
```

**Step 5: Create NeoBot Tauri Project**
```bash
echo "=== Step 5: Creating NeoBot Tauri Project ===" && \
cd ~ && \
pnpm dlx create-tauri-app@latest neobot \
  --template sveltekit \
  --typescript \
  --tailwind \
  --yes && \
cd neobot && \
pnpm install && \
echo "✅ NeoBot project created."
```

**Step 6: Add shadcn/ui + Cyber Theme**
```bash
echo "=== Step 6: Adding shadcn/ui + cyber theme ===" && \
pnpm dlx shadcn-svelte@latest init --yes && \
pnpm dlx shadcn-svelte@latest add button card tabs slider progress dialog && \
echo "✅ UI components and theme ready."
```

**Step 7-9**: See the full **Kali Linux Initial Setup Guide** below for folder structure, Git init, and verification.

---

## 📄 Full Documentation

### Kali Linux Initial Setup Guide

Complete copy-paste setup for a fresh Kali Linux VM (includes all 9 steps + troubleshooting).

**File**: `docs/neobot-kali-linux-setup-guide.md`

### Development Guide — Phases 0–3

Production-grade instructions for advanced UI (cyber theme), Rust backend (VBoxManage commands), and automated download/import system with real-time progress.

**File**: `docs/development-guide-phases-0-3.md`

---

## 🚧 Development Roadmap (Phases 0–5)

**Phase 0**: Initial Project Setup & Environment Fix (Kali repo, dependencies, Rust, Mise, Tauri, shadcn)

**Phase 1**: Advanced User Interface (Cyber Theme, Tailwind config, 4-tab layout: Configure VM, Bot Studio, Progress & Logs, Dashboard)

**Phase 2**: Rust Backend & VirtualBox Commands (`detect_virtualbox`, `setup_kali_vm`, `start_vm`)

**Phase 3**: Automated Download, Import & Progress System (reqwest + sevenz-rust + SHA256 verification + Tauri events)

**Phase 4** (Next): Full UI Integration (form binding, live progress bars, VM control dashboard)

**Phase 5** (Next): Packaging, Cross-Platform Installers, Security Guardrails, GitHub Releases

---

## 🔧 Recommended Development Order (MVP First)

1. Basic Tauri app with VirtualBox detection
2. Automated Kali VM import + start
3. Simple agent (shell command execution via messaging)
4. Full UI + safety features
5. Polish installers + testing on clean machines

**Timeline**: Solid MVP in 3-6 months for a small team or dedicated solo dev.

---

## 🔒 Security & Ethics

- All actions require explicit user confirmation for risky operations.
- Encrypted storage of API keys (ring + aes-gcm).
- Allowed-targets whitelist.
- Designed exclusively for **authorized penetration testing** and **ethical red teaming**.
- **Never** use against systems you do not own or have explicit written permission to test.

---

## 🚀 Next Steps & How to Contribute

1. Clone the repo
2. Follow the **Kali Linux Setup Guide** to get a working dev environment
3. Run `pnpm run tauri dev` to launch the app
4. Implement Phase 4 UI integration
5. Open issues or PRs for new features (VM snapshots, multi-VM support, new messaging bridges, etc.)

**What do you want to dive into next?**
- Detailed Tauri + Rust starter code
- Full `install.sh` skeleton
- UI mockup ideas or component breakdown
- VirtualBox automation examples
- Cost/time estimates or team roles

Let me know the focus, and I'll provide code snippets, diagrams, or deeper guides. This can become a real product.

---

## 📄 License

MIT License — see [LICENSE](LICENSE) for details.

**Created**: May 2026
**Maintainer**: NeoBot Team

---

> **Note**: This is a living project. The GitHub repository will be continuously updated with new phases, code examples, and pre-built binaries as development progresses.