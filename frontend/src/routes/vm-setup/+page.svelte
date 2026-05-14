<script>
  import { invoke } from '@tauri-apps/api/tauri';
  let vmName = 'NeoBot-Kali';
  let cpus = 4;
  let memory = 8192;
  let headless = true;
  let progress = 0;
  let status = 'Ready to create Kali Linux VM';
  let isLoading = false;

  async function createVM() {
    isLoading = true;
    status = 'Starting setup...';
    progress = 0;

    try {
      const result = await invoke('setup_kali_vm', {
        config: {
          vmName,
          cpus,
          memoryMb: memory,
          headless
        }
      });
      status = result;
      progress = 100;
    } catch (error) {
      status = 'Error: ' + error;
    } finally {
      isLoading = false;
    }
  }

  // Listen for progress events
  import { listen } from '@tauri-apps/api/event';
  listen('download-progress', (event) => {
    const data = event.payload;
    progress = data.percentage;
    status = data.status;
  });
</script>

<div class="min-h-screen bg-[#0a0a0a] text-white p-8">
  <div class="max-w-2xl mx-auto">
    <div class="text-center mb-12">
      <div class="inline-flex items-center gap-3 mb-4">
        <div class="w-12 h-12 bg-gradient-to-br from-[#00ff9d] to-[#00b8ff] rounded-2xl flex items-center justify-center">
          <i class="fas fa-robot text-3xl text-black"></i>
        </div>
        <h1 class="text-5xl font-bold tracking-tighter">NeoBot VM Creator</h1>
      </div>
      <p class="text-xl text-white/60">One-click Kali Linux VM Setup</p>
    </div>

    <div class="glass border border-white/10 rounded-3xl p-10">
      <div class="space-y-8">
        <!-- VM Name -->
        <div>
          <label class="block text-sm font-medium mb-2 text-white/70">VM Name</label>
          <input bind:value={vmName} type="text" class="w-full bg-black/40 border border-white/20 rounded-2xl px-5 py-4 text-lg focus:outline-none focus:border-[#00ff9d]" />
        </div>

        <!-- CPU & RAM -->
        <div class="grid grid-cols-2 gap-6">
          <div>
            <label class="block text-sm font-medium mb-2 text-white/70">CPU Cores</label>
            <input bind:value={cpus} type="range" min="1" max="16" step="1" class="w-full accent-[#00ff9d]" />
            <div class="text-center mt-1 font-mono text-[#00ff9d]">{cpus} cores</div>
          </div>
          <div>
            <label class="block text-sm font-medium mb-2 text-white/70">RAM (MB)</label>
            <input bind:value={memory} type="range" min="2048" max="32768" step="512" class="w-full accent-[#00ff9d]" />
            <div class="text-center mt-1 font-mono text-[#00ff9d]">{(memory/1024).toFixed(1)} GB</div>
          </div>
        </div>

        <!-- Headless Mode -->
        <div class="flex items-center justify-between bg-black/30 p-5 rounded-2xl">
          <div>
            <div class="font-medium">Headless Mode</div>
            <div class="text-sm text-white/50">Run without GUI window</div>
          </div>
          <label class="relative inline-flex items-center cursor-pointer">
            <input type="checkbox" bind:checked={headless} class="sr-only peer" />
            <div class="w-14 h-8 bg-white/20 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-1 after:left-1 after:bg-white after:rounded-full after:h-6 after:w-6 after:transition-all peer-checked:bg-[#00ff9d]"></div>
          </label>
        </div>

        <!-- Progress -->
        {#if progress > 0}
          <div class="mt-6">
            <div class="flex justify-between text-sm mb-2">
              <span>{status}</span>
              <span class="font-mono text-[#00ff9d]">{progress.toFixed(0)}%</span>
            </div>
            <div class="h-2 bg-white/10 rounded-full overflow-hidden">
              <div class="h-full bg-gradient-to-r from-[#00ff9d] to-[#00b8ff] transition-all duration-300" style="width: {progress}%"></div>
            </div>
          </div>
        {/if}

        <!-- Action Button -->
        <button 
          on:click={createVM}
          disabled={isLoading}
          class="w-full py-5 bg-[#00ff9d] hover:bg-[#00e68a] text-black font-bold text-xl rounded-2xl mt-4 flex items-center justify-center gap-3 disabled:opacity-70 transition-all active:scale-[0.985]">
          {#if isLoading}
            <span>Creating VM...</span>
          {:else}
            <span>Create & Launch Kali Linux VM</span>
            <i class="fas fa-rocket"></i>
          {/if}
        </button>
      </div>
    </div>

    <div class="text-center mt-8 text-xs text-white/40">
      Powered by VirtualBox • Tauri • SvelteKit
    </div>
  </div>
</div>

<style>
  .glass {
    background: rgba(255,255,255,0.04);
    backdrop-filter: blur(24px);
  }
</style>