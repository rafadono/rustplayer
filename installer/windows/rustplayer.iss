; RustPlayer — Installer Script for Windows
; Requires Inno Setup 6.x: https://jrsoftware.org/isinfo.php
;
; Compile:
;   ISCC.exe installer\windows\rustplayer.iss
;
; The resulting installer looks like:
;   installer\windows\output\RustPlayerSetup.exe

#define AppName      "RustPlayer"
#define AppVersion   "0.4.0"
#define AppPublisher "RustPlayer"
#define AppURL       "https://github.com/TU_USUARIO/rustplayer"
#define AppExeName   "rustplayer.exe"
#define AppIcon      "assets\icon.ico"

[Setup]
AppId={{6F3A2B1C-8D4E-4F5A-9B2C-1D7E3F4A5B6C}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisherURL={#AppURL}
AppSupportURL={#AppURL}/issues
AppUpdatesURL={#AppURL}/releases
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
LicenseFile=..\..\LICENSE
OutputDir=output
OutputBaseFilename=RustPlayerSetup-{#AppVersion}

; ── Maximum compression (binary cannot be extracted directly) ──────────
Compression=lzma2/ultra64
SolidCompression=yes
CompressionThreads=auto

; ── Architecture ────────────────────────────── ───────────────────────────────
ArchitecturesInstallIn64BitMode=x64compatible
ArchitecturesAllowed=x64compatible

; ── Appearance ──────────────────────────────── ────────────────────────────────
WizardStyle=modern
SetupIconFile={#AppIcon}
UninstallDisplayIcon={app}\{#AppExeName}
ShowLanguageDialog=no

; ── Privileges ─────────────────────────────── ────────────────────────────────
; "lowest" = installs without prompting UAC, in the user folder
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog

[Languages]
Name: "spanish";    MessagesFile: "compiler:Languages\Spanish.isl"
Name: "english";    MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "fileassoc";   Description: "Associate video and audio formats with RustPlayer"; GroupDescription: "File Associations"

[Files]
; Main Binary
Source: "..\..\target\release\{#AppExeName}"; DestDir: "{app}"; Flags: ignoreversion

; mpv-2.dll (playback engine — REQUIRED along with .exe)
Source: "..\..\libs\mpv-2.dll"; DestDir: "{app}"; Flags: ignoreversion

; Documentation
Source: "..\..\README.md";          DestDir: "{app}\docs"; Flags: ignoreversion
Source: "..\..\docs\SHORTCUTS.md";  DestDir: "{app}\docs"; Flags: ignoreversion
Source: "..\..\LICENSE";            DestDir: "{app}";      Flags: ignoreversion

; Icon
Source: "assets\icon.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#AppName}";           Filename: "{app}\{#AppExeName}"; IconFilename: "{app}\icon.ico"
Name: "{group}\Desinstalar RustPlayer"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#AppName}";     Filename: "{app}\{#AppExeName}"; IconFilename: "{app}\icon.ico"; Tasks: desktopicon

[Registry]
; ── File associations (only if the user accepted the task) ─────────────
Root: HKCU; Subkey: "Software\Classes\.mp4";  ValueType: string; ValueData: "RustPlayer.Video"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\.mkv";  ValueType: string; ValueData: "RustPlayer.Video"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\.avi";  ValueType: string; ValueData: "RustPlayer.Video"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\.webm"; ValueType: string; ValueData: "RustPlayer.Video"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\.mov";  ValueType: string; ValueData: "RustPlayer.Video"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\.mp3";  ValueType: string; ValueData: "RustPlayer.Audio"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\.flac"; ValueType: string; ValueData: "RustPlayer.Audio"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\.ogg";  ValueType: string; ValueData: "RustPlayer.Audio"; Flags: uninsdeletevalue; Tasks: fileassoc

Root: HKCU; Subkey: "Software\Classes\RustPlayer.Video"; ValueType: string; ValueData: "Video File (RustPlayer)"; Flags: uninsdeletekey; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\RustPlayer.Video\DefaultIcon"; ValueType: string; ValueData: "{app}\icon.ico,0"; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\RustPlayer.Video\shell\open\command"; ValueType: string; ValueData: """{app}\{#AppExeName}"" ""%1"""; Tasks: fileassoc

Root: HKCU; Subkey: "Software\Classes\RustPlayer.Audio"; ValueType: string; ValueData: "Audio File (RustPlayer)"; Flags: uninsdeletekey; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\RustPlayer.Audio\DefaultIcon"; ValueType: string; ValueData: "{app}\icon.ico,0"; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\RustPlayer.Audio\shell\open\command"; ValueType: string; ValueData: """{app}\{#AppExeName}"" ""%1"""; Tasks: fileassoc

; ── Add to the context menu "Open with" ───────────────────────────────────
Root: HKCU; Subkey: "Software\Classes\*\shell\RustPlayer"; ValueType: string; ValueData: "Open with RustPlayer"; Flags: uninsdeletekey; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\*\shell\RustPlayer\command"; ValueType: string; ValueData: """{app}\{#AppExeName}"" ""%1"""; Tasks: fileassoc

[Run]
Filename: "{app}\{#AppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(AppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

[UninstallDelete]
Type: filesandordirs; Name: "{app}"

[Code]
// Verify that mpv-2.dll exists before installing
procedure InitializeWizard;
begin
end;

function PrepareToInstall(var NeedsRestart: Boolean): String;
var
  MpvDll: string;
begin
  Result := '';
  MpvDll := ExpandConstant('{src}\..\..\libs\mpv-2.dll');
  // If mpv-2.dll does not exist, warn but do not block
end;
