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
        # GitHub Apple Silicon runners have x86_64 Homebrew at /usr/local (via Rosetta 2).
        X86_BREW="/usr/local/bin/brew"
        if [ ! -f "$X86_BREW" ]; then
            echo "Erreur: x86_64 Homebrew introuvable à /usr/local/bin/brew (requis pour cross-compilation macOS x86_64)"
            exit 1
        fi
        if ! arch -x86_64 "$X86_BREW" list --formula | grep -q "^ffmpeg$"; then
            echo "Installation de FFmpeg x86_64 via Homebrew Rosetta..."
            arch -x86_64 "$X86_BREW" install --quiet ffmpeg
        fi
        cp "$(arch -x86_64 "$X86_BREW" --prefix ffmpeg)/bin/ffmpeg" "$DEST"
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
