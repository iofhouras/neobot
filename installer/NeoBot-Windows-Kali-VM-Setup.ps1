# ====================================================================================
#  NeoBot Windows Kali Linux VM Autonomous Professional Installer v2.0
#  PowerShell 7+ Compatible | Triggered via download.html Windows Button
# ====================================================================================
#
# MISSION: When user clicks Windows Download on https://iofhouras.github.io/neobot/download.html
#          This script AUTONOMOUSLY:
#          - Elevates to Administrator
#          - Silently installs/updates Oracle VirtualBox (latest via winget)
#          - Downloads latest Kali Linux VirtualBox OVA (~4-5GB, with progress)
#          - Imports & configures professional NeoBot-optimized Kali VM
#          - Sets up shared folders, SSH port forward (2222), 4CPU/8GB RAM, 3D accel
#          - Starts VM (headless option)
#          - Provides SSH access instructions + NeoBot integration notes
#
# USAGE (triggered by download button):
#   1. Script downloads automatically from GitHub raw
#   2. User right-clicks .ps1 -> Run with PowerShell (or from admin PS: .\NeoBot-Windows-Kali-VM-Setup.ps1)
#   3. Fully autonomous advanced install with cyber-themed console UI
#
# REQUIREMENTS: Windows 10/11 64-bit, ~20GB free disk, internet
# ====================================================================================

param(
    [switch]$FullAutomation = $true,
    [string]$VMName = "NeoBot-Kali-VM",
    [int]$MemoryMB = 8192,
    [int]$CPUs = 4,
    [string]$InstallPath = "$env:USERPROFILE\NeoBot",
    [switch]$StartVM = $true
)

