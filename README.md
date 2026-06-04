<div align="center">

# Verto

**Convert anything. Locally.**

Simple, fast, open-source file conversion for Linux, Windows and macOS — no internet, no account, no telemetry.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-lightgrey)](#installation)
[![Release](https://img.shields.io/github/v/release/Mvth1s/Verto)](https://github.com/Mvth1s/Verto/releases)
[![Build](https://github.com/Mvth1s/Verto/actions/workflows/build.yml/badge.svg)](https://github.com/Mvth1s/Verto/actions)

</div>

---

## What is Verto?

Verto is a desktop application that converts files — images, documents, audio and video — entirely on your machine. No upload, no cloud, no subscription. Just drag, drop, convert.

It was born out of a simple frustration: never remembering the right `ffmpeg` or `pandoc` command for a one-off file conversion.

## Supported conversions

| Category  | Formats                                                        | Engine              |
|-----------|----------------------------------------------------------------|---------------------|
| Images    | JPEG, PNG, WebP, GIF, BMP, TIFF, AVIF                         | `image` crate + FFmpeg sidecar |
| Documents | MD, DOCX, HTML, RST, ODT, EPUB, PDF                           | Pandoc sidecar      |
| Audio     | MP3, FLAC, OGG, WAV, AAC                                      | FFmpeg sidecar      |
| Video     | MP4, MKV, WebM, MOV (H.264, H.265, VP9)                       | FFmpeg sidecar      |

## Features

- Drag & drop files or folders (batch conversion)
- File picker for individual files, folder browser for output directory
- Cancel in-progress queue / retry failed items
- Real-time progress bar for audio and video conversions
- Images, Documents, Audio and Video categories — separate queues and format selectors
- Quality slider (images), bitrate selector (audio), codec selector (video)
- Image resize (width × height, keep aspect ratio) and quality presets (Web / Print / Lossless)
- Thumbnail preview for images and videos in the queue
- Settings page with persistence (output format, directory, quality, bitrate, codec)
- System notification when a batch conversion completes
- i18n: English and French (toggle in the UI)
- Auto-updater (checks for new releases on launch)
- 100% local — files never leave your machine
- Cross-platform: Linux, Windows, macOS

## Installation

### One-liner (Linux & macOS)

```bash
curl -sSL https://raw.githubusercontent.com/Mvth1s/Verto/main/scripts/install.sh | bash
```

Linux installs the AppImage to `~/.local/bin/verto`. macOS copies `Verto.app` to `/Applications`.

---

Or download the package directly from [github.com/Mvth1s/Verto/releases](https://github.com/Mvth1s/Verto/releases):

### Linux — Debian / Ubuntu

```bash
sudo dpkg -i Verto_*_amd64.deb
```

### Linux — Arch-based (Arch, Manjaro, EndeavourOS…)

The recommended format is AppImage — it has no system dependencies.

```bash
chmod +x Verto_*.AppImage
./Verto_*.AppImage
```

If you get a FUSE error on first launch, you have two options:

**Option A — install the FUSE 2 library:**
```bash
# Arch / Manjaro
sudo pacman -S fuse2

# Then re-run
./Verto_*.AppImage
```

**Option B — run without FUSE (no install required):**
```bash
./Verto_*.AppImage --appimage-extract-and-run
```

An AUR package (`verto-bin`) is planned for a future release.

### Linux — Fedora / RHEL

```bash
sudo rpm -i Verto-*.x86_64.rpm
```

### Linux — AppImage (all distros)

```bash
chmod +x Verto_*.AppImage && ./Verto_*.AppImage
```

### Windows

Run `Verto_*_x64-setup.exe` (NSIS installer) or `Verto_*_x64_en-US.msi`.

### macOS

Open `Verto_*_aarch64.dmg` and drag Verto to your Applications folder.

<details>
<summary>Advanced: download sidecars manually via curl</summary>

If you prefer to download FFmpeg and Pandoc yourself rather than using the bundled binaries (useful for development without running the full CI setup):

```bash
pnpm --filter desktop download-sidecars
```

This requires `curl` and `bash`. The script auto-detects your platform and places the binaries in `apps/desktop/src-tauri/binaries/` with the correct Tauri sidecar filenames. Re-running it is safe — existing binaries are skipped. The bundled sidecar approach used by CI and releases is unaffected.

</details>

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [Node.js](https://nodejs.org/) ≥ 20
- [pnpm](https://pnpm.io/) ≥ 9
- [Tauri prerequisites](https://tauri.app/start/prerequisites/) for your platform

### Setup

```bash
git clone https://github.com/Mvth1s/Verto.git
cd Verto
pnpm install

# Download sidecars required for conversion (FFmpeg + Pandoc)
bash apps/desktop/scripts/download-ffmpeg.sh
bash apps/desktop/scripts/download-pandoc.sh

# Run the desktop app in dev mode
pnpm --filter desktop tauri dev
```

### Commands

```bash
pnpm --filter desktop tauri dev      # Desktop app (hot-reload)
pnpm --filter web dev                # Landing page
pnpm --filter desktop test           # Vitest unit tests
pnpm lint                            # ESLint + Clippy
pnpm format                          # Prettier + rustfmt
cd apps/desktop/src-tauri && cargo test --lib  # Rust unit tests
```

### Project structure

```
verto/
├── apps/
│   ├── desktop/              # Tauri v2 app
│   │   ├── src-tauri/        # Rust backend (commands, converters, sidecars)
│   │   └── ui/               # Vue 3 frontend
│   └── web/                  # Landing page (Vue 3 + Vite → Vercel)
├── assets/                   # Source assets (logo, etc.)
├── agents/                   # Claude Code sub-agent definitions
├── docs/                     # Architecture, roadmap, specs
└── .github/                  # CI/CD: lint, build matrix, Semantic Release
```

See [docs/architecture.md](./docs/architecture.md) for the full technical overview.

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](./CONTRIBUTING.md) before opening a PR.

Commit messages must follow [Conventional Commits](https://www.conventionalcommits.org/) — enforced by Commitlint + Husky.

## License

[MIT](./LICENSE) — Mathis Aguado, 2026
