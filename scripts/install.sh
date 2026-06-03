#!/usr/bin/env bash
# Installs the latest Verto release for the current platform.
#
# Usage:
#   curl -sSL https://raw.githubusercontent.com/Mvth1s/Verto/main/scripts/install.sh | bash
#
# Linux  → AppImage placed in ~/.local/bin/verto
# macOS  → Verto.app copied to /Applications

set -euo pipefail

REPO="Mvth1s/Verto"
API_URL="https://api.github.com/repos/${REPO}/releases/latest"

# ── Helpers ───────────────────────────────────────────────────────────────────

check_dep() {
  if ! command -v "$1" &>/dev/null; then
    echo "Error: '$1' is required but not installed." >&2
    exit 1
  fi
}

asset_url() {
  local pattern="$1"
  echo "$RELEASE_JSON" \
    | grep -o '"browser_download_url": *"[^"]*'"$pattern"'[^"]*"' \
    | grep -o 'https://[^"]*' \
    | head -1
}

# ── Fetch release info ────────────────────────────────────────────────────────

check_dep curl

echo "Fetching latest Verto release…"
RELEASE_JSON=$(curl -sSfL "$API_URL")
VERSION=$(echo "$RELEASE_JSON" \
  | grep -o '"tag_name": *"[^"]*"' \
  | grep -o 'v[0-9][^"]*')

if [[ -z "$VERSION" ]]; then
  echo "Error: could not determine latest version. GitHub API may be rate-limited." >&2
  exit 1
fi

echo "Latest version: $VERSION"

# ── Platform dispatch ─────────────────────────────────────────────────────────

OS="$(uname -s)"

case "$OS" in

  # ── Linux ──────────────────────────────────────────────────────────────────
  Linux)
    ARCH="$(uname -m)"
    case "$ARCH" in
      x86_64)  EXT_PAT="amd64.AppImage" ;;
      aarch64) EXT_PAT="aarch64.AppImage" ;;
      *)
        echo "Error: unsupported architecture '$ARCH'." >&2
        exit 1
        ;;
    esac

    URL=$(asset_url "$EXT_PAT")
    if [[ -z "$URL" ]]; then
      echo "Error: no AppImage found for $ARCH in release $VERSION." >&2
      exit 1
    fi

    INSTALL_DIR="$HOME/.local/bin"
    DEST="$INSTALL_DIR/verto"
    mkdir -p "$INSTALL_DIR"

    echo "Downloading $URL…"
    curl -L --progress-bar "$URL" -o "$DEST"
    chmod +x "$DEST"

    echo ""
    echo "✓ Verto $VERSION installed → $DEST"

    # Warn if ~/.local/bin is not in PATH
    case ":${PATH}:" in
      *":$INSTALL_DIR:"*) ;;
      *)
        echo ""
        echo "Note: $INSTALL_DIR is not in your PATH."
        echo "Add this line to your ~/.bashrc or ~/.zshrc:"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
        ;;
    esac
    ;;

  # ── macOS ──────────────────────────────────────────────────────────────────
  Darwin)
    check_dep hdiutil

    ARCH="$(uname -m)"
    case "$ARCH" in
      arm64)  EXT_PAT="aarch64.dmg" ;;
      x86_64) EXT_PAT="x86_64.dmg"  ;;
      *)
        echo "Error: unsupported architecture '$ARCH'." >&2
        exit 1
        ;;
    esac

    URL=$(asset_url "$EXT_PAT")
    if [[ -z "$URL" ]]; then
      echo "Error: no DMG found for $ARCH in release $VERSION." >&2
      exit 1
    fi

    TMP=$(mktemp -d)
    trap 'rm -rf "$TMP"' EXIT

    echo "Downloading $URL…"
    curl -L --progress-bar "$URL" -o "$TMP/Verto.dmg"

    echo "Installing to /Applications…"
    hdiutil attach "$TMP/Verto.dmg" -quiet -nobrowse -mountpoint "$TMP/mnt"
    cp -r "$TMP/mnt/Verto.app" /Applications/
    hdiutil detach "$TMP/mnt" -quiet

    echo ""
    echo "✓ Verto $VERSION installed → /Applications/Verto.app"
    ;;

  # ── Windows / other ────────────────────────────────────────────────────────
  *)
    echo "This script supports Linux and macOS only."
    echo "Windows: download the installer from https://github.com/${REPO}/releases/latest"
    exit 1
    ;;

esac
