$ErrorActionPreference = "Stop"

$BINARIES_DIR = Join-Path $PSScriptRoot "..\src-tauri\binaries"
New-Item -ItemType Directory -Force -Path $BINARIES_DIR | Out-Null

$TARGET = "x86_64-pc-windows-msvc"
$DEST = Join-Path $BINARIES_DIR "ffmpeg-$TARGET.exe"

if (Test-Path $DEST) {
    Write-Host "ffmpeg-$TARGET.exe déjà présent, skip."
    exit 0
}

$URL = "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip"
$TMP = Join-Path ([System.IO.Path]::GetTempPath()) ([System.IO.Path]::GetRandomFileName())
New-Item -ItemType Directory -Path $TMP | Out-Null

Write-Host "Téléchargement de $URL..."
Invoke-WebRequest -Uri $URL -OutFile "$TMP\ffmpeg.zip"
Expand-Archive "$TMP\ffmpeg.zip" -DestinationPath $TMP
$FFMPEG_BIN = Get-ChildItem -Recurse -Filter "ffmpeg.exe" -Path $TMP |
    Where-Object { $_.DirectoryName -like "*\bin" } |
    Select-Object -First 1
if (-not $FFMPEG_BIN) {
    Write-Error "Erreur: ffmpeg.exe introuvable dans l'archive"
    exit 1
}
Copy-Item $FFMPEG_BIN.FullName $DEST
Remove-Item -Recurse -Force $TMP
Write-Host "FFmpeg installé dans $DEST"
