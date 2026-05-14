# NeoBot Development Guide
## Phases 0–3: Complete Production-Grade Instructions

**For Kali Linux 2026.x VM (Running as root under zsh)**

**Project Goal**: Build a professional cross-platform desktop application (Tauri 2 + SvelteKit + TypeScript) that automatically downloads, installs, and launches a fully configured Kali Linux VM in VirtualBox with one click.

---

## Phase 0: Initial Project Setup & Environment Fix

**Run these commands first to fix repository and prepare the system.**

```bash
# Fix Kali Repository (Critical)
sudo cp /etc/apt/sources.list /etc/apt/sources.list.backup 2>/dev/null || true
cat << EOF | sudo tee /etc/apt/sources.list
deb http://http.kali.org/kali kali-rolling main contrib non-free non-free-firmware
EOF
sudo apt clean
sudo rm -rf /var/lib/apt/lists/*
sudo apt update --fix-missing -y
sudo apt full-upgrade -y

# Install Core Dependencies
sudo apt install -y \
  libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev \
  libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev pkg-config \
  libxdo-dev libfuse2 git virtualbox p7zip-full
```

**Full Phase 0 Setup Script** (Recommended):

```bash
nano ~/neobot-phase0.sh
# Paste the full fixed script from previous response or continue manually
```

**Toolchain Installation**:

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustup default stable && rustup update

# Mise + Node.js + pnpm
curl https://mise.run | sh
echo 'eval "$(~/.local/bin/mise activate zsh)"' >> ~/.zshrc
source ~/.zshrc
mise use --global node@lts
mise use --global pnpm@latest

# Tauri CLI
cargo install tauri-cli --locked --force
```

**Create Project**:

```bash
cd ~
pnpm dlx create-tauri-app@latest neobot --template sveltekit --typescript --tailwind --yes
cd neobot
pnpm install
pnpm dlx shadcn-svelte@latest init --yes
pnpm dlx shadcn-svelte@latest add button card tabs slider progress dialog
```

---

## Phase 1: Advanced User Interface (Cyber Theme)

**Configure Tailwind**:

```bash
cat > tailwind.config.js << 'EOF'
module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        cyber: { bg: '#0a0a0a', accent: '#00ff9d', neon: '#00b8ff' }
      }
    }
  },
  darkMode: 'class'
};
EOF
```

**Main Layout** (`src/routes/+layout.svelte`):
- Use the cyber-themed layout with 4 tabs: Configure VM, Bot Studio, Progress & Logs, Dashboard.

**Verification**:

```bash
cd ~/neobot
pnpm run tauri dev
```

---

## Phase 2: Rust Backend & VirtualBox Commands

**Update `src-tauri/Cargo.toml`** with:

```toml
tauri = { version = "2", features = ["devtools", "process", "dialog"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["progress", "stream"] }
indicatif = "0.17"
anyhow = "1.0"
```

**Key Commands** (`src/commands/vm_control.rs`):
- `detect_virtualbox()`
- `setup_kali_vm(cpus, memory_mb, vm_name)`
- `start_vm(vm_name, headless)`

**Register in `src/main.rs`** using `tauri::generate_handler![]`.

**Verification**:

```bash
cd ~/neobot
cargo check -p neobot
pnpm run tauri dev
```

---

## Phase 3: Automated Download, Import & Progress System

**Add More Crates**:

```toml
sevenz-rust = "0.6"
sha2 = "0.10"
hex = "0.4"
futures-util = "0.3"
```

**Core Command** (`src/commands/provision.rs`):
- `download_and_import_kali_ova(window, vm_name)` with:
  - Real-time progress via Tauri events (`download-progress`)
  - SHA256 verification
  - VBoxManage import

**Register the command** in `main.rs`.

**Verification**:

```bash
cargo check -p neobot
pnpm run tauri dev
```

---

## Next Steps After Completing Phases 0–3

- Proceed to Phase 4: Full UI Integration (Progress bars + form binding)
- Phase 5: Packaging, installers, security guardrails, and release

**Git Commands**:

```bash
cd ~/neobot
git init
git add .
git commit -m "NeoBot Phases 0-3 Complete"
```

**Troubleshooting Tips**:
- Always run `source ~/.zshrc` after toolchain changes.
- Ensure Nested Virtualization is enabled in VirtualBox settings.
- Keep at least 20 GB free disk space.

---

**End of Guide – Phases 0 to 3**

You now have a solid, production-grade foundation for NeoBot.