#!/usr/bin/env bash
set -e

PANDOC_VERSION="3.6.4"
BINARIES_DIR="$(dirname "$0")/../src-tauri/binaries"
mkdir -p "$BINARIES_DIR"

ARCH=$(uname -m)
if [ "$ARCH" = "x86_64" ]; then
    TARGET="x86_64-unknown-linux-gnu"
    PANDOC_ARCH="amd64"
elif [ "$ARCH" = "aarch64" ]; then
    TARGET="aarch64-unknown-linux-gnu"
    PANDOC_ARCH="arm64"
else
    echo "Architecture non supportée: $ARCH"
    exit 1
fi

DEST="$BINARIES_DIR/pandoc-$TARGET"

if [ -f "$DEST" ]; then
    echo "pandoc-$TARGET déjà présent, skip."
    exit 0
fi

TMP=$(mktemp -d)
URL="https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/pandoc-${PANDOC_VERSION}-linux-${PANDOC_ARCH}.tar.gz"
echo "Téléchargement de $URL..."
curl -sSL "$URL" -o "$TMP/pandoc.tar.gz"
tar -xzf "$TMP/pandoc.tar.gz" -C "$TMP"
cp "$TMP/pandoc-${PANDOC_VERSION}/bin/pandoc" "$DEST"
chmod +x "$DEST"
rm -rf "$TMP"
echo "Pandoc installé dans $DEST"
