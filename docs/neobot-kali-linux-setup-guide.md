# NeoBot Tauri Project - Kali Linux Initial Setup Guide

This guide sets up a complete development environment on Kali Linux for the 
NeoBot Tauri application (SvelteKit + TypeScript + Tailwind CSS + shadcn/ui).

**IMPORTANT NOTES:**
- Run all commands as a regular user (not root).
- Each step is provided as a single copy-paste block.
- Some steps may take several minutes to complete.
- A reboot or new terminal session may be required after certain installations.
- Kali Linux is assumed to be freshly installed in a VirtualBox VM.

---

## STEP 1: Update System & Install All Basic Dependencies

Copy and paste this entire block into your terminal:

```bash
echo "=== Step 1: Updating Kali and installing basics ===" && \
sudo apt update && sudo apt full-upgrade -y && \
sudo apt install -y curl git build-essential \
  libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev \
  librsvg2-dev libssl-dev pkg-config \
  virtualbox virtualbox-ext-pack p7zip-full && \
echo "✅ Basics installed successfully."
```

---

## STEP 2: Install Rust (Official Tauri-Recommended One-Liner)

Copy and paste this entire block into your terminal:

```bash
echo "=== Step 2: Installing Rust (official Tauri one-liner) ===" && \
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
source "$HOME/.cargo/env" && \
rustup default stable && \
rustup update && \
rustup component add rustfmt clippy && \
echo "✅ Rust installed and ready."
```

---

## STEP 3: Install Mise (Universal Version Manager) + Node.js LTS + pnpm

Copy and paste this entire block into your terminal:

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

> **Note**: This step configures zsh. If you use bash, you may need to adjust the shell configuration.

---

## STEP 4: Install Tauri CLI v2

Copy and paste this entire block into your terminal:

```bash
echo "=== Step 4: Installing Tauri CLI v2 ===" && \
cargo install tauri-cli --locked --force && \
echo "✅ Tauri CLI v2 installed."
```

---

## STEP 5: Create the NeoBot Tauri Project (SvelteKit + TypeScript + Tailwind)

Copy and paste this entire block into your terminal:

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

---

## STEP 6: Add shadcn/ui Components + Cyber Theme (Recommended for NeoBot)

Copy and paste this entire block into your terminal:

```bash
echo "=== Step 6: Adding shadcn/ui + cyber theme ===" && \
pnpm dlx shadcn-svelte@latest init --yes && \
pnpm dlx shadcn-svelte@latest add button card tabs slider progress dialog && \
echo "✅ UI components and theme ready."
```

---

## STEP 7: Set Up Exact Project Structure from the Original Guide

Copy and paste this entire block into your terminal:

```bash
echo "=== Step 7: Creating exact project structure ===" && \
cd ~/neobot && \
mkdir -p kali-preconfig assets installer/src-tauri/src/commands src-tauri/src/vbox && \
echo "✅ Recommended folders created."
```

---

## STEP 8: Initialize Git (Optional but Recommended)

Copy and paste this entire block into your terminal:

```bash
echo "=== Step 8: Initializing Git ===" && \
cd ~/neobot && \
git init --initial-branch=main && \
cat > .gitignore << 'EOF'
target/
node_modules/
build/
dist/
.env
.DS_Store
EOF
git add . && git commit -m "Initial NeoBot setup" && \
echo "✅ Git initialized."
```

---

## STEP 9: Final Verification (Run This to Confirm Everything Works)

Copy and paste this entire block into your terminal:

```bash
echo "=== Step 9: Final Verification ===" && \
cd ~/neobot && \
echo "Rust:" && rustc --version && cargo --version && \
echo "Node & pnpm:" && node --version && pnpm --version && \
echo "Tauri CLI:" && tauri --version && \
echo "Testing Tauri dev build (this will open the app window)..." && \
pnpm run tauri dev -- --no-watch && \
echo "✅ Everything is working! Close the Tauri window when ready."
```

---

## AFTER COMPLETING ALL STEPS

You now have a fully working NeoBot skeleton on your fresh Kali Linux VM.

**Quick test command for future use:**

```bash
cd ~/neobot && pnpm run tauri dev
```

---

## TROUBLESHOOTING TIPS

- If any command fails, check the error message and ensure you have internet access.
- For Rust issues: Run `rustup update` and try again.
- For permission issues: Make sure you're not running as root for user-specific installs.
- VirtualBox: The ext-pack installation may require accepting a license (press Tab/Enter as prompted).
- After Step 3, open a new terminal or run `source ~/.zshrc` if commands are not found.
- Tauri dev build may take time on first run as it compiles dependencies.

---

**Created for NeoBot Tauri Project Setup on Kali Linux**  
**Date: May 2026**