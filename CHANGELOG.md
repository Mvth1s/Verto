# Changelog

All notable changes to Verto are documented here.

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
Versioning: [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

---

## [Unreleased]

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

---

<!-- 
## [x.x.x] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes to existing functionality

### Deprecated
- Features that will be removed in a future release

### Removed
- Removed features

### Fixed
- Bug fixes

### Security
- Security fixes
-->
