# build-release.ps1 — Builds RustPlayer on Windows with all protections.
#
# Use:
#   .\scripts\build-release.ps1
#   .\scripts\build-release.ps1 -Package # also builds the Inno Setup installer
#
# If you have API keys, define them before running:
#   $env:RUSTPLAYER_LASTFM_KEY="your_key"
#   $env:RUSTPLAYER_OPENSUBS_KEY="your_key"
#   .\scripts\build-release.ps1

param(
    [switch]$Package
)

$ErrorActionPreference = "Stop"

$Binary   = "rustplayer.exe"
$Version  = (Select-String -Path "Cargo.toml" -Pattern '^version\s*=\s*"(.*)"').Matches[0].Groups[1].Value
$TargetDir = "target\release"
$ArtifactsDir = "artifacts"

Write-Host "=== RustPlayer v$Version — build release ===" -ForegroundColor Cyan

# ── Check Rust ────────────────────────────── ──────────────────────────────
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "cargo no encontrado. Instala Rust desde https://rustup.rs"
}

# ── Show active keys ─────────────────────────── ───────────────────────────
Write-Host ""
Write-Host "Keys configuradas:"
if ($env:RUSTPLAYER_LASTFM_KEY) {
    Write-Host "  Last.fm:       OK (variable de entorno)" -ForegroundColor Green
} else {
    Write-Host "  Last.fm:       -- (placeholder compilado)" -ForegroundColor Yellow
}
if ($env:RUSTPLAYER_OPENSUBS_KEY) {
    Write-Host "  OpenSubtitles: OK (variable de entorno)" -ForegroundColor Green
} else {
    Write-Host "  OpenSubtitles: -- (placeholder compilado)" -ForegroundColor Yellow
}
Write-Host ""

# ── Compile ───────────────────────────────── ─────────────────────────────────
Write-Host "Compilando..."
$env:RUSTFLAGS = "-C target-cpu=native"
cargo build --release

if (-not (Test-Path "$TargetDir\$Binary")) {
    Write-Error "El binario no se generó."
}

$Size = (Get-Item "$TargetDir\$Binary").Length / 1MB
Write-Host ""
Write-Host ("OK Binario: $TargetDir\$Binary ({0:F1} MB)" -f $Size) -ForegroundColor Green

# ── Check exposed strings ─────────────────────── ────────────────────────
Write-Host ""
Write-Host "Verificando strings expuestas..."

$HasStrings = Get-Command strings -ErrorAction SilentlyContinue
if ($HasStrings) {
    $leaked = $false

    if ($env:RUSTPLAYER_LASTFM_KEY) {
        $found = strings "$TargetDir\$Binary" | Select-String -SimpleMatch $env:RUSTPLAYER_LASTFM_KEY
        if ($found) { Write-Warning "API key de Last.fm visible en el binario."; $leaked = $true }
    }
    if ($env:RUSTPLAYER_OPENSUBS_KEY) {
        $found = strings "$TargetDir\$Binary" | Select-String -SimpleMatch $env:RUSTPLAYER_OPENSUBS_KEY
        if ($found) { Write-Warning "API key de OpenSubtitles visible en el binario."; $leaked = $true }
    }

    if (-not $leaked) {
        Write-Host "  OK Sin strings sensibles visibles." -ForegroundColor Green
    }
} else {
    Write-Host "  (instala 'strings' de Sysinternals para verificar)" -ForegroundColor Gray
}

# ── Generate installer (optional) ────────────────────── ───────────────────────
if ($Package) {
    $ISCC = "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe"
    if (Test-Path $ISCC) {
        Write-Host ""
        Write-Host "Generando instalador Inno Setup..."
        & $ISCC "installer\windows\rustplayer.iss"
        Write-Host "OK Instalador: installer\windows\output\RustPlayerSetup-$Version.exe" -ForegroundColor Green
    } else {
        Write-Warning "Inno Setup no encontrado en '$ISCC'. Descarga desde https://jrsoftware.org/isinfo.php"
    }
}

Write-Host ""
Write-Host "=== Build completado ===" -ForegroundColor Cyan
