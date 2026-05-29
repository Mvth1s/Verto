#!/usr/bin/env bash
set -e

BINARIES_DIR="$(dirname "$0")/../src-tauri/binaries"
mkdir -p "$BINARIES_DIR"

OS=$(uname -s)
ARCH=$(uname -m)

if [ "$OS" = "Linux" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        TARGET="x86_64-unknown-linux-gnu"
        URL="https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz"
    elif [ "$ARCH" = "aarch64" ]; then
        TARGET="aarch64-unknown-linux-gnu"
        URL="https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-arm64-static.tar.xz"
    else
        echo "Architecture Linux non supportée: $ARCH"
        exit 1
    fi

    DEST="$BINARIES_DIR/ffmpeg-$TARGET"
    if [ -f "$DEST" ]; then
        echo "ffmpeg-$TARGET déjà présent, skip."
        exit 0
    fi

    TMP=$(mktemp -d)
    echo "Téléchargement de $URL..."
    curl -sSL "$URL" -o "$TMP/ffmpeg.tar.xz"
    tar -xJf "$TMP/ffmpeg.tar.xz" -C "$TMP"
    FFMPEG_BIN=$(find "$TMP" -maxdepth 2 -name "ffmpeg" -type f | head -1)
    if [ -z "$FFMPEG_BIN" ]; then
        echo "Erreur: binaire ffmpeg introuvable dans l'archive"
        exit 1
    fi
    cp "$FFMPEG_BIN" "$DEST"
    chmod +x "$DEST"
    rm -rf "$TMP"
    echo "FFmpeg installé dans $DEST"

elif [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        TARGET="x86_64-apple-darwin"
    elif [ "$ARCH" = "arm64" ]; then
        TARGET="aarch64-apple-darwin"
    else
        echo "Architecture macOS non supportée: $ARCH"
        exit 1
    fi

    DEST="$BINARIES_DIR/ffmpeg-$TARGET"
    if [ -f "$DEST" ]; then
        echo "ffmpeg-$TARGET déjà présent, skip."
        exit 0
    fi

    if ! command -v ffmpeg &>/dev/null; then
        echo "Installation de FFmpeg via Homebrew..."
        brew install --quiet ffmpeg
    fi
    cp "$(which ffmpeg)" "$DEST"
    chmod +x "$DEST"
    echo "FFmpeg installé dans $DEST"

else
    echo "OS non supporté: $OS (utilisez download-ffmpeg.ps1 sur Windows)"
    exit 1
fi
