[Setup]
AppId={{A1B2C3D4-E5F6-7890-ABCD-EF1234567890}}
AppName=NeoBot
AppVersion=0.2.0
AppPublisher=iofhouras
AppPublisherURL=https://github.com/iofhouras/neobot
AppSupportURL=https://github.com/iofhouras/neobot/issues
AppUpdatesURL=https://github.com/iofhouras/neobot/releases
DefaultDirName={autopf}\Neobot
DefaultGroupName=NeoBot
AllowNoIcons=yes
LicenseFile=
InfoBeforeFile=
OutputDir=Output
OutputBaseFilename=NeoBot-Setup
SetupIconFile=..\assets\neobot-logo.ico
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=admin
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
DisableProgramGroupPage=yes
UninstallDisplayIcon={app}\Neobot.exe
UninstallDisplayName=NeoBot Professional Installer
VersionInfoVersion=0.2.0.0
VersionInfoCompany=NeoBot Project
VersionInfoDescription=Professional Windows Installer for NeoBot + Kali Linux VM

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "launchvm"; Description: "Launch Kali Linux VM after installation"; GroupDescription: "Post-Install Actions"; Flags: unchecked

[Files]
Source: "NeoBot-Windows-Kali-VM-Setup.ps1"; DestDir: "{tmp}"; Flags: deleteafterinstall
; Add any additional assets, config files, or bundled Tauri exe here in future releases
; Source: "..\src-tauri\target\release\neobot.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\NeoBot"; Filename: "{app}\Neobot.exe"; IconFilename: "{app}\Neobot.exe"
Name: "{group}\Kali Linux VM (NeoBot)"; Filename: "C:\Program Files\Oracle\VirtualBox\VirtualBox.exe"; Parameters: "--startvm NeoBot-Kali-VM"
Name: "{autodesktop}\NeoBot"; Filename: "{app}\Neobot.exe"; Tasks: desktopicon
Name: "{autodesktop}\Kali Linux VM"; Filename: "C:\Program Files\Oracle\VirtualBox\VirtualBox.exe"; Parameters: "--startvm NeoBot-Kali-VM"; Tasks: desktopicon

[Run]
Filename: "powershell.exe"; Parameters: "-NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -File ""{tmp}\NeoBot-Windows-Kali-VM-Setup.ps1"" -FullAutomation -Silent -MemoryMB 4096 -CPUs 2 -VMName NeoBot-Kali-VM"; StatusMsg: "Provisioning Kali Linux VM and configuring NeoBot environment... This may take 20-60 minutes. Please wait."; Flags: waituntilterminated; Check: not WizardSilent

[UninstallRun]
Filename: "powershell.exe"; Parameters: "-NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -Command \"if (VBoxManage list vms 2>$null | Select-String 'NeoBot-Kali-VM') { VBoxManage unregistervm NeoBot-Kali-VM --delete }\"; Stop-Transcript | Out-Null"; Flags: waituntilterminated; RunOnceId: "CleanupVM"

[Code]
procedure InitializeWizard;
begin
  WizardForm.WizardBitmapImage.Bitmap := LoadBitmapFromFile(ExpandConstant('{tmp}\neobot-banner.bmp')); // Optional branding
end;

function NextButtonClick(CurPageID: Integer): Boolean;
begin
  Result := True;
  if CurPageID = wpSelectDir then begin
    if not DirExists(ExpandConstant('{app}')) then begin
      if not CreateDir(ExpandConstant('{app}')) then begin
        MsgBox('Failed to create install directory. Please choose another location.', mbError, MB_OK);
        Result := False;
      end;
    end;
  end;
end;

procedure CurPageChanged(CurPageID: Integer);
begin
  if CurPageID = wpFinished then begin
    if MsgBox('Installation complete! Would you like to launch the Kali VM now?', mbConfirmation, MB_YESNO) = IDYES then begin
      Exec('C:\Program Files\Oracle\VirtualBox\VirtualBox.exe', '--startvm NeoBot-Kali-VM', '', SW_SHOWNORMAL, ewNoWait, ResultCode);
    end;
  end;
end;

// Advanced: Add disk space check, admin re-launch logic if needed (Inno handles admin)
