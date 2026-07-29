# build-release.ps1 — Builds RustPlayer on Windows with all protections.
#
# Use:
#   .\scripts\build-release.ps1
#   .\scripts\build-release.ps1 -Package # also builds the Inno Setup installer
#
# One-time dev machine setup (generates mpv.lib for MSVC linking from a
# downloaded libmpv DLL, then exits without building):
#   .\scripts\build-release.ps1 -SetupMpv "C:\path\to\mpv-2.dll"
#
# If you have API keys, define them before running:
#   $env:RUSTPLAYER_LASTFM_KEY="your_key"
#   $env:RUSTPLAYER_OPENSUBS_KEY="your_key"
#   .\scripts\build-release.ps1

param(
    [switch]$Package,
    [string]$SetupMpv
)

$ErrorActionPreference = "Stop"

function Invoke-SetupMpv {
    param([string]$MpvDllPath)

    $mpvDll = Resolve-Path $MpvDllPath
    $mpvDir = Split-Path -Parent $mpvDll
    $mpvDllName = Split-Path -Leaf $mpvDll
    $mpvLib = Join-Path $mpvDir "mpv.lib"

    if (-not (Test-Path $mpvDll)) {
        throw "mpv DLL does not exist at: $MpvDllPath"
    }

    if (-not (Test-Path $mpvLib)) {
        $vsBinRoots = Get-ChildItem "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC" -Directory -ErrorAction SilentlyContinue |
            Sort-Object Name -Descending |
            ForEach-Object { Join-Path $_.FullName "bin\Hostx64\x64" }

        $toolsDir = $vsBinRoots |
            Where-Object {
                $hasDumpbin = Test-Path (Join-Path $_ "dumpbin.exe")
                $hasLib = Test-Path (Join-Path $_ "lib.exe")
                if ($hasDumpbin) { return $hasLib }
                return $false
            } |
            Select-Object -First 1
        if (-not $toolsDir) {
            throw "dumpbin.exe/lib.exe from Visual Studio Build Tools not found."
        }

        $dumpbin = Join-Path $toolsDir "dumpbin.exe"
        $libexe = Join-Path $toolsDir "lib.exe"
        $defFile = Join-Path $mpvDir "mpv.def"

        $exports = & $dumpbin /exports $mpvDll
        $hasRequiredSymbol = ($exports | Select-String -Pattern "mpv_get_time_ns" -SimpleMatch)
        if (-not $hasRequiredSymbol) {
            throw "The specified DLL ($mpvDllName) does not export mpv_get_time_ns. You need a newer/compatible version of libmpv."
        }

        $symbols = $exports |
            ForEach-Object { $_.ToString() } |
            Where-Object { $_ -match "^\s+\d+\s+[0-9A-F]+\s+[0-9A-F]+\s+(\S+)$" } |
            ForEach-Object { ([regex]::Match($_, "^\s+\d+\s+[0-9A-F]+\s+[0-9A-F]+\s+(\S+)$")).Groups[1].Value } |
            Where-Object {
                if (-not $_) { return $false }
                return ($_ -ne "[NONAME]")
            }

        if (-not $symbols -or $symbols.Count -eq 0) {
            throw "Could not extract exported symbols from $mpvDll"
        }

        @("LIBRARY $mpvDllName", "EXPORTS") + $symbols | Set-Content -Encoding ASCII $defFile
        & $libexe /def:$defFile /machine:x64 /out:$mpvLib | Out-Null

        if (-not (Test-Path $mpvLib)) {
            throw "Could not generate mpv.lib"
        }
    }

    Write-Host "mpv.lib detected/generated at: $mpvLib"
    Write-Host "Use these commands in this terminal:"
    Write-Host ""
    Write-Host "  `$env:RPLAYER_MPV_LIB_DIR = '$mpvDir'"
    Write-Host "  `$env:PATH = '$mpvDir;' + `$env:PATH"
    Write-Host "  cargo run"
    Write-Host ""
    Write-Host "If you want to persist it for your user:"
    Write-Host "  setx RPLAYER_MPV_LIB_DIR `"$mpvDir`""
}

if ($SetupMpv) {
    Invoke-SetupMpv -MpvDllPath $SetupMpv
    return
}

$Binary   = "rplayer.exe"
$Version  = (Select-String -Path "Cargo.toml" -Pattern '^version\s*=\s*"(.*)"').Matches[0].Groups[1].Value
$TargetDir = "target\release"
$ArtifactsDir = "artifacts"

Write-Host "=== RustPlayer v$Version — build release ===" -ForegroundColor Cyan

# ── Check Rust ────────────────────────────── ──────────────────────────────
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "cargo not found. Install Rust from https://rustup.rs"
}

# ── Show active keys ─────────────────────────── ───────────────────────────
Write-Host ""
Write-Host "Configured keys:"
if ($env:RUSTPLAYER_LASTFM_KEY) {
    Write-Host "  Last.fm:       OK (environment variable)" -ForegroundColor Green
} else {
    Write-Host "  Last.fm:       -- (compiled placeholder)" -ForegroundColor Yellow
}
if ($env:RUSTPLAYER_OPENSUBS_KEY) {
    Write-Host "  OpenSubtitles: OK (environment variable)" -ForegroundColor Green
} else {
    Write-Host "  OpenSubtitles: -- (compiled placeholder)" -ForegroundColor Yellow
}
Write-Host ""

# ── Compile ───────────────────────────────── ─────────────────────────────────
Write-Host "Building..."
$env:RUSTFLAGS = "-C target-cpu=native"
cargo build --release

if (-not (Test-Path "$TargetDir\$Binary")) {
    Write-Error "The binary was not generated."
}

$Size = (Get-Item "$TargetDir\$Binary").Length / 1MB
Write-Host ""
Write-Host ("OK Binary: $TargetDir\$Binary ({0:F1} MB)" -f $Size) -ForegroundColor Green

# ── Check exposed strings ─────────────────────── ────────────────────────
Write-Host ""
Write-Host "Checking for exposed strings..."

$HasStrings = Get-Command strings -ErrorAction SilentlyContinue
if ($HasStrings) {
    $leaked = $false

    if ($env:RUSTPLAYER_LASTFM_KEY) {
        $found = strings "$TargetDir\$Binary" | Select-String -SimpleMatch $env:RUSTPLAYER_LASTFM_KEY
        if ($found) { Write-Warning "The Last.fm API key is visible in the binary."; $leaked = $true }
    }
    if ($env:RUSTPLAYER_OPENSUBS_KEY) {
        $found = strings "$TargetDir\$Binary" | Select-String -SimpleMatch $env:RUSTPLAYER_OPENSUBS_KEY
        if ($found) { Write-Warning "The OpenSubtitles API key is visible in the binary."; $leaked = $true }
    }

    if (-not $leaked) {
        Write-Host "  OK No sensitive strings visible." -ForegroundColor Green
    }
} else {
    Write-Host "  (install Sysinternals 'strings' to check)" -ForegroundColor Gray
}

# ── Generate installer (optional) ────────────────────── ───────────────────────
if ($Package) {
    $ISCC = "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe"
    if (Test-Path $ISCC) {
        Write-Host ""
        Write-Host "Generating Inno Setup installer..."
        & $ISCC "installer\windows\rustplayer.iss"
        Write-Host "OK Installer: installer\windows\output\RustPlayerSetup-$Version.exe" -ForegroundColor Green
    } else {
        Write-Warning "Inno Setup not found at '$ISCC'. Download it from https://jrsoftware.org/isinfo.php"
    }
}

Write-Host ""
Write-Host "=== Build complete ===" -ForegroundColor Cyan
