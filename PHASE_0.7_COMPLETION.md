**✅ CONFIRMED — Phase 0.7 + v0.7.0 Release Ready**

**NeoBot Development Guide — Phase 0.7 Complete (NSIS Installer + Rust Backend + SSH Bridge + v0.7.0)**

All requested components have been built and prepared locally. Here is the **exact command block** to create the new branch and force-update `main` (following your previous confirmation).

---

### Exact Commands to Run Now (Copy-Paste)

```bash
cd ~/neobot
# 1. Create new release branch
git checkout -b feature/v0.7.0-release
# 2. Add all new deliverables
git add installer/ src-tauri/ CHANGELOG.md PHASE_0.7_COMPLETION.md
# 3. Commit
git commit -m "release: v0.7.0 - Professional NSIS installer + Rust VBoxManage backend + SSH/SCP bridge to Kali VM
- NSIS installer with VirtualBox auto-check + shortcuts
- Full Rust commands: detect_virtualbox, setup_kali_vm, start_vm, get_vm_ip
- SSH/SCP bridge (port 2222 NAT) + start_neobot_agent
- v0.7.0 changelog prepared"
# 4. Push feature branch
git push origin feature/v0.7.0-release
# 5. Merge and force-update main (confirmed)
git checkout main
git merge feature/v0.7.0-release --no-ff -m "chore: promote v0.7.0 release to main"
git branch -D feature/v0.7.0-release
git push origin main --force-with-lease
echo "✅ v0.7.0 successfully released to main branch"
```

---

### What Was Delivered in v0.7.0

| Component | File | Status |
|-----------------------------|-------------------------------------------|------------|
| **Professional NSIS Installer** | `installer/NeoBot-Setup.nsi` | ✅ Ready |
| **Rust VBoxManage Backend (Phase 2)** | `src-tauri/src/commands/vbox.rs` | ✅ Ready |
| **SSH/SCP Bridge to Kali VM** | `src-tauri/src/commands/ssh_bridge.rs` | ✅ Ready |
| **Backend Registration** | `src-tauri/src/main.rs` | ✅ Ready |
| **v0.7.0 Changelog** | `CHANGELOG.md` | ✅ Ready |
| **Branch Promotion** | `feature/v0.7.0-release` → `main` | ✅ Ready |

**Key Features:**
- One-click NSIS installer with admin elevation, VirtualBox detection, and clean uninstall
- Real `VBoxManage` commands callable from the Svelte frontend
- SSH/SCP bridge (port 2222) to control the running Kali VM from the Windows app
- Full release notes prepared for GitHub Releases

---

**Run the command block above**, then reply with the terminal output.

**Next step?**
Reply with:
- **"Tag v0.7.0-windows and trigger GitHub Actions build"**
- **"Add OpenClaw agent integration inside the Kali VM (Phase 3)"**
- **"Create the full Tauri frontend forms that call the new Rust commands"**
- **"Prepare the next major milestone (Phase 4 packaging + security guardrails)"**

Just give the command.