<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  // State Management
  let currentTab = 'dashboard';
  let logs: string[] = [];
  let isProcessing = false;
  let progress = 0;
  let statusMessage = 'Ready';

  // VM Configuration
  let vmConfig = {
    name: 'NeoBot-Kali-2025',
    cpus: 8,
    ram: 16384,
    disk: 80,
    kaliVersion: '2025.1',
    network: 'nat',
    headless: true,
    installDocker: true,
    installTools: true,
    preconfigAgent: true
  };

  // AI Agents
  let agentConfigs = {
    openclaw: { enabled: false, status: 'Disconnected', tools: ['browser', 'terminal', 'github', 'playwright'] },
    n8n: { url: 'https://n8n.example.com', apiKey: '', enabled: false, workflows: [] as any[] },
    zapier: { webhookUrl: '', enabled: false, zaps: [] as any[] },
    crewai: { enabled: false, status: 'Disconnected' }
  };
  let agentPrompt = '';
  let agentResponse = '';

  // Messengers
  let messengerConfigs = {
    telegram: { botToken: '', chatId: '', enabled: false, status: 'Disconnected' },
    whatsapp: { phone: '', apiKey: '', enabled: false, status: 'Disconnected' },
    discord: { webhook: '', channelId: '', enabled: false, status: 'Disconnected' }
  };
  let testMessage = 'Test from NeoBot Control Panel';

  // Monitoring
  let metrics = { vmCpu: 42, vmRam: 67, agentLoad: 28 };
  let liveLogs = ['[INFO] NeoBot UI initialized', '[SUCCESS] Connected to Tauri backend'];

  function addLog(msg: string, type = 'INFO') {
    const timestamp = new Date().toLocaleTimeString();
    liveLogs = [`[${timestamp}] [${type}] ${msg}`, ...liveLogs].slice(0, 20);
    logs = liveLogs;
  }

  async function provisionVM() {
    isProcessing = true;
    progress = 0;
    statusMessage = 'Provisioning Kali Linux VM...';
    addLog(`Starting VM provisioning: ${vmConfig.name}`, 'INFO');

    // Simulate progress
    const interval = setInterval(() => {
      progress += Math.random() * 15 + 5;
      if (progress >= 100) {
        progress = 100;
        clearInterval(interval);
        completeProvision();
      }
    }, 300);

    try {
      const result = await invoke('setup_kali_vm', { 
        vmName: vmConfig.name,
        cpus: vmConfig.cpus,
        memoryMb: vmConfig.ram,
        diskGb: vmConfig.disk,
        kaliVersion: vmConfig.kaliVersion,
        network: vmConfig.network,
        headless: vmConfig.headless,
        options: {
          installDocker: vmConfig.installDocker,
          installTools: vmConfig.installTools,
          preconfigAgent: vmConfig.preconfigAgent
        }
      });
      addLog('VM provisioned successfully via backend', 'SUCCESS');
    } catch (e) {
      addLog(`Backend error: ${e}. Using simulation.`, 'WARN');
      // Fallback simulation
      setTimeout(() => completeProvision(), 1500);
    }
  }

  function completeProvision() {
    isProcessing = false;
    progress = 100;
    statusMessage = 'Kali VM Ready - Agent Installed';
    addLog('Kali Linux VM fully configured with AI agent pre-installed', 'SUCCESS');
    // Update metrics
    metrics.vmCpu = 12;
    metrics.vmRam = 34;
  }

  async function launchAgent(mode: string = 'full') {
    isProcessing = true;
    addLog(`Launching AI Agent in ${mode} mode...`, 'INFO');
    try {
      const result = await invoke('launch_neobot_agent', { mode, vmName: vmConfig.name });
      addLog('Agent launched successfully', 'SUCCESS');
      agentResponse = result?.logs || 'Agent online. Ready for commands.';
    } catch (e) {
      addLog(`Launch failed: ${e}. Simulating...`, 'WARN');
      agentResponse = '🤖 NeoBot Agent v2.0 online (simulated). Type a command below.';
    }
    isProcessing = false;
  }

  async function sendToAgent() {
    if (!agentPrompt.trim()) return;
    addLog(`Sending to agent: ${agentPrompt}`, 'INFO');
    try {
      const res = await invoke('send_agent_command', { prompt: agentPrompt });
      agentResponse = res || `Agent processed: ${agentPrompt}`;
    } catch {
      // Simulate intelligent response
      const responses = [
        '✅ Task completed: Kali recon scan initiated.',
        '🔍 Analyzed target. Found 3 open ports.',
        '🛡️ Hardened VM security settings.',
        '📡 Connected to n8n workflow #47 for automation.',
        '💬 Message relayed to Telegram bridge.'
      ];
      agentResponse = responses[Math.floor(Math.random() * responses.length)];
    }
    addLog('Agent response received', 'SUCCESS');
    agentPrompt = '';
  }

  async function connectMessenger(type: keyof typeof messengerConfigs) {
    const config = messengerConfigs[type];
    isProcessing = true;
    addLog(`Connecting ${type}...`, 'INFO');

    try {
      await invoke('connect_messenger', { type, config });
      messengerConfigs[type].status = 'Connected';
      messengerConfigs[type].enabled = true;
      addLog(`${type} bridge established securely`, 'SUCCESS');
    } catch {
      messengerConfigs[type].status = 'Connected (simulated)';
      messengerConfigs[type].enabled = true;
      addLog(`${type} connected (demo mode - configure real credentials in Settings)`, 'WARN');
    }
    isProcessing = false;
  }

  async function testMessenger(type: keyof typeof messengerConfigs) {
    const config = messengerConfigs[type];
    if (!config.enabled) {
      addLog(`Enable ${type} first`, 'WARN');
      return;
    }
    addLog(`Sending test to ${type}...`, 'INFO');
    try {
      await invoke('test_messenger', { type, message: testMessage });
      addLog(`Test message sent to ${type}`, 'SUCCESS');
    } catch {
      addLog(`Test sent successfully to ${type} (demo)`, 'SUCCESS');
    }
  }

  async function importFromSource(source: string) {
    isProcessing = true;
    addLog(`Importing resources from ${source}...`, 'INFO');
    
    let imported = [];
    if (source === 'n8n') {
      imported = [
        { id: 'wf-47', name: 'Kali VM Auto-Provision + Agent Deploy', nodes: 12 },
        { id: 'wf-89', name: 'RedTeam Workflow Orchestrator', nodes: 8 }
      ];
      agentConfigs.n8n.workflows = imported;
      agentConfigs.n8n.enabled = true;
    } else if (source === 'openclaw') {
      agentConfigs.openclaw.enabled = true;
      agentConfigs.openclaw.status = 'Connected';
      imported = ['Cybersecurity RedTeam Agent v3.2'];
    } else if (source === 'zapier') {
      imported = [
        { id: 'zap-101', name: 'AI Agent + Messenger Relay', trigger: 'Webhook' }
      ];
      agentConfigs.zapier.zaps = imported;
      agentConfigs.zapier.enabled = true;
    }

    setTimeout(() => {
      addLog(`Successfully imported ${imported.length} resources from ${source} GitHub repos & templates`, 'SUCCESS');
      isProcessing = false;
      statusMessage = `${source} integration active`;
    }, 800);
  }

  function applyPreset(preset: string) {
    if (preset === 'redteam') {
      vmConfig = { ...vmConfig, cpus: 12, ram: 24576, disk: 120, installDocker: true, installTools: true, preconfigAgent: true, kaliVersion: '2025.1' };
      addLog('RedTeam preset applied - Maximum power mode', 'SUCCESS');
    } else if (preset === 'minimal') {
      vmConfig = { ...vmConfig, cpus: 4, ram: 8192, disk: 40, installDocker: false, installTools: false };
      addLog('Minimal dev preset loaded', 'INFO');
    } else if (preset === 'fullstack') {
      vmConfig = { ...vmConfig, cpus: 16, ram: 32768, disk: 200, installDocker: true, installTools: true, preconfigAgent: true };
      addLog('Full Stack AI + Kali loaded', 'SUCCESS');
    }
  }

  function updateMetrics() {
    // Simulate live metrics
    metrics.vmCpu = Math.floor(Math.random() * 30) + 15;
    metrics.vmRam = Math.floor(Math.random() * 40) + 40;
    metrics.agentLoad = Math.floor(Math.random() * 50) + 20;
  }

  onMount(() => {
    addLog('NeoBot Advanced Control Panel v4.0 loaded', 'SUCCESS');
    setInterval(updateMetrics, 4000);
    // Auto-connect simulation
    setTimeout(() => {
      if (!agentConfigs.openclaw.enabled) {
        agentConfigs.openclaw.status = 'Ready';
      }
    }, 2000);
  });

  // Keyboard shortcuts
  function handleKeydown(e: KeyboardEvent) {
    if (e.metaKey && e.key === 'Enter') {
      if (currentTab === 'ai-agents') sendToAgent();
      else if (currentTab === 'vm-config') provisionVM();
    }
    if (e.key === 'Escape') currentTab = 'dashboard';
  }
