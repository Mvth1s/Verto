#!/usr/bin/env bash
set -e

BINARIES_DIR="$(dirname "$0")/../src-tauri/binaries"
mkdir -p "$BINARIES_DIR"

OS=$(uname -s)
ARCH=$(uname -m)
# TAURI_TARGET allows CI to override arch detection for cross-compilation
TAURI_TARGET="${TAURI_TARGET:-}"

if [ "$OS" = "Linux" ]; then
    if [ "$TAURI_TARGET" = "x86_64-unknown-linux-gnu" ] || { [ -z "$TAURI_TARGET" ] && [ "$ARCH" = "x86_64" ]; }; then
        TARGET="x86_64-unknown-linux-gnu"
        BTBN_FILE="ffmpeg-master-latest-linux64-gpl.tar.xz"
    elif [ "$TAURI_TARGET" = "aarch64-unknown-linux-gnu" ] || { [ -z "$TAURI_TARGET" ] && [ "$ARCH" = "aarch64" ]; }; then
        TARGET="aarch64-unknown-linux-gnu"
        BTBN_FILE="ffmpeg-master-latest-linuxarm64-gpl.tar.xz"
    else
        echo "Architecture Linux non supportée: ${TAURI_TARGET:-$ARCH}"
        exit 1
    fi

    DEST="$BINARIES_DIR/ffmpeg-$TARGET"
    if [ -f "$DEST" ] && [ -s "$DEST" ]; then
        echo "ffmpeg-$TARGET déjà présent, skip."
        exit 0
    fi

    # Préférer apt-get quand disponible (CI Debian/Ubuntu, dev local)
    if command -v apt-get &>/dev/null; then
        if ! command -v ffmpeg &>/dev/null; then
            sudo apt-get install -y --no-install-recommends ffmpeg
        fi
        cp "$(which ffmpeg)" "$DEST"
        chmod +x "$DEST"
        echo "FFmpeg installé dans $DEST (via apt-get)"
        exit 0
    fi

    # Fallback : build statique GitHub Releases (BtbN)
    URL="https://github.com/BtbN/FFmpeg-Builds/releases/latest/download/${BTBN_FILE}"
    TMP=$(mktemp -d)
    echo "Téléchargement de $URL..."
    curl -sSfL --max-time 120 "$URL" -o "$TMP/ffmpeg.tar.xz"
    tar -xJf "$TMP/ffmpeg.tar.xz" -C "$TMP"
    FFMPEG_BIN=$(find "$TMP" -name "ffmpeg" -type f | head -1)
    if [ -z "$FFMPEG_BIN" ]; then
        echo "Erreur: binaire ffmpeg introuvable dans l'archive"
        exit 1
    fi
    cp "$FFMPEG_BIN" "$DEST"
    chmod +x "$DEST"
    rm -rf "$TMP"
    echo "FFmpeg installé dans $DEST (via BtbN)"

elif [ "$OS" = "Darwin" ]; then
    if [ "$TAURI_TARGET" = "x86_64-apple-darwin" ] || { [ -z "$TAURI_TARGET" ] && [ "$ARCH" = "x86_64" ]; }; then
        TARGET="x86_64-apple-darwin"
        NEED_X86=true
    elif [ "$TAURI_TARGET" = "aarch64-apple-darwin" ] || { [ -z "$TAURI_TARGET" ] && [ "$ARCH" = "arm64" ]; }; then
        TARGET="aarch64-apple-darwin"
        NEED_X86=false
    else
        echo "Architecture macOS non supportée: ${TAURI_TARGET:-$ARCH}"
        exit 1
    fi

    DEST="$BINARIES_DIR/ffmpeg-$TARGET"
    if [ -f "$DEST" ] && [ -s "$DEST" ]; then
        echo "ffmpeg-$TARGET déjà présent, skip."
        exit 0
    fi

    if [ "$NEED_X86" = true ] && [ "$ARCH" = "arm64" ]; then
        # Cross-compilation: ARM host targeting x86_64.
        # Use `brew fetch --bottle-tag=ventura` to download the Intel (x86_64) macOS 13 bottle.
        # Homebrew bottle tags: plain version name = Intel, arm64_ prefix = Apple Silicon.
        echo "Cross-compilation: téléchargement du bottle FFmpeg Intel (x86_64)..."
        TMP=$(mktemp -d)
        BOTTLE_FILE=""
        for BOTTLE_TAG in ventura sonoma monterey; do
            FETCH_OUT=$(brew fetch --bottle-tag="$BOTTLE_TAG" --no-deps ffmpeg 2>&1) && {
                BOTTLE_FILE=$(echo "$FETCH_OUT" | grep -E "^(Already downloaded|Downloaded to):" | head -1 | sed 's/^[^:]*: //')
                [ -n "$BOTTLE_FILE" ] && [ -f "$BOTTLE_FILE" ] && break
                BOTTLE_FILE=""
            }
        done
        if [ -z "$BOTTLE_FILE" ] || [ ! -f "$BOTTLE_FILE" ]; then
            echo "Erreur: bottle FFmpeg Intel introuvable. Sortie brew: $FETCH_OUT"
            rm -rf "$TMP"
            exit 1
        fi
        tar -xzf "$BOTTLE_FILE" -C "$TMP"
        FFMPEG_BIN=$(find "$TMP" -path "*/bin/ffmpeg" -type f | head -1)
        [ -z "$FFMPEG_BIN" ] && FFMPEG_BIN=$(find "$TMP" -name "ffmpeg" -type f | head -1)
        if [ -z "$FFMPEG_BIN" ]; then
            echo "Erreur: binaire ffmpeg introuvable dans le bottle"
            rm -rf "$TMP"
            exit 1
        fi
        cp "$FFMPEG_BIN" "$DEST"
        rm -rf "$TMP"
    else
        if ! command -v ffmpeg &>/dev/null; then
            echo "Installation de FFmpeg via Homebrew..."
            brew install --quiet ffmpeg
        fi
        cp "$(which ffmpeg)" "$DEST"
    fi
    chmod +x "$DEST"
    echo "FFmpeg installé dans $DEST"

else
    echo "OS non supporté: $OS (utilisez download-ffmpeg.ps1 sur Windows)"
    exit 1
fi
