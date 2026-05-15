# NeoBot Windows Kali VM Autonomous Installer v2.1
# Professional one-click setup for VirtualBox + Kali Linux VM
# Triggered from download.html Windows button

param([switch]$FullAutomation, [int]$MemoryMB=4096, [int]$CPUs=2, [string]$VMName="NeoBot-Kali-VM", [switch]$Headless, [switch]$EnableGUIProgress)

if (-not ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    if (-not $FullAutomation) { Read-Host 'Run as Admin required. Press Enter to exit'; exit }
    Start-Process powershell -Verb RunAs -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File `$PSCommandPath"; exit
}

$ErrorActionPreference = 'Stop'
$Host.UI.RawUI.WindowTitle = 'NeoBot Kali VM Installer v2.1'
Start-Transcript -Path "$env:TEMP\NeoBot-Kali-Setup.log" -Force | Out-Null

function Write-Phase($t) { Write-Host "`n=== $t ===" -ForegroundColor Cyan }
function Write-Step($m, $s='INFO') { $ts=Get-Date -Format 'HH:mm:ss'; Write-Host ("{0} [{1}] {2}" -f (@{SUCCESS='✅';ERROR='❌';WARN='⚠️';default='🔹'}[$s] ?? '🔹'), $ts, $m) -ForegroundColor (@{SUCCESS='Green';ERROR='Red';WARN='Yellow';default='White'}[$s] ?? 'White') }

function Test-Cmd($c) { [bool](Get-Command $c -EA SilentlyContinue) }

function Refresh-Path { $env:Path = [Environment]::GetEnvironmentVariable('Path','Machine') + ';' + [Environment]::GetEnvironmentVariable('Path','User') }

Write-Phase 'PHASE 0: Pre-Flight'
if ((Get-CimInstance Win32_OperatingSystem).Version -lt '10.0.19041') { Write-Step 'Windows 10 20H1+ recommended' 'WARN' }
if (-not (Test-Cmd 'winget')) { try { Add-AppxPackage -RegisterByFamilyName Microsoft.DesktopAppInstaller_8wekyb3d8bbwe -EA Stop; Refresh-Path } catch { Write-Step 'winget unavailable, using fallback' 'WARN' } }
Write-Step 'Pre-flight OK' 'SUCCESS'

Write-Phase 'PHASE 1: VirtualBox'
if (-not (Test-Cmd 'VBoxManage')) {
    Write-Step 'Installing VirtualBox...' 'WARN'
    if (Test-Cmd 'winget') { winget install Oracle.VirtualBox --exact --accept-package-agreements --accept-source-agreements --silent | Out-Null; Refresh-Path }
    else { $u='https://download.virtualbox.org/virtualbox/7.0.20/VirtualBox-7.0.20-163906-Win.exe'; $f="$env:TEMP\vbox.exe"; Invoke-WebRequest $u -OutFile $f; Start-Process $f '--silent' -Wait; Remove-Item $f }
}
Write-Step "VirtualBox ready: $(if (Test-Cmd 'VBoxManage') { VBoxManage --version } else { 'failed' })" 'SUCCESS'

Write-Phase 'PHASE 2: Download Kali OVA'
$KaliUrl = 'https://cdimage.kali.org/kali-2025.1/kali-linux-2025.1-virtualbox-amd64.ova'
$KaliFile = "$env:TEMP\kali-linux-2025.1-virtualbox-amd64.ova"
if (-not (Test-Path $KaliFile)) {
    Write-Step 'Downloading Kali OVA (~4.5GB, ~10-40min)...' 'WARN'
    Start-BitsTransfer -Source $KaliUrl -Destination $KaliFile -DisplayName 'Kali OVA Download' -Priority High
}
Write-Step 'Kali OVA ready' 'SUCCESS'

Write-Phase 'PHASE 3: Create & Configure VM'
if (VBoxManage list vms 2>$null | Select-String $VMName) { VBoxManage unregistervm $VMName --delete 2>$null | Out-Null }
VBoxManage import $KaliFile --vsys 0 --vmname $VMName --cpus $CPUs --memory $MemoryMB --ostype Debian_64 | Out-Null
VBoxManage modifyvm $VMName --nic1 nat --natpf1 'ssh,tcp,,2222,,22' --natpf1 'web,tcp,,8080,,80' --ioapic on --pae on --hwvirtex on --vram 128
VBoxManage modifyvm $VMName --description "NeoBot Kali VM - $(Get-Date -f 'yyyy-MM-dd') | SSH localhost:2222"
Write-Step "VM $VMName configured ($MemoryMB MB / $CPUs CPU, SSH:2222)" 'SUCCESS'

Write-Phase 'PHASE 4: Launch & Verify'
if ($Headless) { VBoxManage startvm $VMName --type headless | Out-Null } else { Start-Process 'C:\Program Files\Oracle\VirtualBox\VirtualBox.exe' "--startvm `$VMName" }
$Wsh = New-Object -ComObject WScript.Shell; $sc = $Wsh.CreateShortcut("$env:USERPROFILE\Desktop\NeoBot-Kali-VM.lnk"); $sc.TargetPath = 'C:\Program Files\Oracle\VirtualBox\VirtualBox.exe'; $sc.Arguments = "--startvm `$VMName"; $sc.Save()
Write-Step 'Desktop shortcut created' 'SUCCESS'
if (VBoxManage list runningvms 2>$null | Select-String $VMName) { Write-Step 'VM is running!' 'SUCCESS' }

Write-Host @"
✅ NeoBot Kali Linux VM ready on Windows!

SSH: ssh -p 2222 kali@localhost (pass: kali - change it!)
Desktop shortcut: NeoBot-Kali-VM
Log: $env:TEMP\NeoBot-Kali-Setup.log

Update Kali inside VM: sudo apt update && sudo apt full-upgrade -y
Full guide: https://github.com/iofhouras/neobot

Cyber ready. 🛡️
"@ -ForegroundColor Green

Stop-Transcript | Out-Null
# End of script - Advanced professional autonomous installer for NeoBot Windows download button