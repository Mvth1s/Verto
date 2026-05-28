<div align="center">

# Verto

**Convert anything. Locally.**

Simple, fast, open-source file conversion for Linux, Windows and macOS — no internet, no account, no telemetry.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-lightgrey)](#installation)
[![Release](https://img.shields.io/github/v/release/mathis-aguado/verto)](https://github.com/mathis-aguado/verto/releases)
[![Build](https://github.com/mathis-aguado/verto/actions/workflows/build.yml/badge.svg)](https://github.com/mathis-aguado/verto/actions)

</div>

---

## What is Verto?

Verto is a desktop application that converts files — images, documents, audio, video — entirely on your machine. No upload, no cloud, no subscription. Just drag, drop, convert.

It was born out of a simple frustration: never remembering the right `ffmpeg` or `convert` command for a one-off file conversion.

## Supported conversions

| Category  | Formats                                           |
|-----------|---------------------------------------------------|
| Images    | JPEG, PNG, WebP, AVIF, GIF, BMP, TIFF             |
| Documents | PDF, DOCX, Markdown, HTML, ODT                    |
| Audio     | MP3, FLAC, OGG, WAV, AAC *(roadmap v0.3)*        |
| Video     | MP4, MKV, WebM, MOV *(roadmap v1.1)*             |

## Features

- 🖱️ Drag & drop or file picker
- 📦 Batch conversion
- 🔒 100% local — files never leave your machine
- ⚙️ Per-format options (quality, resolution, compression)
- 🌍 Cross-platform: Linux, Windows, macOS
- 🪶 Lightweight (~15 MB installer)

## Installation

### Linux

```bash
# AppImage
chmod +x Verto_x.x.x_amd64.AppImage && ./Verto_x.x.x_amd64.AppImage

# .deb
sudo dpkg -i verto_x.x.x_amd64.deb
```

### Windows

Download and run `Verto_x.x.x_x64-setup.exe` from the [releases page](https://github.com/mathis-aguado/verto/releases).

### macOS

Download and open `Verto_x.x.x_x64.dmg` from the [releases page](https://github.com/mathis-aguado/verto/releases).

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) ≥ 20
- [pnpm](https://pnpm.io/) ≥ 9
- [Tauri CLI](https://tauri.app/start/prerequisites/)
- FFmpeg (system or bundled via sidecar)
- Pandoc (system or bundled via sidecar)

### Setup

```bash
git clone https://github.com/mathis-aguado/verto.git
cd verto
pnpm install
pnpm tauri dev
```

### Project structure

```
verto/
├── apps/
│   ├── desktop/        # Tauri app (Rust backend + Vue 3 frontend)
│   └── web/            # Landing page (Vue 3 + Vite → Vercel)
├── packages/
│   └── ui/             # Shared Vue components
├── agents/             # Claude Code sub-agent definitions
├── docs/               # Technical documentation
└── .github/            # CI/CD workflows, issue templates
```

See [docs/architecture.md](./docs/architecture.md) for the full technical overview.

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](./CONTRIBUTING.md) before opening a PR.

## License

[MIT](./LICENSE) — Mathis Aguado, 2025
