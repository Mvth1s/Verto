#!/usr/bin/env bash
set -e

PANDOC_VERSION="3.6.4"
BINARIES_DIR="$(dirname "$0")/../src-tauri/binaries"
mkdir -p "$BINARIES_DIR"

OS=$(uname -s)
ARCH=$(uname -m)

if [ "$OS" = "Linux" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        TARGET="x86_64-unknown-linux-gnu"
        PANDOC_ARCH="amd64"
    elif [ "$ARCH" = "aarch64" ]; then
        TARGET="aarch64-unknown-linux-gnu"
        PANDOC_ARCH="arm64"
    else
        echo "Architecture Linux non supportée: $ARCH"
        exit 1
    fi
    URL="https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/pandoc-${PANDOC_VERSION}-linux-${PANDOC_ARCH}.tar.gz"
    BINARY_IN_ARCHIVE="pandoc-${PANDOC_VERSION}/bin/pandoc"
    EXT="tar.gz"
elif [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        TARGET="x86_64-apple-darwin"
        PANDOC_MACOS_ARCH="x86_64"
    elif [ "$ARCH" = "arm64" ]; then
        TARGET="aarch64-apple-darwin"
        PANDOC_MACOS_ARCH="arm64"
    else
        echo "Architecture macOS non supportée: $ARCH"
        exit 1
    fi
    URL="https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/pandoc-${PANDOC_VERSION}-${PANDOC_MACOS_ARCH}-macOS.zip"
    BINARY_IN_ARCHIVE="pandoc-${PANDOC_VERSION}/bin/pandoc"
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
else
    unzip -q "$TMP/pandoc.$EXT" -d "$TMP"
fi

cp "$TMP/$BINARY_IN_ARCHIVE" "$DEST"
chmod +x "$DEST"
rm -rf "$TMP"
echo "Pandoc installé dans $DEST"
