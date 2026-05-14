# NeoBot

**Enterprise-Grade AI-Powered Cybersecurity Automation Platform**

<div align="center">
  <img src="https://raw.githubusercontent.com/iofhouras/neobot/main/assets/neobot-logo.png" alt="NeoBot Logo" width="220"/>
  
  <h2>One-Click Kali Linux VM + Autonomous AI Agent</h2>
  
  <p><strong>Control everything through WhatsApp, Telegram, or Signal</strong></p>
</div>

---

## ⬇️ Download NeoBot

<div align="center">

### ✨ Best Experience: Visit the Official Download Page

<a href="https://iofhouras.github.io/neobot/download.html" style="display: inline-block; background: linear-gradient(90deg, #00ff9d, #00b8ff); color: #0a0a0a; padding: 18px 40px; border-radius: 50px; text-decoration: none; font-weight: bold; font-size: 20px; box-shadow: 0 10px 30px rgba(0, 255, 157, 0.4); transition: transform 0.2s;">
    🚀 Open Beautiful Download Page
</a>

<p><em>Auto-detects your device + beautiful cyberpunk design</em></p>

### Or Download Directly

| Platform   | Direct Link                                                                 | 
|------------|-----------------------------------------------------------------------------|
| **Windows** | [Download Setup.exe](https://github.com/iofhouras/neobot/releases/latest/download/NeoBot-Setup.exe) |
| **macOS**   | [Download Universal.dmg](https://github.com/iofhouras/neobot/releases/latest/download/NeoBot-0.1.0.dmg) |
| **Linux**   | [Download AppImage](https://github.com/iofhouras/neobot/releases/latest/download/NeoBot-x86_64.AppImage) |

</div>

---

## What is NeoBot?

**NeoBot** is a production-ready, cross-platform desktop application that delivers **zero-touch deployment** of a fully hardened Kali Linux virtual machine with an embedded autonomous AI agent.

Control the AI agent using natural language through WhatsApp, Telegram, or Signal.

## System Architecture

```mermaid
graph TD
    %% User Layer
    subgraph User["👤 User Layer"]
        U["User via Messenger<br/>(WhatsApp / Telegram / Signal)"]
    end

    %% Messaging Platform
    subgraph Messaging["📱 Messaging Platform Layer"]
        M["Messaging Platform<br/>(API + Webhook)"]
    end

    %% NeoBot Application
    subgraph NeoBot["🖥️ NeoBot Application (Tauri + Rust)"]
        N1["Pre-Installation Wizard"]
        N2["Secure Credential Vault<br/>(Encrypted API Keys)"]
        N3["Zero-Touch Provisioning Engine"]
    end

    %% Kali Linux VM
    subgraph KaliVM["🐧 Kali Linux Virtual Machine"]
        K1["AI Agent Core<br/>(Grok + Tools)"]
        K2["Execution Engine<br/>(Root Privileges)"]
        K3["Toolchain<br/>(nmap, Metasploit, etc.)"]
    end

    %% Data Flows
    U -->|1. Natural Language Command + API Key| M
    M -->|2. Authenticated Message + Context| N1
    N1 -->|3. Encrypted Credentials| N2
    N2 -->|4. Trigger Provisioning| N3
    N3 -->|5. Download + Verify + Install VM| K1
    K1 -->|6. Inject Credentials + Configure| K2
    K2 -->|7. Execute Tools & Commands| K3
    K3 -->|8. Results + Artifacts| K1
    K1 -->|9. Response via Messenger| M
    M -->|10. Final Output to User| U

    %% Security Boundaries
    classDef security fill:#fff3cd,stroke:#856404,stroke-width:2px
    class N2,K2 security

    %% Legend
    classDef userLayer fill:#e3f2fd,stroke:#1565c0
    classDef messagingLayer fill:#f3e5f5,stroke:#7b1fa2
    classDef appLayer fill:#e8f5e9,stroke:#2e7d32
    classDef vmLayer fill:#fff8e1,stroke:#f57c00

    class U userLayer
    class M messagingLayer
    class N1,N2,N3 appLayer
    class K1,K2,K3 vmLayer
```

**Architecture Highlights:**
- **Layered Design** with clear security boundaries
- **End-to-End Encryption** for credentials and communication
- **Zero-Trust Model** with API key validation at every step
- **Autonomous Execution** inside isolated Kali Linux VM with root privileges
- **Real-time Bidirectional Flow** between user and AI agent

## Key Features

| Category                    | Description |
|-----------------------------|-----------|
| **Zero-Touch Provisioning** | Fully automated VM creation + AI agent deployment in under 15 minutes |
| **Conversational AI Agent** | Control via WhatsApp, Telegram, or Signal using natural language |
| **Enterprise Security**     | Encrypted credential vault + zero-trust architecture |
| **Advanced Toolchain**      | 15+ pre-installed pentesting tools with intelligent orchestration |
| **Cross-Platform**          | Native apps for Windows, macOS, and Linux |

## Quick Start

1. **Download** the version for your platform above
2. **Run the installer** and follow the 5-step wizard
3. **Configure** your AI agent (Grok API + Messenger)
4. **Launch** your fully provisioned Kali Linux VM

## Roadmap

- [x] Core VM + AI Agent System
- [x] Multi-Platform Support
- [ ] Full WhatsApp + Telegram Integration
- [ ] Vector Memory & Long-term Context
- [ ] Plugin Marketplace

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md)

## Security & Ethics

**For authorized penetration testing and ethical security research only.**

## License

MIT License

---

<div align="center">
  <p><strong>Ready to get started?</strong> Click the big green button above.</p>
</div>