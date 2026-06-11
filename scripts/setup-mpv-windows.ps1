param(
    [Parameter(Mandatory = $true)]
    [string]$MpvDllPath
)

$ErrorActionPreference = "Stop"

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
