$ErrorActionPreference = "Stop"

$PANDOC_VERSION = "3.6.4"
$BINARIES_DIR = Join-Path $PSScriptRoot "..\src-tauri\binaries"
New-Item -ItemType Directory -Force -Path $BINARIES_DIR | Out-Null

$TARGET = "x86_64-pc-windows-msvc"
$DEST = Join-Path $BINARIES_DIR "pandoc-$TARGET.exe"

if (Test-Path $DEST) {
    Write-Host "pandoc-$TARGET.exe déjà présent, skip."
    exit 0
}

$URL = "https://github.com/jgm/pandoc/releases/download/$PANDOC_VERSION/pandoc-$PANDOC_VERSION-windows-x86_64.zip"
$TMP = Join-Path ([System.IO.Path]::GetTempPath()) ([System.IO.Path]::GetRandomFileName())
New-Item -ItemType Directory -Path $TMP | Out-Null

Write-Host "Téléchargement de $URL..."
Invoke-WebRequest -Uri $URL -OutFile "$TMP\pandoc.zip"
Expand-Archive "$TMP\pandoc.zip" -DestinationPath $TMP
Copy-Item "$TMP\pandoc-$PANDOC_VERSION\pandoc.exe" $DEST
Remove-Item -Recurse -Force $TMP
Write-Host "Pandoc installé dans $DEST"
