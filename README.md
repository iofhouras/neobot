# NeoBot

**Enterprise-Grade AI-Powered Cybersecurity Automation Platform**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tauri](https://img.shields.io/badge/Tauri-2.x-FFC131?logo=tauri)](https://tauri.app/)
[![SvelteKit](https://img.shields.io/badge/SvelteKit-2.x-FF3E00?logo=svelte)](https://kit.svelte.dev/)
[![Rust](https://img.shields.io/badge/Rust-1.85+-orange?logo=rust)](https://www.rust-lang.org/)
[![GitHub release](https://img.shields.io/github/v/release/iofhouras/neobot?include_prereleases&sort=semver)](https://github.com/iofhouras/neobot/releases)

---

<div align="center">
  <img src="https://raw.githubusercontent.com/iofhouras/neobot/main/assets/neobot-hero.png" alt="NeoBot Hero" width="800"/>
  
  <p><strong>One-Click Kali Linux VM Provisioning • Autonomous AI Agent • Real-time Messenger Control</strong></p>
  
  <p>
    <a href="#-quick-start">Quick Start</a> •
    <a href="#-features">Features</a> •
    <a href="#-architecture">Architecture</a> •
    <a href="#-roadmap">Roadmap</a>
  </p>
</div>

---

## What is NeoBot?

**NeoBot** is a production-ready, cross-platform desktop application that delivers **zero-touch deployment** of a fully hardened Kali Linux virtual machine with an embedded autonomous AI agent. Control everything through natural language via WhatsApp, Telegram, or Signal.

Built for ethical hackers, red teamers, and security researchers who demand speed, security, and automation.

## Key Features

| Category              | Highlights |
|-----------------------|----------|
| **Zero-Touch Provisioning** | Fully automated VM creation, hardening, and AI agent deployment in under 15 minutes |
| **AI Agent Orchestration** | Persistent, conversational AI agent controllable via WhatsApp, Telegram, Signal |
| **Enterprise Security** | Encrypted credential vault, zero-trust architecture, sandboxed execution |
| **Toolchain Automation** | 15+ pre-installed pentesting tools with intelligent orchestration |
| **Cross-Platform** | Native builds for Windows, macOS, and Linux with professional installers |
| **Real-time Control** | Bidirectional messaging with low-latency command execution |

## Quick Start

### 1. Download & Install

**Click the links below to download the latest version for your platform:**

| Platform   | Direct Download Link                                                                 | Status  |
|------------|---------------------------------------------------------------------------------------|---------|
| **Windows** | [⬇️ Download NeoBot-Setup.exe](https://github.com/iofhouras/neobot/releases/latest/download/NeoBot-Setup.exe) | Stable |
| **macOS**   | [⬇️ Download NeoBot-Universal.dmg](https://github.com/iofhouras/neobot/releases/latest/download/NeoBot-0.1.0.dmg) | Stable |
| **Linux**   | [⬇️ Download NeoBot-x86_64.AppImage](https://github.com/iofhouras/neobot/releases/latest/download/NeoBot-x86_64.AppImage) | Stable |

> **Note:** These links always point to the latest release. The files will be available once the first stable version (v0.1.0) is published.

### 2. Run the Setup Wizard

After downloading and installing, launch NeoBot and follow the guided 5-step wizard:

1. **System Check** — Verify your environment
2. **VM Configuration** — Set CPU, RAM, and disk size
3. **AI Agent Setup** — Enter Grok API key and messenger tokens
4. **Zero-Touch Provisioning** — Automated installation
5. **Launch** — Start your Kali Linux VM

### 3. Start Using

Once setup is complete, simply message your AI agent on WhatsApp or Telegram:

> "Run a full reconnaissance on target.example.com"

## Architecture

```mermaid
graph TD
    A[User] -->|Natural Language| B[Messenger Platform]
    B --> C[NeoBot Backend]
    C --> D[Kali Linux VM]
    D --> E[AI Agent + Tools]
    E --> F[Execution Engine]
    F --> G[Results]
    G --> B
```

**Core Components:**
- **Frontend**: SvelteKit + Tailwind + shadcn/ui (Cyberpunk theme)
- **Backend**: Rust + Tauri 2 (secure, lightweight)
- **VM Layer**: VirtualBox with automated provisioning
- **AI Layer**: Grok + extensible tool system (OpenClaw-inspired)

## Project Structure

```
neobot/
├── src-tauri/              # Rust backend
│   ├── src/
│   ├── commands/         # Tauri commands
│   ├── core/             # Terminal, security, AI
│   ├── provisioning/     # Zero-touch engine
│   ├── ai_agent/         # Agent orchestration
├── frontend/               # SvelteKit UI
├── assets/                 # Icons, logos
├── docs/                   # Documentation
└── .github/                # Workflows, templates
```

## Roadmap

- [x] Core VM Provisioning Engine
- [x] AI Agent Framework
- [x] Multi-Platform Support
- [ ] Full Messenger Integration (WhatsApp + Telegram)
- [ ] Vector Memory + Long-term Context
- [ ] Plugin Marketplace
- [ ] Enterprise SSO & Audit Logging
- [ ] Mobile Companion App

## Contributing

We welcome contributions! Please read our [Contributing Guidelines](CONTRIBUTING.md) before submitting PRs.

## Security & Ethics

**NeoBot is designed exclusively for authorized penetration testing and ethical security research.**

- All actions require explicit user confirmation
- Encrypted credential storage
- Full audit logging
- Never use against systems you do not have permission to test

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

---

<div align="center">
  <p>Made with ❤️ by the NeoBot Team</p>
  <p>Questions? Open an issue or join our discussions.</p>
</div>