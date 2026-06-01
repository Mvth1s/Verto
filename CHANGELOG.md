# Changelog

All notable changes to Verto are documented here.

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
Versioning: [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

# [1.5.0](https://github.com/Mvth1s/Verto/compare/v1.4.0...v1.5.0) (2026-06-01)


### Bug Fixes

* **ci:** add FFmpeg download to release.yml + fix macOS ARM target ([5c45b34](https://github.com/Mvth1s/Verto/commit/5c45b343aebb604a72976ba8719299a57d50fed4))


### Features

* **desktop:** show system notification when conversion completes ([d86fe1b](https://github.com/Mvth1s/Verto/commit/d86fe1b2cf96d3660633b10f268d0f8c4e9e7649))

# [1.4.0](https://github.com/Mvth1s/Verto/compare/v1.3.0...v1.4.0) (2026-06-01)


### Bug Fixes

* **frontend:** extract setLocale() — multi-line Vue template handlers crash compiler ([29220f8](https://github.com/Mvth1s/Verto/commit/29220f84f93e05bf42c7e88c41d57c338897807b))


### Features

* **desktop:** add settings page with persistence ([cc7111c](https://github.com/Mvth1s/Verto/commit/cc7111c823be3db13e29ed564815ffc14f01cb1a))
* **desktop:** add video conversion (MP4, MKV, WebM, MOV) ([927b57b](https://github.com/Mvth1s/Verto/commit/927b57ba0de991141bddb20d42a5f5a8ae509cc1))
* **desktop:** add video thumbnail preview in queue ([6ce7332](https://github.com/Mvth1s/Verto/commit/6ce7332915f9a797af1d76e0600357d97db75520))
* **desktop:** v0.4.0 — AVIF, image resize, quality presets ([6096147](https://github.com/Mvth1s/Verto/commit/60961472ceea50e6619d6dd4e38593a4c9d800a9))
* **desktop:** v0.4.0 — image thumbnail preview in queue ([828d32b](https://github.com/Mvth1s/Verto/commit/828d32b48583984a870d4f4b1e00d2ca2af3e613))
* **desktop:** v1.0.0 — i18n, accessibilité, auto-updater, coverage, SEO ([2be5970](https://github.com/Mvth1s/Verto/commit/2be5970a85f419f8b3487dcbdedd5be06b408e8c))

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
