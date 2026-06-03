#!/usr/bin/env bash
# Downloads FFmpeg and Pandoc sidecars for the current (or specified) platform
# and places them in apps/desktop/src-tauri/binaries/ using the Tauri naming convention.
#
# Usage: ./scripts/download-sidecars.sh [--platform linux|macos|windows]
#
# The bundled-sidecar approach (existing CI scripts) remains the default.
# Use this script when you need to populate the binaries directory manually.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BINARIES_DIR="$REPO_ROOT/apps/desktop/src-tauri/binaries"
mkdir -p "$BINARIES_DIR"

# ── Platform detection ────────────────────────────────────────────────────────

PLATFORM=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --platform)
      PLATFORM="$2"
      shift 2
      ;;
    *)
      echo "Unknown argument: $1" >&2
      exit 1
      ;;
  esac
done

if [[ -z "$PLATFORM" ]]; then
  OS="$(uname -s)"
  case "$OS" in
    Linux)   PLATFORM="linux" ;;
    Darwin)  PLATFORM="macos" ;;
    *)
      echo "Cannot auto-detect platform for OS '$OS'. Pass --platform linux|macos|windows." >&2
      exit 1
      ;;
  esac
fi

ARCH="$(uname -m)"

case "$PLATFORM" in
  linux)
    case "$ARCH" in
      x86_64)  RUST_TARGET="x86_64-unknown-linux-gnu"  ;;
      aarch64) RUST_TARGET="aarch64-unknown-linux-gnu" ;;
      *) echo "Unsupported arch: $ARCH" >&2; exit 1 ;;
    esac
    ;;
  macos)
    case "$ARCH" in
      x86_64)  RUST_TARGET="x86_64-apple-darwin"   ;;
      arm64)   RUST_TARGET="aarch64-apple-darwin"   ;;
      *) echo "Unsupported arch: $ARCH" >&2; exit 1 ;;
    esac
    ;;
  windows)
    RUST_TARGET="x86_64-pc-windows-msvc"
    ;;
  *)
    echo "Unknown platform '$PLATFORM'. Expected linux, macos, or windows." >&2
    exit 1
    ;;
esac

echo "Platform : $PLATFORM ($RUST_TARGET)"
echo "Output   : $BINARIES_DIR"
echo ""

# ── Helpers ───────────────────────────────────────────────────────────────────

download() {
  local url="$1" dest="$2"
  echo "  Downloading $(basename "$url")…"
  curl -sSfL --max-time 300 "$url" -o "$dest"
}

# ── FFmpeg ────────────────────────────────────────────────────────────────────

install_ffmpeg() {
  local DEST="$BINARIES_DIR/ffmpeg-$RUST_TARGET"
  [[ "$PLATFORM" == "windows" ]] && DEST="${DEST}.exe"

  if [[ -f "$DEST" && -s "$DEST" ]]; then
    echo "✓ FFmpeg already present, skipping."
    return
  fi

  echo "→ Installing FFmpeg…"

  case "$PLATFORM" in
    linux)
      # Prefer system package manager when available (CI / Debian/Ubuntu dev)
      if command -v apt-get &>/dev/null; then
        if ! command -v ffmpeg &>/dev/null; then
          sudo apt-get install -y --no-install-recommends ffmpeg
        fi
        cp "$(command -v ffmpeg)" "$DEST"
        chmod +x "$DEST"
        echo "  Installed from apt-get → $DEST"
        return
      fi

      # Fallback: static build from BtbN
      local BTBN_FILE
      case "$ARCH" in
        x86_64)  BTBN_FILE="ffmpeg-master-latest-linux64-gpl.tar.xz"    ;;
        aarch64) BTBN_FILE="ffmpeg-master-latest-linuxarm64-gpl.tar.xz" ;;
      esac
      local TMP
      TMP="$(mktemp -d)"
      download "https://github.com/BtbN/FFmpeg-Builds/releases/latest/download/${BTBN_FILE}" \
        "$TMP/ffmpeg.tar.xz"
      tar -xJf "$TMP/ffmpeg.tar.xz" -C "$TMP"
      local BIN
      BIN="$(find "$TMP" -name "ffmpeg" -type f | head -1)"
      [[ -z "$BIN" ]] && { echo "  Error: ffmpeg binary not found in archive" >&2; exit 1; }
      cp "$BIN" "$DEST"
      chmod +x "$DEST"
      rm -rf "$TMP"
      echo "  Installed from BtbN → $DEST"
      ;;

    macos)
      if ! command -v ffmpeg &>/dev/null; then
        echo "  Installing FFmpeg via Homebrew…"
        brew install --quiet ffmpeg
      fi
      cp "$(command -v ffmpeg)" "$DEST"
      chmod +x "$DEST"
      echo "  Installed from Homebrew → $DEST"
      ;;

    windows)
      echo "  Windows FFmpeg: run download-ffmpeg.ps1 from apps/desktop/scripts/ instead." >&2
      exit 1
      ;;
  esac
}