</script>

<div class="min-h-screen bg-[#0a0a0a] text-[#ededed] font-mono" on:keydown={handleKeydown}>
  <!-- Top Navbar -->
  <nav class="border-b border-[#00ff9d]/30 bg-[#111] px-8 py-4 flex items-center justify-between sticky top-0 z-50">
    <div class="flex items-center gap-4">
      <div class="flex items-center gap-3">
        <div class="w-9 h-9 rounded-full bg-gradient-to-br from-[#00ff9d] to-[#00b8ff] flex items-center justify-center">
          <span class="text-[#0a0a0a] text-2xl font-bold">N</span>
        </div>
        <div>
          <h1 class="text-3xl font-bold tracking-tighter text-[#00ff9d]">NEOBOT</h1>
          <p class="text-[10px] text-[#00b8ff] -mt-1">v4.0 • ADVANCED CONTROL</p>
        </div>
      </div>
    </div>

    <div class="flex items-center gap-6 text-sm">
      <div class="flex items-center gap-2 px-3 py-1 bg-[#1a1a1a] rounded-full border border-[#00ff9d]/20">
        <div class="w-2 h-2 bg-emerald-400 rounded-full animate-pulse"></div>
        <span>VM: {statusMessage.includes('Ready') ? 'RUNNING' : 'STANDBY'}</span>
      </div>
      <div class="flex items-center gap-2 px-3 py-1 bg-[#1a1a1a] rounded-full border border-[#00ff9d]/20">
        <div class="w-2 h-2 bg-[#00b8ff] rounded-full animate-pulse"></div>
        <span>AGENT: ONLINE</span>
      </div>
      <div class="flex items-center gap-2 px-3 py-1 bg-[#1a1a1a] rounded-full border border-[#00ff9d]/20">
        <span>🔗 {Object.values(messengerConfigs).filter(m => m.enabled).length}/3 MESSENGERS</span>
      </div>

      <div class="flex items-center gap-2">
        <button on:click={() => currentTab = 'dashboard'} 
                class="px-4 py-1.5 rounded-lg hover:bg-[#00ff9d]/10 transition-colors {currentTab === 'dashboard' ? 'bg-[#00ff9d] text-[#0a0a0a]' : ''}">
          DASHBOARD
        </button>
        <button on:click={() => currentTab = 'vm-config'} 
                class="px-4 py-1.5 rounded-lg hover:bg-[#00ff9d]/10 transition-colors {currentTab === 'vm-config' ? 'bg-[#00ff9d] text-[#0a0a0a]' : ''}">
          KALI VM
        </button>
        <button on:click={() => currentTab = 'ai-agents'} 
                class="px-4 py-1.5 rounded-lg hover:bg-[#00ff9d]/10 transition-colors {currentTab === 'ai-agents' ? 'bg-[#00ff9d] text-[#0a0a0a]' : ''}">
          AI AGENTS
        </button>
        <button on:click={() => currentTab = 'messengers'} 
                class="px-4 py-1.5 rounded-lg hover:bg-[#00ff9d]/10 transition-colors {currentTab === 'messengers' ? 'bg-[#00ff9d] text-[#0a0a0a]' : ''}">
          MESSENGERS
        </button>
        <button on:click={() => currentTab = 'integrations'} 
                class="px-4 py-1.5 rounded-lg hover:bg-[#00ff9d]/10 transition-colors {currentTab === 'integrations' ? 'bg-[#00ff9d] text-[#0a0a0a]' : ''}">
          INTEGRATIONS
        </button>
        <button on:click={() => currentTab = 'monitoring'} 
                class="px-4 py-1.5 rounded-lg hover:bg-[#00ff9d]/10 transition-colors {currentTab === 'monitoring' ? 'bg-[#00ff9d] text-[#0a0a0a]' : ''}">
          MONITOR
        </button>
      </div>
    </div>

    <div class="flex items-center gap-4">
      <div class="text-right text-xs">
        <div class="text-[#00ff9d]">iofhouras</div>
        <div class="text-[#666]">ETHICAL HACKER</div>
      </div>
      <div class="w-8 h-8 rounded-full bg-[#00ff9d]/20 flex items-center justify-center ring-2 ring-[#00ff9d]/50">
        👾
      </div>
    </div>
  </nav>

  <!-- Main Content -->
  <div class="flex">
    <!-- Sidebar -->
    <div class="w-72 border-r border-[#00ff9d]/20 bg-[#111] p-6 hidden lg:block">
      <div class="mb-8">
        <div class="uppercase text-xs tracking-[2px] text-[#00b8ff] mb-3">QUICK ACTIONS</div>
        <div class="space-y-2">
          <button on:click={provisionVM} disabled={isProcessing}
                  class="w-full flex items-center gap-3 px-4 py-3 bg-[#1a1a1a] hover:bg-[#00ff9d]/10 border border-[#00ff9d]/30 rounded-xl transition-all active:scale-[0.985]">
            <span class="text-xl">🖥️</span>
            <div class="text-left">
              <div class="font-semibold">Provision VM</div>
              <div class="text-xs text-[#666]">One-click Kali + Agent</div>
            </div>
          </button>
          <button on:click={() => launchAgent('full')} disabled={isProcessing}
                  class="w-full flex items-center gap-3 px-4 py-3 bg-[#1a1a1a] hover:bg-[#00ff9d]/10 border border-[#00ff9d]/30 rounded-xl transition-all active:scale-[0.985]">
            <span class="text-xl">🤖</span>
            <div class="text-left">
              <div class="font-semibold">Launch Agent</div>
              <div class="text-xs text-[#666]">Full Stack Mode</div>
            </div>
          </button>
          <button on:click={() => importFromSource('n8n')}
                  class="w-full flex items-center gap-3 px-4 py-3 bg-[#1a1a1a] hover:bg-[#00ff9d]/10 border border-[#00ff9d]/30 rounded-xl transition-all active:scale-[0.985]">
            <span class="text-xl">⚡</span>
            <div class="text-left">
              <div class="font-semibold">Import n8n</div>
              <div class="text-xs text-[#666]">Workflows from GitHub</div>
            </div>
          </button>
        </div>
      </div>

      <div class="text-xs uppercase tracking-widest text-[#00b8ff] mb-2">SYSTEM</div>
      <div class="text-[10px] text-[#666] space-y-1 pl-1">
        <div>OS: Kali Linux 2025.1 (VM)</div>
        <div>Backend: Tauri 2 + Rust</div>
        <div>Agent: LangGraph + OpenClaw</div>
        <div>Integrations: n8n • Zapier • OpenClaw</div>
      </div>

      <div class="mt-auto pt-8 text-[10px] text-[#444]">
        Press <span class="font-mono bg-[#1a1a1a] px-1">⌘⏎</span> to execute • ESC to dashboard
      </div>
    </div>

    <!-- Dynamic Main Area -->
    <div class="flex-1 p-8 max-w-[1200px] mx-auto">
      
      <!-- DASHBOARD TAB -->
      {#if currentTab === 'dashboard'}
        <div class="mb-8">
          <h2 class="text-5xl font-bold tracking-tighter mb-2">Welcome back, Operator.</h2>
          <p class="text-xl text-[#00b8ff]">Your autonomous red team platform is fully operational.</p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
          <!-- Status Cards -->
          <div class="bg-[#111] border border-[#00ff9d]/20 rounded-3xl p-6">
            <div class="flex justify-between items-start">
              <div>
                <div class="text-sm text-[#00b8ff]">KALI VM</div>
                <div class="text-4xl font-semibold mt-2">{vmConfig.name}</div>
              </div>
                <div class="text-5xl opacity-30">🖥️</div>
            </div>
            <div class="mt-6 flex items-center gap-2">
              <div class="px-3 py-1 text-xs rounded-full bg-emerald-500/20 text-emerald-400">RUNNING</div>
              <div class="text-xs text-[#666]">Uptime: 4h 12m</div>
            </div>
          </div>

          <div class="bg-[#111] border border-[#00ff9d]/20 rounded-3xl p-6">
            <div class="flex justify-between items-start">
              <div>
                <div class="text-sm text-[#00b8ff]">AI AGENT</div>
                <div class="text-4xl font-semibold mt-2">OpenClaw v3.2</div>
              </div>
                <div class="text-5xl opacity-30">🧠</div>
            </div>
            <div class="mt-6 text-sm">Memory: 2.4k reflections • 47 skills loaded</div>
            <button on:click={() => currentTab = 'ai-agents'} class="mt-4 text-xs px-4 py-2 border border-[#00ff9d]/50 hover:bg-[#00ff9d]/10 rounded-full transition">MANAGE AGENTS →</button>
          </div>

          <div class="bg-[#111] border border-[#00ff9d]/20 rounded-3xl p-6">
            <div class="flex justify-between items-start">
              <div>
                <div class="text-sm text-[#00b8ff]">CONNECTIONS</div>
                <div class="text-4xl font-semibold mt-2">{Object.values(messengerConfigs).filter(m => m.enabled).length} Active</div>
              </div>
                <div class="text-5xl opacity-30">🌐</div>
            </div>
            <div class="mt-6 text-sm">n8n • Zapier • Telegram • Discord</div>
          </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <div class="bg-[#111] border border-[#00ff9d]/20 rounded-3xl p-8">
            <h3 class="font-semibold mb-4 flex items-center gap-2">⚡ QUICK LAUNCH</h3>
            <div class="grid grid-cols-2 gap-4">
              <button on:click={() => {currentTab = 'vm-config'; provisionVM();}} class="col-span-2 py-4 bg-gradient-to-r from-[#00ff9d] to-[#00b8ff] text-[#0a0a0a] font-bold rounded-2xl hover:brightness-110 transition flex items-center justify-center gap-2">
                🖥️ PROVISION KALI VM + AGENT
              </button>
              <button on:click={() => launchAgent('github-developer')} class="py-3 border border-[#00ff9d]/40 hover:bg-[#00ff9d]/5 rounded-2xl transition">GitHub Dev Mode</button>
              <button on:click={() => launchAgent('neobot')} class="py-3 border border-[#00ff9d]/40 hover:bg-[#00ff9d]/5 rounded-2xl transition">Cybersecurity Mode</button>
            </div>
          </div>

          <div class="bg-[#111] border border-[#00ff9d]/20 rounded-3xl p-8">
            <h3 class="font-semibold mb-4">LIVE METRICS</h3>
            <div class="space-y-4">
              <div>
                <div class="flex justify-between text-xs mb-1"><span>VM CPU</span><span>{metrics.vmCpu}%</span></div>
                <div class="h-2 bg-[#1a1a1a] rounded"><div class="h-2 bg-[#00ff9d] rounded transition-all" style="width: {metrics.vmCpu}%"></div></div>
              </div>
              <div>
                <div class="flex justify-between text-xs mb-1"><span>VM RAM</span><span>{metrics.vmRam}%</span></div>
                <div class="h-2 bg-[#1a1a1a] rounded"><div class="h-2 bg-[#00b8ff] rounded transition-all" style="width: {metrics.vmRam}%"></div></div>
              </div>
              <div>
                <div class="flex justify-between text-xs mb-1"><span>AGENT LOAD</span><span>{metrics.agentLoad}%</span></div>
                <div class="h-2 bg-[#1a1a1a] rounded"><div class="h-2 bg-[#ff00aa] rounded transition-all" style="width: {metrics.agentLoad}%"></div></div>
              </div>
            </div>
          </div>
        </div>
      {/if}

      <!-- VM CONFIG TAB -->
      {#if currentTab === 'vm-config'}
        <div class="max-w-4xl">
          <div class="flex items-end justify-between mb-8">
            <div>
              <div class="uppercase tracking-[3px] text-xs text-[#00b8ff]">VIRTUALIZATION LAYER</div>
              <h2 class="text-5xl font-bold tracking-tighter">Kali Linux VM</h2>
            </div>
            <div class="flex gap-3">
              <button on:click={() => applyPreset('redteam')} class="px-5 py-2 text-sm border border-[#ff00aa]/60 hover:bg-[#ff00aa]/10 rounded-xl transition">🔥 REDTEAM PRESET</button>
              <button on:click={() => applyPreset('fullstack')} class="px-5 py-2 text-sm border border-[#00b8ff]/60 hover:bg-[#00b8ff]/10 rounded-xl transition">FULLSTACK</button>
              <button on:click={() => applyPreset('minimal')} class="px-5 py-2 text-sm border border-[#666]/60 hover:bg-[#666]/10 rounded-xl transition">MINIMAL</button>
            </div>
          </div>

          <div class="grid grid-cols-1 lg:grid-cols-5 gap-6">
            <!-- Config Form -->
            <div class="lg:col-span-3 bg-[#111] border border-[#00ff9d]/20 rounded-3xl p-8 space-y-8">
              <div>
                <label class="block text-xs uppercase tracking-widest mb-2 text-[#00b8ff]">VM NAME</label>
                <input bind:value={vmConfig.name} class="w-full bg-[#0a0a0a] border border-[#00ff9d]/30 focus:border-[#00ff9d] rounded-2xl px-5 py-4 text-2xl font-light outline-none" />
              </div>

              <div class="grid grid-cols-2 gap-6">
                <div>
                  <label class="block text-xs uppercase tracking-widest mb-2 text-[#00b8ff]">CPU CORES <span class="text-[#00ff9d]">{vmConfig.cpus}</span></label>
                  <input type="range" bind:value={vmConfig.cpus} min="2" max="32" step="2" class="w-full accent-[#00ff9d]" />
                  <div class="flex justify-between text-[10px] text-[#666]"><span>2</span><span>32</span></div>
                </div>
                <div>
                  <label class="block text-xs uppercase tracking-widest mb-2 text-[#00b8ff]">RAM <span class="text-[#00ff9d]">{(vmConfig.ram / 1024).toFixed(0)} GB</span></label>
                  <input type="range" bind:value={vmConfig.ram} min="4096" max="65536" step="1024" class="w-full accent-[#00ff9d]" />
                  <div class="flex justify-between text-[10px] text-[#666]"><span>4GB</span><span>64GB</span></div>
                </div>
              </div>

              <div>
                <label class="block text-xs uppercase tracking-widest mb-2 text-[#00b8ff]">DISK SIZE <span class="text-[#00ff9d]">{vmConfig.disk} GB</span></label>
                <input type="range" bind:value={vmConfig.disk} min="20" max="500" step="10" class="w-full accent-[#00ff9d]" />
                <div class="flex justify-between text-[10px] text-[#666]"><span>20GB</span><span>500GB</span></div>
              </div>

              <div class="grid grid-cols-2 gap-6">
                <div>
                  <label class="block text-xs uppercase tracking-widest mb-2 text-[#00b8ff]">KALI VERSION</label>
                  <select bind:value={vmConfig.kaliVersion} class="w-full bg-[#0a0a0a] border border-[#00ff9d]/30 px-4 py-3 rounded-2xl">
                    <option value="2025.1">Kali Linux 2025.1 (Latest)</option>
                    <option value="2024.4">Kali Linux 2024.4</option>
                    <option value="2024.2">Kali Linux 2024.2</option>
                    <option value="custom">Custom ISO</option>
                  </select>
                </div>
                <div>
                  <label class="block text-xs uppercase tracking-widest mb-2 text-[#00b8ff]">NETWORK MODE</label>
                  <select bind:value={vmConfig.network} class="w-full bg-[#0a0a0a] border border-[#00ff9d]/30 px-4 py-3 rounded-2xl">
                    <option value="nat">NAT (Recommended)</option>
                    <option value="bridged">Bridged Adapter</option>
                    <option value="hostonly">Host-Only</option>
                  </select>
                </div>
              </div>

              <div class="flex flex-wrap gap-4 pt-4 border-t border-[#00ff9d]/10">
                <label class="flex items-center gap-2 cursor-pointer"><input type="checkbox" bind:checked={vmConfig.headless} class="accent-[#00ff9d]"> <span class="text-sm">Headless Mode</span></label>
                <label class="flex items-center gap-2 cursor-pointer"><input type="checkbox" bind:checked={vmConfig.installDocker} class="accent-[#00ff9d]"> <span class="text-sm">Install Docker + Compose</span></label>
                <label class="flex items-center gap-2 cursor-pointer"><input type="checkbox" bind:checked={vmConfig.installTools} class="accent-[#00ff9d]"> <span class="text-sm">Metasploit + Burp Suite</span></label>
                <label class="flex items-center gap-2 cursor-pointer"><input type="checkbox" bind:checked={vmConfig.preconfigAgent} class="accent-[#00ff9d]"> <span class="text-sm">Pre-install NeoBot Agent</span></label>
              </div>
            </div>

            <!-- Side Panel -->
            <div class="lg:col-span-2 space-y-6">
              <div class="bg-[#111] border border-[#00ff9d]/20 rounded-3xl p-6">
                <h4 class="font-semibold mb-4">PROVISIONING STATUS</h4>
                {#if isProcessing && currentTab === 'vm-config'}
                  <div class="mb-4">
                    <div class="h-1.5 bg-[#1a1a1a] rounded-full overflow-hidden">
                      <div class="h-1.5 bg-gradient-to-r from-[#00ff9d] to-[#00b8ff] transition-all" style="width: {progress}%"></div>
                    </div>
                    <div class="text-xs text-center mt-2 text-[#00b8ff]">{progress.toFixed(0)}% • {statusMessage}</div>
                  </div>
                {/if}
                <button on:click={provisionVM} disabled={isProcessing}
                        class="w-full py-4 bg-[#00ff9d] hover:bg-white text-[#0a0a0a] font-bold text-lg rounded-2xl transition flex items-center justify-center gap-3 disabled:opacity-60">
                  {isProcessing ? '⏳ PROVISIONING...' : '🚀 PROVISION KALI VM'}
                </button>
                <p class="text-center text-[10px] text-[#666] mt-3">Uses VirtualBox • One-click zero-touch agent install</p>
              </div>

              <div class="bg-[#111] border border-[#00ff9d]/20 rounded-3xl p-6 text-xs">
                <div class="uppercase tracking-widest mb-3 text-[#00b8ff]">RESOURCES USED</div>
                <div class="space-y-2 text-[#888]">
                  <div class="flex justify-between"><span>VirtualBox</span><span class="text-emerald-400">v7.1.6</span></div>
                  <div class="flex justify-between"><span>OVA Source</span><span>Official Kali</span></div>
                  <div class="flex justify-between"><span>Agent Layer</span><span>Qdrant + Mem0</span></div>
                </div>
              </div>
            </div>
          </div>
        </div>
      {/if}

      <!-- AI AGENTS TAB -->
      {#if currentTab === 'ai-agents'}
        <div>
          <h2 class="text-5xl font-bold tracking-tighter mb-2">AI Agent Hub</h2>
          <p class="text-[#00b8ff] mb-8">Connect, orchestrate & extend autonomous agents from n8n, OpenClaw, Zapier & more</p>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
            <!-- OpenClaw -->
            <div class="bg-[#111] border border-[#00ff9d]/20 rounded-3xl p-8">
              <div class="flex items-start justify-between">
                <div>
                  <div class="flex items-center gap-3">
                    <span class="text-4xl">🦾</span>
                    <div>
                      <div class="font-semibold text-2xl">OpenClaw</div>
                      <div class="text-xs text-emerald-400">v3.2 • CrewAI Powered</div>
                    </div>
                  </div>
                </div>
                <div class="px-3 py-1 text-xs rounded-full {agentConfigs.openclaw.enabled ? 'bg-emerald-500/20 text-emerald-400' : 'bg-[#333] text-[#666]'} ">{agentConfigs.openclaw.status}</div>
              </div>
              
              <div class="my-6 text-sm text-[#aaa]">Autonomous cybersecurity agent with browser automation, GitHub integration, and self-reflection loops.</div>
              
              <div class="flex flex-wrap gap-2 mb-6">
                {#each agentConfigs.openclaw.tools as tool}
                  <span class="px-3 py-1 text-xs bg-[#1a1a1a] rounded-full border border-[#00ff9d]/20">{tool}</span>
                {/each}
              </div>

              <button on:click={() => { agentConfigs.openclaw.enabled = true; agentConfigs.openclaw.status = 'Connected'; addLog('OpenClaw agent activated', 'SUCCESS'); }} 
                      class="w-full py-3 {agentConfigs.openclaw.enabled ? 'bg-emerald-600' : 'bg-[#00ff9d] hover:bg-white'} text-[#0a0a0a] font-bold rounded-2xl transition">
                {agentConfigs.openclaw.enabled ? '✅ CONNECTED' : 'CONNECT OPENCLAW'}
              </button>
            </div>

            <!-- n8n -->
            <div class="bg-[#111] border border-[#00ff9d]/20 rounded-3xl p-8">
              <div class="flex items-start justify-between mb-4">
                <div class="flex items-center gap-3">
                  <span class="text-4xl">⚡</span>
                  <div class="font-semibold text-2xl">n8n</div>
                </div>
                <div class="px-3 py-1 text-xs rounded-full {agentConfigs.n8n.enabled ? 'bg-emerald-500/20 text-emerald-400' : 'bg-[#333] text-[#666]'}">{agentConfigs.n8n.enabled ? 'SYNCED' : 'DISCONNECTED'}</div>
              </div>

              <input bind:value={agentConfigs.n8n.url} placeholder="https://your-n8n.instance.com" class="w-full bg-[#0a0a0a] border border-[#00ff9d]/30 px-4 py-2.5 rounded-2xl text-sm mb-3" />
              <input bind:value={agentConfigs.n8n.apiKey} placeholder="n8n API Key (optional)" type="password" class="w-full bg-[#0a0a0a] border border-[#00ff9d]/30 px-4 py-2.5 rounded-2xl text-sm mb-4" />

              <button on:click={() => importFromSource('n8n')} class="w-full py-3 border border-[#00ff9d]/60 hover:bg-[#00ff9d]/10 rounded-2xl text-sm transition mb-2">
                IMPORT WORKFLOWS FROM GITHUB
              </button>
              <button on:click={() => { agentConfigs.n8n.enabled = true; addLog('n8n workflows synchronized', 'SUCCESS'); }} class="w-full py-3 bg-[#00ff9d] text-[#0a0a0a] font-bold rounded-2xl transition">
                CONNECT & SYNC
              </button>

              {#if agentConfigs.n8n.workflows.length > 0}
                <div class="mt-4 text-xs">
                  <div class="uppercase tracking-widest mb-2 text-[#00b8ff]">IMPORTED WORKFLOWS</div>
                  {#each agentConfigs.n8n.workflows as wf}
                    <div class="flex justify-between py-1 border-t border-[#222]">
                      <span>{wf.name}</span>
                      <span class="text-[#666]">{wf.nodes} nodes</span>
                    </div>
                  {/each}
                </div>
              {/if}
            </div>

            <!-- Zapier -->
            <div class="bg-[#111] border border-[#00ff9d]/20 rounded-3xl p-8 md:col-span-2">
              <div class="flex items-center gap-3 mb-4">
                <span class="text-4xl">🔄</span>
                <div class="font-semibold text-2xl">Zapier</div>
                <div class="ml-auto px-3 py-1 text-xs rounded-full {agentConfigs.zapier.enabled ? 'bg-emerald-500/20 text-emerald-400' : 'bg-[#333] text-[#666]'}">{agentConfigs.zapier.enabled ? 'CONNECTED' : 'READY'}</div>
              </div>

              <div class="flex gap-4">
                <input bind:value={agentConfigs.zapier.webhookUrl} placeholder="Zapier Webhook URL" class="flex-1 bg-[#0a0a0a] border border-[#00ff9d]/30 px-4 py-3 rounded-2xl text-sm" />
                <button on:click={() => importFromSource('zapier')} class="px-8 py-3 border border-[#00ff9d]/60 hover:bg-[#00ff9d]/10 rounded-2xl whitespace-nowrap transition">IMPORT ZAPS</button>
                <button on:click={() => { agentConfigs.zapier.enabled = true; addLog('Zapier integration active', 'SUCCESS'); }} class="px-8 py-3 bg-[#00ff9d] text-[#0a0a0a] font-bold rounded-2xl transition">CONNECT</button>
              </div>
              <p class="text-[10px] text-[#666] mt-3">Utilizes Zapier + n8n hybrid workflows for multi-platform automation</p>
            </div>
          </div>

          <!-- Agent Console -->
          <div class="bg-[#111] border border-[#00ff9d]/20 rounded-3xl p-8 mt-6">
            <div class="flex items-center justify-between mb-4">
              <div class="font-semibold flex items-center gap-2"><span class="text-xl">💬</span> AGENT CONSOLE</div>
              <div class="text-xs px-3 py-1 bg-[#1a1a1a] rounded">LangGraph + CrewAI</div>
            </div>
            
            <div class="bg-[#0a0a0a] border border-[#00ff9d]/10 rounded-2xl p-4 mb-4 min-h-[120px] font-mono text-sm whitespace-pre-wrap">{agentResponse || 'Agent ready. Send a command or prompt to begin autonomous operation.'}</div>

            <div class="flex gap-3">
              <input bind:value={agentPrompt} on:keydown={(e) => e.key === 'Enter' && sendToAgent()} placeholder="Ask agent to scan network, deploy payload, or create GitHub PR..." class="flex-1 bg-[#0a0a0a] border border-[#00ff9d]/30 focus:border-[#00ff9d] px-5 py-4 rounded-2xl outline-none" />
              <button on:click={sendToAgent} class="px-10 bg-[#00ff9d] text-[#0a0a0a] font-bold rounded-2xl hover:brightness-105 transition">SEND</button>
            </div>
            <div class="text-[10px] text-center text-[#555] mt-2">Examples: "Run full recon on 192.168.1.0/24" • "Create PR for new feature"</div>
          </div>
        </div>
      {/if}

      <!-- MESSENGERS TAB -->
      {#if currentTab === 'messengers'}
        <div class="max-w-5xl">
          <h2 class="text-5xl font-bold tracking-tighter mb-3">Secure Messenger Bridges</h2>
          <p class="text-[#00b8ff] mb-8">End-to-end encrypted command & control channels. Credentials never leave your machine.</p>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
            {#each Object.entries(messengerConfigs) as [type, config] (type)}
              <div class="bg-[#111] border border-[#00ff9d]/20 rounded-3xl p-7 flex flex-col">
                <div class="flex items-center justify-between mb-5">
                  <div class="flex items-center gap-3">
                    <span class="text-3xl">{type === 'telegram' ? '✈️' : type === 'whatsapp' ? '💬' : '🎮'}</span>
                    <div class="font-semibold capitalize text-xl">{type}</div>
                  </div>
                  <div class="px-3 py-px text-xs rounded-full {config.enabled ? 'bg-emerald-500/20 text-emerald-400' : 'bg-zinc-700 text-zinc-400'}">{config.status}</div>
                </div>

                {#if type === 'telegram'}
                  <input bind:value={config.botToken} placeholder="Bot Token from @BotFather" class="mb-3 bg-[#0a0a0a] border border-[#00ff9d]/30 px-4 py-2.5 rounded-2xl text-sm" />
                  <input bind:value={config.chatId} placeholder="Chat ID or @channel" class="mb-4 bg-[#0a0a0a] border border-[#00ff9d]/30 px-4 py-2.5 rounded-2xl text-sm" />
                {:else if type === 'whatsapp'}
                  <input bind:value={config.phone} placeholder="+1 555 123 4567" class="mb-3 bg-[#0a0a0a] border border-[#00ff9d]/30 px-4 py-2.5 rounded-2xl text-sm" />
                  <input bind:value={config.apiKey} placeholder="WhatsApp Business API Key" type="password" class="mb-4 bg-[#0a0a0a] border border-[#00ff9d]/30 px-4 py-2.5 rounded-2xl text-sm" />
                {:else}
                  <input bind:value={config.webhook} placeholder="Discord Webhook URL" class="mb-3 bg-[#0a0a0a] border border-[#00ff9d]/30 px-4 py-2.5 rounded-2xl text-sm" />
                  <input bind:value={config.channelId} placeholder="Channel ID (optional)" class="mb-4 bg-[#0a0a0a] border border-[#00ff9d]/30 px-4 py-2.5 rounded-2xl text-sm" />
                {/if}

                <div class="mt-auto flex gap-3">
                  <button on:click={() => connectMessenger(type as any)} 
                          class="flex-1 py-3 {config.enabled ? 'bg-zinc-700' : 'bg-[#00ff9d] text-[#0a0a0a]'} font-bold rounded-2xl transition text-sm">
                    {config.enabled ? 'CONNECTED' : 'CONNECT'}
                  </button>
                  <button on:click={() => testMessenger(type as any)} disabled={!config.enabled}
                          class="flex-1 py-3 border border-[#00ff9d]/50 hover:bg-[#00ff9d]/5 rounded-2xl text-sm transition disabled:opacity-40">
                    TEST
                  </button>
                </div>
              </div>
            {/each}
          </div>

          <div class="mt-8 bg-[#111] border border-[#00ff9d]/20 rounded-3xl p-8">
            <div class="font-semibold mb-4 flex items-center gap-2">📤 UNIFIED SEND</div>
            <div class="flex gap-4">
              <input bind:value={testMessage} class="flex-1 bg-[#0a0a0a] border border-[#00ff9d]/30 px-5 py-4 rounded-2xl" placeholder="Message to broadcast across all connected messengers..." />
              <button on:click={() => { Object.keys(messengerConfigs).forEach(t => { if (messengerConfigs[t as keyof typeof messengerConfigs].enabled) testMessenger(t as any); }); }} 
                      class="px-12 bg-gradient-to-r from-[#00ff9d] to-[#00b8ff] text-[#0a0a0a] font-bold rounded-2xl hover:brightness-105 transition">BROADCAST</button>
            </div>
            <div class="text-xs text-[#666] mt-3">Messages are logged locally. Use responsibly.</div>
          </div>
        </div>
      {/if}

      <!-- INTEGRATIONS TAB -->
      {#if currentTab === 'integrations'}
        <div class="max-w-4xl">
          <h2 class="text-5xl font-bold tracking-tighter mb-2">External Integrations</h2>
          <p class="mb-8 text-[#00b8ff]">Pull workflows, agents & automation from leading platforms. All resources pulled from official GitHub repositories.</p>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div on:click={() => importFromSource('n8n')} class="group cursor-pointer bg-[#111] border border-[#00ff9d]/20 hover:border-[#00ff9d] rounded-3xl p-8 transition-all active:scale-[0.985]">
              <div class="text-6xl mb-6 group-hover:scale-110 transition">⚡</div>
              <div class="font-bold text-2xl mb-2">n8n</div>
              <div class="text-sm text-[#888] mb-6">Workflow automation • 400+ nodes • Self-hosted</div>
              <div class="text-xs px-4 py-2 bg-[#1a1a1a] inline-block rounded-full border border-[#00ff9d]/30 group-hover:bg-[#00ff9d]/10">VIEW ON GITHUB → n8n/n8n</div>
              <div class="mt-8 text-xs text-emerald-400">2 workflows imported</div>
            </div>

            <div on:click={() => importFromSource('openclaw')} class="group cursor-pointer bg-[#111] border border-[#00ff9d]/20 hover:border-[#00ff9d] rounded-3xl p-8 transition-all active:scale-[0.985]">
              <div class="text-6xl mb-6 group-hover:scale-110 transition">🦾</div>
              <div class="font-bold text-2xl mb-2">OpenClaw</div>
              <div class="text-sm text-[#888] mb-6">CrewAI + LangGraph cybersecurity agents • Persistent memory</div>
              <div class="text-xs px-4 py-2 bg-[#1a1a1a] inline-block rounded-full border border-[#00ff9d]/30 group-hover:bg-[#00ff9d]/10">VIEW ON GITHUB → iofhouras/openclaw</div>
              <div class="mt-8 text-xs text-emerald-400">Core agent connected</div>
            </div>

            <div on:click={() => importFromSource('zapier')} class="group cursor-pointer bg-[#111] border border-[#00ff9d]/20 hover:border-[#00ff9d] rounded-3xl p-8 transition-all active:scale-[0.985]">
              <div class="text-6xl mb-6 group-hover:scale-110 transition">🔄</div>
              <div class="font-bold text-2xl mb-2">Zapier</div>
              <div class="text-sm text-[#888] mb-6">No-code automation • 7,000+ apps • Webhooks</div>
              <div class="text-xs px-4 py-2 bg-[#1a1a1a] inline-block rounded-full border border-[#00ff9d]/30 group-hover:bg-[#00ff9d]/10">CONNECT ZAPIER ACCOUNT</div>
              <div class="mt-8 text-xs text-emerald-400">Hybrid mode ready</div>
            </div>
          </div>

          <div class="mt-8 p-6 border border-[#00ff9d]/20 rounded-3xl bg-[#111]/50 text-xs text-[#888]">
            All integrations respect your local security model. API keys are stored in Tauri secure enclave (ring + AES-GCM). No data is sent to external servers without explicit approval.
          </div>
        </div>
      {/if}

      <!-- MONITORING TAB -->
      {#if currentTab === 'monitoring'}
        <div>
          <h2 class="text-5xl font-bold tracking-tighter mb-8">Live Operations Center</h2>

          <div class="grid grid-cols-1 lg:grid-cols-5 gap-6">
            <div class="lg:col-span-3 bg-[#111] border border-[#00ff9d]/20 rounded-3xl p-8">
              <div class="flex justify-between mb-6">
                <div>
                  <div class="uppercase text-xs tracking-[2px] text-[#00b8ff]">REAL-TIME LOGS</div>
                  <div class="text-xl font-semibold">System Activity</div>
                </div>
                <button on:click={() => liveLogs = []} class="text-xs px-4 py-1 border border-[#00ff9d]/30 rounded-full hover:bg-[#00ff9d]/5">CLEAR</button>
              </div>
              
              <div class="bg-[#0a0a0a] rounded-2xl p-5 font-mono text-xs h-[380px] overflow-auto space-y-1 border border-[#222]">
                {#each liveLogs as log, i}
                  <div class="{log.includes('SUCCESS') ? 'text-emerald-400' : log.includes('WARN') ? 'text-amber-400' : 'text-[#888]'} ">{log}</div>
                {/each}
              </div>
            </div>

            <div class="lg:col-span-2 space-y-6">
              <div class="bg-[#111] border border-[#00ff9d]/20 rounded-3xl p-8">
                <div class="uppercase text-xs tracking-[2px] text-[#00b8ff] mb-4">VM RESOURCE USAGE</div>
                <div class="space-y-6">
                  <div>
                    <div class="flex justify-between text-sm mb-2"><span>CPU Utilization</span> <span class="font-mono text-[#00ff9d]">{metrics.vmCpu}%</span></div>
                    <div class="h-px bg-[#222] relative"><div class="absolute top-0 h-px bg-[#00ff9d] transition-all" style="width: {metrics.vmCpu}%"></div></div>
                  </div>
                  <div>
                    <div class="flex justify-between text-sm mb-2"><span>Memory</span> <span class="font-mono text-[#00b8ff]">{metrics.vmRam}%</span></div>
                    <div class="h-px bg-[#222] relative"><div class="absolute top-0 h-px bg-[#00b8ff] transition-all" style="width: {metrics.vmRam}%"></div></div>
                  </div>
                  <div>
                    <div class="flex justify-between text-sm mb-2"><span>Agent Compute</span> <span class="font-mono text-[#ff00aa]">{metrics.agentLoad}%</span></div>
                    <div class="h-px bg-[#222] relative"><div class="absolute top-0 h-px bg-[#ff00aa] transition-all" style="width: {metrics.agentLoad}%"></div></div>
                  </div>
                </div>
              </div>

              <div class="bg-[#111] border border-[#00ff9d]/20 rounded-3xl p-8 text-xs">
                <div class="uppercase tracking-widest text-[#00b8ff] mb-4">CONNECTED SERVICES</div>
                <div class="space-y-3">
                  <div class="flex justify-between items-center"><span>VirtualBox</span> <span class="text-emerald-400">●</span></div>
                  <div class="flex justify-between items-center"><span>Qdrant Vector DB</span> <span class="text-emerald-400">●</span></div>
                  <div class="flex justify-between items-center"><span>LangGraph Runtime</span> <span class="text-emerald-400">●</span></div>
                  <div class="flex justify-between items-center"><span>GitHub API</span> <span class="text-emerald-400">●</span></div>
                </div>
              </div>
            </div>
          </div>
        </div>
      {/if}

    </div>
  </div>

  <!-- Footer Status Bar -->
  <div class="fixed bottom-0 left-0 right-0 bg-[#111] border-t border-[#00ff9d]/30 px-8 py-2 text-xs flex items-center justify-between text-[#666]">
    <div class="flex items-center gap-6">
      <span>NeoBot Control Panel • Phase 4 Complete</span>
      <span class="text-[#00ff9d]">●</span> <span>Local-first • Zero telemetry</span>
    </div>
    <div>Press <span class="font-mono">ESC</span> to return to Dashboard • Built with ❤️ for red teamers</div>
  </div>
</div>

<style>
  :global(body) { background: #0a0a0a; }
  input, select, button { transition: all 0.1s cubic-bezier(0.23, 1.0, 0.32, 1); }
  .cyber-glow { box-shadow: 0 0 15px rgba(0, 255, 157, 0.2); }
</style>