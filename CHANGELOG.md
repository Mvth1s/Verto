# Changelog

All notable changes to Verto are documented here.

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
Versioning: [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

> **Note:** From `v1.4.0` onward, this file is maintained automatically by Semantic Release.
> Entries below `v1.1.0` reflect pre-automation milestones written manually.

<!-- SEMANTIC RELEASE WILL PREPEND NEW VERSIONS ABOVE THIS LINE -->

---

## [1.3.0] - 2026-05-30

### Added
- Audio conversion: MP3, FLAC, OGG, WAV, AAC via FFmpeg sidecar
- Audio category in sidebar with bitrate selector
- FFmpeg sidecar download scripts (Linux/macOS/Windows)

### Fixed
- CI: download-ffmpeg.sh — apt-get primary, BtbN fallback
- Frontend: clicking ✓ on a done queue item removes it

---

## [1.2.0] - 2026-05-29

### Added
- Verto logo across app sidebar, landing page nav, hero
- App icons regenerated (RGBA PNG, ICO, ICNS)
- Husky v9 + commitlint (Conventional Commits enforced)
- Semantic Release configured (branch: main, no npm publish)
- Landing page: dynamic download links from GitHub API (1h cache)
- Landing page: Privacy section, expanded features grid

### Fixed
- `release.yml`: missing `permissions: contents: write` on upload job
- Version sync: `tauri.conf.json` patched with SR version before build
- macOS Pandoc sidecar: `find`-based binary detection
- Windows build: `icons/icon.ico` generated for `tauri-winres`
- Icons PNGs regenerated as RGBA to avoid Tauri panic

---

## [1.1.0] - 2026-05-29

### Added
- Document conversion via Pandoc sidecar (MD, DOCX, HTML, RST, ODT, EPUB)
- Batch conversion: drop a folder to enqueue all supported files recursively
- File picker on dropzone click, folder picker for output directory
- Cancel and retry buttons on the conversion queue
- Tauri commands `convert_document` and `list_directory`
- Build matrix Linux × Windows × macOS in CI
- 30 Vitest unit tests for stores

---

## [1.0.0] - 2026-05-28

### Added
- Monorepo pnpm workspaces (`apps/desktop`, `apps/web`)
- Desktop app with Tauri v2 + Vue 3 + Vite
- Image conversion: JPEG, PNG, WebP, BMP, TIFF, GIF via `image` crate
- Drag-and-drop file queue with per-file status
- Output format selector and quality slider
- Pinia stores: `useConversionStore`, `useSettingsStore`
- Landing page: hero, features, privacy, download, footer
- GitHub Actions: `lint.yml` + `build.yml` (Linux × Windows × macOS)
