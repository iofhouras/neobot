# NeoBot Integration Guide

## Phase 3: Core Integration (Complete & Live)

This release integrates the full persistent AI agent stack into the NeoBot Tauri desktop application and Kali VM.

### What's New
- **NeoBot Prompt** (`kali-preconfig/kali-agent/neobot_prompt.txt`): Complete rebranded system prompt with memory JSON format, XML tool blocks, safety guardrails, and self-improvement loop.
- **Python Agent** (`neobot_agent.py`): LangGraph-powered agent with reflection, persistent memory loading/saving, and LLM integration points.
- **Tool Executor** (`tool_executor.py`): Handles browser automation (Playwright), guarded system commands, dynamic skill installation, and GitHub operations.
- **Zero-Touch Provisioning**: Automatically installs Qdrant, Mem0, LangGraph, Playwright, and starts the agent on VM boot.
- **Tauri UI Integration**: New commands for launching agent, memory export/reset, live logs, and GitHub Developer Dashboard.

### Memory Persistence
Every agent response ends with the exact JSON block. Paste it into the next message (or let the Tauri app handle it) for infinite session continuity across WhatsApp/Telegram/Signal.

### GitHub Developer Mode
Connect your account once → full repo read/write, PR creation, code editing, and self-extension capabilities unlocked.

### Security
All high-risk actions require explicit Tauri UI confirmation. Dangerous commands are blocked by default.

## Next Phases
- Phase 4: Full installer bundling + GitHub Actions CI with agent tests
- Multi-user memory isolation

**NeoBot is now the most advanced local persistent GitHub developer platform available.**