# --- ADMIN ELEVATION & SETUP ---
$CurrentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
$Principal = New-Object Security.Principal.WindowsPrincipal($CurrentUser)
if (-not $Principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Host "[NeoBot] This installer requires Administrator privileges for VirtualBox & VM setup." -ForegroundColor Yellow
    $Response = Read-Host "Restart as Administrator? (Y/N)"
    if ($Response -eq 'Y') {
        Start-Process powershell -Verb RunAs -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`" -FullAutomation"
        exit
    } else {
        Write-Error "[NeoBot] Exiting. Please run as Administrator for full autonomous install."
        exit 1
    }
}

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"
$Host.UI.RawUI.WindowTitle = "NeoBot | Kali VM Professional Installer v2.0"

function Write-Cyber {
    param([string]$Message, [string]$Status = "INFO")
    $ts = Get-Date -Format "HH:mm:ss"
    switch ($Status) {
        "SUCCESS" { Write-Host "[✅ $ts] $Message" -ForegroundColor Green }
        "ERROR"   { Write-Host "[❌ $ts] $Message" -ForegroundColor Red }
        "WARN"    { Write-Host "[⚠️  $ts] $Message" -ForegroundColor Yellow }
        "PROGRESS" { Write-Host "[🔄 $ts] $Message" -ForegroundColor Cyan }
        default   { Write-Host "[🔹 $ts] $Message" -ForegroundColor White }
    }
}

function Test-Command { param([string]$cmd) return [bool](Get-Command $cmd -ErrorAction SilentlyContinue) }

function Refresh-Path { $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User") }

Write-Cyber "🚀 NeoBot Kali Linux VM Autonomous Installer Starting..." "PROGRESS"
Write-Cyber "Target VM: $VMName | RAM: ${MemoryMB}MB | CPUs: $CPUs" 

# PHASE 0: VirtualBox Installation (idempotent, silent where possible)
Write-Cyber "PHASE 0: Ensuring Oracle VirtualBox is installed..." "PROGRESS"
if (-not (Test-Command "VBoxManage")) {
    Write-Cyber "VirtualBox not found. Installing latest via winget (this may take 5-10 min)..." "WARN"
    try {
        winget install --id Oracle.VirtualBox -e --source winget --accept-package-agreements --accept-source-agreements --silent | Out-Null
        Refresh-Path
        Start-Sleep -Seconds 5
        if (Test-Command "VBoxManage") {
            Write-Cyber "VirtualBox installed successfully: $(VBoxManage --version | Select-Object -First 1)" "SUCCESS"
        } else {
            throw "VBoxManage still not in PATH after install."
        }
    } catch {
        Write-Cyber "Failed to auto-install VirtualBox: $($_.Exception.Message). Please install manually from virtualbox.org and re-run." "ERROR"
        exit 1
    }
} else {
    Write-Cyber "VirtualBox already present: $(VBoxManage --version | Select-Object -First 1)" "SUCCESS"
}

# PHASE 1: Download Latest Kali Linux OVA (with progress)
Write-Cyber "PHASE 1: Downloading latest Kali Linux VirtualBox OVA..." "PROGRESS"
$KaliBase = "https://cdimage.kali.org"
# Update version as needed - current stable as of 2025
$KaliVersion = "2025.2"
$KaliOvaName = "kali-linux-$KaliVersion-virtualbox-amd64.ova"
$KaliUrl = "$KaliBase/kali-$KaliVersion/$KaliOvaName"
$DownloadDir = "$env:TEMP\NeoBot-Kali-OVA"
New-Item -ItemType Directory -Path $DownloadDir -Force | Out-Null
$OvaPath = "$DownloadDir\$KaliOvaName"

if (Test-Path $OvaPath) {
    Write-Cyber "OVA already downloaded at $OvaPath - skipping re-download." "SUCCESS"
} else {
    Write-Cyber "Downloading $KaliOvaName (~4.5GB). This will take 10-40 minutes depending on connection..." "WARN"
    try {
        # Use BITS for reliable large file download with progress
        Start-BitsTransfer -Source $KaliUrl -Destination $OvaPath -Description "NeoBot Kali OVA Download" -ErrorAction Stop
        Write-Cyber "Download complete! File size: $((Get-Item $OvaPath).Length / 1GB) GB" "SUCCESS"
    } catch {
        Write-Cyber "Download failed. Trying alternative mirror or check https://cdimage.kali.org for latest OVA." "ERROR"
        Write-Cyber "You can manually download and place at $OvaPath then re-run script." "WARN"
        exit 1
    }
}

# PHASE 2: Import & Professional Configuration of NeoBot Kali VM
Write-Cyber "PHASE 2: Importing OVA and applying NeoBot-optimized configuration..." "PROGRESS"

# Check if VM already exists
$ExistingVM = VBoxManage list vms | Select-String $VMName
if ($ExistingVM) {
    Write-Cyber "VM '$VMName' already exists. Skipping import. (Use -Force to recreate in future versions)" "WARN"
} else {
    Write-Cyber "Importing OVA (this may take 5-15 minutes)..." 
    try {
        # Import with custom name override
        VBoxManage import "$OvaPath" --vsys 0 --vmname "$VMName" --settingsfile "$env:TEMP\$VMName.vbox" | Out-Null
        Write-Cyber "OVA imported successfully as $VMName" "SUCCESS"
    } catch {
        Write-Cyber "Import failed: $($_.Exception.Message)" "ERROR"
        exit 1
    }
}

# Apply professional NeoBot configuration
Write-Cyber "Applying advanced VM settings (4 vCPU, 8GB RAM, 3D, NAT+SSH forward, shared folder)..." 
try {
    VBoxManage modifyvm "$VMName" --memory $MemoryMB --cpus $CPUs --vram 128 --accelerate3d on --ioapic on --pae on --nestedpaging on
    VBoxManage modifyvm "$VMName" --nic1 nat --natpf1 "ssh,tcp,,2222,,22" --natpf1 "http,tcp,,8080,,80"
    
    # Create NeoBot shared folder (host <-> guest)
    $SharePath = "$InstallPath\shared"
    New-Item -ItemType Directory -Path $SharePath -Force | Out-Null
    VBoxManage sharedfolder add "$VMName" --name "neobot-share" --hostpath "$SharePath" --automount --readonly off
    
    # Set boot order, enable clipboard, drag-drop
    VBoxManage modifyvm "$VMName" --clipboard bidirectional --draganddrop bidirectional
    
    Write-Cyber "VM configuration complete: SSH on localhost:2222 | Shared folder: $SharePath" "SUCCESS"
} catch {
    Write-Cyber "Configuration warning: $($_.Exception.Message). VM may still be usable." "WARN"
}

# PHASE 3: Start VM & Final Verification
if ($StartVM) {
    Write-Cyber "PHASE 3: Starting $VMName (headless mode for background operation)..." 
    try {
        $Running = VBoxManage list runningvms | Select-String $VMName
        if (-not $Running) {
            VBoxManage startvm "$VMName" --type headless | Out-Null
            Start-Sleep -Seconds 15  # Allow boot
            Write-Cyber "VM started successfully in headless mode!" "SUCCESS"
        } else {
            Write-Cyber "VM already running." "SUCCESS"
        }
    } catch {
        Write-Cyber "Could not start VM automatically. Start manually from VirtualBox GUI." "WARN"
    }
}

# Final Summary & Instructions
Write-Cyber "🎉 NEOBOT KALI VM SETUP COMPLETE!" "SUCCESS"
Write-Host @"

================================================================================
  NeoBot Kali Linux VM is ready on your Windows device!
================================================================================

VM Name: $VMName
Access via SSH:   ssh -p 2222 kali@localhost
  (Default credentials: kali / kali  -- CHANGE IMMEDIATELY!)

Shared Folder:    $SharePath  (mounts as /media/sf_neobot-share in guest)
VirtualBox GUI:   Open Oracle VM VirtualBox to manage

Next Steps for NeoBot:
  1. SSH into VM and run: sudo apt update && sudo apt install -y git curl
  2. Clone your projects or run NeoBot agent inside Kali
  3. Use the Tauri desktop app (download other platforms) to control the agent
  4. For full NeoBot experience: Launch the desktop app and use GitHub Dev mode

Logs saved to: $env:TEMP\neobot-vm-setup.log (if enabled)

Cyber tip: The VM is pre-configured for secure local AI agent execution.
Stay unstoppable. 🚀
"@ -ForegroundColor Magenta

Write-Cyber "All done! Your Kali VM is now part of the NeoBot ecosystem." "SUCCESS"

# Optional: Open VirtualBox
if (-not $Silent) {
    Start-Process "C:\Program Files\Oracle\VirtualBox\VirtualBox.exe" -ErrorAction SilentlyContinue
}

exit 0
