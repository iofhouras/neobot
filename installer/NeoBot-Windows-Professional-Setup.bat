@echo off
:: NeoBot Professional Windows Installer v3.0
:: Double-click this .bat to launch the advanced autonomous Kali VM + AI Agent installer
:: Requests full admin permission to make system changes (install VirtualBox, download 25GB Kali, configure VM)

setlocal enabledelayedexpansion

:: Self-elevate to Administrator if not already
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo Requesting Administrator permission to install NeoBot...
    echo This will allow changes to system: VirtualBox installation, VM creation, network config.
    powershell -Command "Start-Process -Verb RunAs -FilePath '%~f0' -ArgumentList '%*'; exit /b"
    exit /b
)

:: Now running as Admin
cd /d "%~dp0"

:: Launch the professional PowerShell installer with full automation
powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& { .\NeoBot-Windows-Kali-VM-Setup.ps1 -FullAutomation -EnableGUIProgress }"

if %errorLevel% neq 0 (
    echo.
    echo [ERROR] Installer encountered an issue. Check the log in %%TEMP%%\NeoBot-Kali-Setup.log
    pause
)

exit /b