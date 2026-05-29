# Changelog

All notable changes to Verto are documented here.

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
Versioning: [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

> **Note:** Semantic Release manages version tags from `v1.x` onward.
> The `[0.1.0]` and `[0.2.0]` entries reflect feature milestones reached before automated tagging was configured.

---

## [Unreleased]

---

## [1.1.0] - 2026-05-29

### Added
- Verto logo integrated across all surfaces (app sidebar, landing page nav, hero mockup)
- App icons regenerated (RGBA PNG, ICO, ICNS) from the real logo
- Favicon and apple-touch-icon for the landing page
- Husky v9 git hooks: `commit-msg` (commitlint), `pre-commit` (ESLint), `pre-push` (lint + format check)
- Commitlint with `@commitlint/config-conventional` — enforces Conventional Commits on every commit
- Semantic Release configured via `.releaserc.json` (branch: main, no npm publish)
- Landing page: dynamic download links fetched from GitHub API with localStorage cache (1h TTL)
- Landing page: Privacy section anchor, expanded features grid (6 cards), corrected format lists

### Fixed
- `release.yml` `build-and-upload` job missing `permissions: contents: write` — uploads were failing
- Version sync: `tauri.conf.json` now patched with Semantic Release version before build (artifact names match the release tag)
- macOS Pandoc sidecar: `download-pandoc.sh` uses `find` to locate binary regardless of zip structure
- Windows build: `icons/icon.ico` generated and committed (required by `tauri-winres` at compile time)
- macOS bundle: `icon.icns` generated on CI via `sips`/`iconutil`; placeholder committed to repo
- Icon PNGs regenerated as RGBA — `tauri::generate_context!()` panicked on RGB-only icons

---

## [0.2.0] - 2026-05-29

### Added
- File picker on dropzone click via `@tauri-apps/plugin-dialog`
- Browse button to select output folder via folder picker dialog
- Cancel button to stop the conversion queue mid-run
- Retry action on failed queue items (click the refresh icon to re-queue)
- Document conversion via Pandoc sidecar: MD, DOCX, HTML, RST, ODT, EPUB ↔ all supported formats
- Batch conversion: drop a folder to recursively enqueue all supported files
- Documents category in sidebar — format selector and queue filtered by active category
- Tauri commands `convert_document` and `list_directory`
- Pandoc sidecar download scripts for Linux/macOS (`download-pandoc.sh`) and Windows (`download-pandoc.ps1`)
- Build matrix (Linux × Windows × macOS) downloads Pandoc sidecar before building
- 30 Vitest unit tests for `useConversionStore` and `useSettingsStore`

---

## [0.1.0] - 2026-05-28

### Added
- Monorepo pnpm workspaces (`apps/desktop`, `apps/web`)
- Desktop app skeleton with Tauri v2 + Vue 3 + Vite
- Image conversion: JPEG, PNG, WebP, BMP, TIFF, GIF via `image` crate
- Tauri command `convert_image` with async `spawn_blocking`, quality control for JPEG
- Drag-and-drop file queue with per-file status (waiting / converting / done / error)
- Output format selector and quality slider (auto-disabled for lossless formats)
- Pinia stores: `useConversionStore` (queue, progress) and `useSettingsStore` (preferences)
- Landing page: hero, features, privacy section, download section, footer
- GitHub Actions: `lint.yml` (ESLint + Prettier + Clippy + rustfmt) triggered on every push
- GitHub Actions: `build.yml` (Linux × Windows × macOS matrix) on PRs to main
- ESLint 9 flat config + Prettier for Vue/TypeScript workspaces
