#!/usr/bin/env bash
set -e

PANDOC_VERSION="3.6.4"
BINARIES_DIR="$(dirname "$0")/../src-tauri/binaries"
mkdir -p "$BINARIES_DIR"

OS=$(uname -s)
ARCH=$(uname -m)
# TAURI_TARGET allows CI to override arch detection for cross-compilation
TAURI_TARGET="${TAURI_TARGET:-}"

if [ "$OS" = "Linux" ]; then
    if [ "$TAURI_TARGET" = "x86_64-unknown-linux-gnu" ] || { [ -z "$TAURI_TARGET" ] && [ "$ARCH" = "x86_64" ]; }; then
        TARGET="x86_64-unknown-linux-gnu"
        PANDOC_ARCH="amd64"
    elif [ "$TAURI_TARGET" = "aarch64-unknown-linux-gnu" ] || { [ -z "$TAURI_TARGET" ] && [ "$ARCH" = "aarch64" ]; }; then
        TARGET="aarch64-unknown-linux-gnu"
        PANDOC_ARCH="arm64"
    else
        echo "Architecture Linux non supportée: ${TAURI_TARGET:-$ARCH}"
        exit 1
    fi
    URL="https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/pandoc-${PANDOC_VERSION}-linux-${PANDOC_ARCH}.tar.gz"
    EXT="tar.gz"
elif [ "$OS" = "Darwin" ]; then
    if [ "$TAURI_TARGET" = "x86_64-apple-darwin" ] || { [ -z "$TAURI_TARGET" ] && [ "$ARCH" = "x86_64" ]; }; then
        TARGET="x86_64-apple-darwin"
        PANDOC_MACOS_ARCH="x86_64"
    elif [ "$TAURI_TARGET" = "aarch64-apple-darwin" ] || { [ -z "$TAURI_TARGET" ] && [ "$ARCH" = "arm64" ]; }; then
        TARGET="aarch64-apple-darwin"
        PANDOC_MACOS_ARCH="arm64"
    else
        echo "Architecture macOS non supportée: ${TAURI_TARGET:-$ARCH}"
        exit 1
    fi
    URL="https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/pandoc-${PANDOC_VERSION}-${PANDOC_MACOS_ARCH}-macOS.zip"
    EXT="zip"
else
    echo "OS non supporté: $OS (utilisez download-pandoc.ps1 sur Windows)"
    exit 1
fi

DEST="$BINARIES_DIR/pandoc-$TARGET"

if [ -f "$DEST" ]; then
    echo "pandoc-$TARGET déjà présent, skip."
    exit 0
fi

TMP=$(mktemp -d)
echo "Téléchargement de $URL..."
curl -sSL "$URL" -o "$TMP/pandoc.$EXT"

if [ "$EXT" = "tar.gz" ]; then
    tar -xzf "$TMP/pandoc.$EXT" -C "$TMP"
    cp "$TMP/pandoc-${PANDOC_VERSION}/bin/pandoc" "$DEST"
else
    unzip -q "$TMP/pandoc.$EXT" -d "$TMP"
    PANDOC_BIN=$(find "$TMP" -name "pandoc" -type f | head -1)
    if [ -z "$PANDOC_BIN" ]; then
        echo "Erreur: binaire pandoc introuvable dans l'archive"
        exit 1
    fi
    cp "$PANDOC_BIN" "$DEST"
fi

chmod +x "$DEST"
rm -rf "$TMP"
echo "Pandoc installé dans $DEST"
