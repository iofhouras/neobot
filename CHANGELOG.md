# NeoBot Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0] - 2026-05-15

### Added
- Professional NSIS installer (`installer/NeoBot-Setup.nsi`) featuring:
  - VirtualBox auto-detection and installation guidance
  - Admin elevation prompt
  - Desktop/start menu shortcuts
  - Clean uninstall support
- Complete Rust Tauri backend for VirtualBox management:
  - `detect_virtualbox`
  - `setup_kali_vm`
  - `start_vm`
  - `get_vm_ip`
  (implemented in `src-tauri/src/commands/vbox.rs`)
- SSH/SCP bridge to running Kali Linux VM (forwarded on port 2222):
  - `start_neobot_agent` command
  - Full file transfer and remote execution capabilities
  (in `src-tauri/src/commands/ssh_bridge.rs`)
- Backend command registration in `src-tauri/src/main.rs`
- `PHASE_0.7_COMPLETION.md` documentation
- This changelog (`CHANGELOG.md`)

### Changed
- Integrated Phase 0.7 deliverables into release branch `feature/v0.7.0-release`
- Promoted to `main` branch with merge commit

### Notes
This release marks the completion of the core Windows installer and VM control infrastructure for NeoBot. The application is now ready for end-to-end testing and GitHub Releases packaging.

[0.7.0]: https://github.com/iofhouras/neobot/compare/main...feature/v0.7.0-release