# ── Pandoc ────────────────────────────────────────────────────────────────────

install_pandoc() {
  local PANDOC_VERSION
  # Read from existing script to stay in sync
  PANDOC_VERSION="$(grep '^PANDOC_VERSION=' \
    "$REPO_ROOT/apps/desktop/scripts/download-pandoc.sh" | cut -d'"' -f2)"
  PANDOC_VERSION="${PANDOC_VERSION:-3.6.4}"

  local DEST="$BINARIES_DIR/pandoc-$RUST_TARGET"
  [[ "$PLATFORM" == "windows" ]] && DEST="${DEST}.exe"

  if [[ -f "$DEST" && -s "$DEST" ]]; then
    echo "✓ Pandoc already present, skipping."
    return
  fi

  echo "→ Installing Pandoc ${PANDOC_VERSION}…"

  case "$PLATFORM" in
    linux)
      local PANDOC_ARCH
      case "$ARCH" in
        x86_64)  PANDOC_ARCH="amd64" ;;
        aarch64) PANDOC_ARCH="arm64" ;;
      esac
      local TMP
      TMP="$(mktemp -d)"
      download \
        "https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/pandoc-${PANDOC_VERSION}-linux-${PANDOC_ARCH}.tar.gz" \
        "$TMP/pandoc.tar.gz"
      tar -xzf "$TMP/pandoc.tar.gz" -C "$TMP"
      cp "$TMP/pandoc-${PANDOC_VERSION}/bin/pandoc" "$DEST"
      chmod +x "$DEST"
      rm -rf "$TMP"
      echo "  Installed → $DEST"
      ;;

    macos)
      local PANDOC_MACOS_ARCH
      case "$ARCH" in
        x86_64) PANDOC_MACOS_ARCH="x86_64" ;;
        arm64)  PANDOC_MACOS_ARCH="arm64"   ;;
      esac
      local TMP
      TMP="$(mktemp -d)"
      download \
        "https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/pandoc-${PANDOC_VERSION}-${PANDOC_MACOS_ARCH}-macOS.zip" \
        "$TMP/pandoc.zip"
      unzip -q "$TMP/pandoc.zip" -d "$TMP"
      local BIN
      BIN="$(find "$TMP" -name "pandoc" -type f | head -1)"
      [[ -z "$BIN" ]] && { echo "  Error: pandoc binary not found in archive" >&2; exit 1; }
      cp "$BIN" "$DEST"
      chmod +x "$DEST"
      rm -rf "$TMP"
      echo "  Installed → $DEST"
      ;;

    windows)
      echo "  Windows Pandoc: run download-pandoc.ps1 from apps/desktop/scripts/ instead." >&2
      exit 1
      ;;
  esac
}

# ── Run ───────────────────────────────────────────────────────────────────────

install_ffmpeg
install_pandoc

echo ""
echo "All sidecars ready in $BINARIES_DIR"
