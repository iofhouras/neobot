<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { Button, Card, Tabs, Progress } from '$lib/components/ui';

  let vmName = 'NeoBot-Kali';
  let mode = 'full';
  let logs = '';
  let memoryJson = '';
  let isRunning = false;

  async function launchAgent() {
    isRunning = true;
    try {
      const result = await invoke('launch_neobot_agent', { vmName, mode });
      logs = result.logs || 'Agent launched successfully';
      memoryJson = await invoke('export_neobot_memory');
    } catch (e) {
      logs = `Error: ${e}`;
    }
    isRunning = false;
  }

  async function triggerImprovement() {
    const msg = await invoke('trigger_self_improvement');
    logs = msg;
  }
</script>

<div class="cyber-bg p-8">
  <h1 class="text-4xl font-bold text-cyber-accent">NeoBot Control Panel</h1>
  
  <Tabs.Root>
    <Tabs.List>
      <Tabs.Trigger value="control">Agent Control</Tabs.Trigger>
      <Tabs.Trigger value="memory">Memory</Tabs.Trigger>
      <Tabs.Trigger value="github">GitHub Dev</Tabs.Trigger>
    </Tabs.List>
    
    <Tabs.Content value="control">
      <Card>
        <select bind:value={mode}>
          <option value="neobot">NeoBot Mode</option>
          <option value="github-developer">GitHub Developer Mode</option>
          <option value="full">Full Stack</option>
        </select>
        <Button on:click={launchAgent} disabled={isRunning}>
          {isRunning ? 'Launching...' : 'Launch NeoBot Agent'}
        </Button>
        <Button variant="outline" on:click={triggerImprovement}>Trigger Self-Improvement</Button>
      </Card>
      <pre class="logs">{logs}</pre>
    </Tabs.Content>
    
    <Tabs.Content value="memory">
      <pre>{memoryJson}</pre>
      <Button on:click={() => navigator.clipboard.writeText(memoryJson)}>Copy Memory Block</Button>
    </Tabs.Content>
    
    <Tabs.Content value="github">
      <h2>GitHub Developer Dashboard</h2>
      <p>Connected account: iofhouras</p>
      <Button on:click={() => invoke('github_connect')}>Connect / Refresh</Button>
      <!-- Add repo list, PR creator, etc. -->
    </Tabs.Content>
  </Tabs.Root>
</div>

<style>
  .cyber-bg { background: #0a0a0a; color: #00ff9d; }
  .logs { background: #111; padding: 1rem; font-family: monospace; }
</style>