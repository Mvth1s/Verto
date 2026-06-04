# Changelog

All notable changes to Verto are documented here.

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
Versioning: [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

## [Unreleased]

### Fixed

* **packaging:** confirmed FFmpeg and Pandoc sidecars isolated to app resource directory via `externalBin` — prevents conflict with system `ffmpeg`/`pandoc` packages on `.deb` install
* **ci:** set `APPIMAGE_EXTRACT_AND_RUN=1` in `build.yml` and `release.yml` so AppImage runs without FUSE on Arch Linux, Fedora 38+ and Ubuntu 24.04+

### Added

* **web:** OS-aware hero download button — detects Linux / Windows / macOS via `navigator.userAgent` and links directly to the matching release asset; falls back to `#download` when the OS is unknown or the release has not loaded yet
* **web:** "See all platforms" secondary button in the hero section, always visible, scrolls to the full download grid
* **web:** complete i18n coverage — all previously hardcoded strings extracted to `en.json` / `fr.json`: privacy section detail texts, download section title and subtitle, OS badge labels, feature chips, and download button labels; download buttons now use `t('download.format', { ext })` interpolation instead of template concatenation
* **web:** new i18n keys — `os.{linux,windows,macos}`, `download.{section_title,section_sub,forOs,generic,allPlatforms,format}`, `privacy.{your_machine,no_upload,cloud_server,detail_1..5_{title,body}}`, `features.{batch,queue,history,local}_chip_{1,2,3}` — added in both `en.json` and `fr.json` with full French translations

* **frontend:** dismissible toast after conversion batch completes, offering to open the output folder in the file manager (auto-dismissed after 8 s)
* **backend:** new `open_output_folder` Tauri command backed by `tauri-plugin-opener`
* **ci:** add `libfuse2` to Linux apt-get step so AppImage is built correctly on ubuntu-22.04
* **docs:** explicit Arch Linux / Manjaro installation instructions with `fuse2` workaround
* **tooling:** `scripts/download-sidecars.sh` — unified curl-based script to populate `binaries/` for FFmpeg and Pandoc; surfaced as `pnpm --filter desktop download-sidecars`
* **frontend:** update check on launch — dismiss is tracked per version (persisted in `localStorage` as `verto.updateDismissedVersion`) so restarting the app doesn't re-show a banner already dismissed for the same version, but a newer release will show it again

# [1.6.0](https://github.com/Mvth1s/Verto/compare/v1.5.1...v1.6.0) (2026-06-02)


### Bug Fixes

* **backend:** allow too_many_arguments on convert_video (Clippy -D warnings) ([0daa233](https://github.com/Mvth1s/Verto/commit/0daa233b3bb3b9b450ea9df2b09cccfee6c2df61))
* **backend:** delete partial output file when FFmpeg fails or is cancelled ([9e8efe2](https://github.com/Mvth1s/Verto/commit/9e8efe20c6ea6c5bf936183864b07d2331dba3da))
* **ci:** add contents:write permission and version sync to release build job ([fafaa16](https://github.com/Mvth1s/Verto/commit/fafaa165aa011a824bea0c7a094bd86bd7b5d36b))
* **ci:** add FFmpeg download to release.yml + fix macOS ARM target ([c0acfd8](https://github.com/Mvth1s/Verto/commit/c0acfd8d936ad7a80bba7809448733014f2c9ebf))
* **ci:** add icon.icns to bundle.icon so macOS bundler skips ICO decoding ([ecd218c](https://github.com/Mvth1s/Verto/commit/ecd218c8edb9f4bfabe54be512fa7ee5ffb15402))
* **ci:** add icon.ico, generate icon.icns on macOS, bump Cargo version ([7fee873](https://github.com/Mvth1s/Verto/commit/7fee8739bdd0144b254dec796d0d3841de735eed))
* **ci:** add Pandoc sidecar download step to release.yml build matrix ([1d65654](https://github.com/Mvth1s/Verto/commit/1d6565483c0aadf0310a5d1c7aea8d0be229119a))
* **ci:** apply rustfmt corrections to Rust sources ([cfd3de9](https://github.com/Mvth1s/Verto/commit/cfd3de9d4cfd2f7b16cbb9eb7b1611481e70ee04))
* **ci:** commit formatted style.css, add libssl-dev and pkg-config to lint-rust ([77ff813](https://github.com/Mvth1s/Verto/commit/77ff813ee7e1bbb08b79a993431c8932083c1fce))
* **ci:** commit minimal icon.icns so bundle.icon resolves on all platforms ([b103247](https://github.com/Mvth1s/Verto/commit/b103247d9f0d9fb32376783333e4102c878b3442))
* **ci:** correct Vitest pnpm passthrough and rustfmt formatting ([599dd90](https://github.com/Mvth1s/Verto/commit/599dd9057a09d7bc817891f0ec844907da4445da))
* **ci:** download-ffmpeg.sh — apt-get primary, BtbN fallback ([47b269a](https://github.com/Mvth1s/Verto/commit/47b269a6f14eaf0a43e513f0fbc9f0459be11ede))
* **ci:** locate pandoc binary via find in macOS zip (structure varies) ([ca8ba43](https://github.com/Mvth1s/Verto/commit/ca8ba437a18d8e7f7ee6e7636ed1501805973ff8))
* **desktop:** regenerate icon PNGs as RGBA — required by tauri::generate_context! ([dedcb59](https://github.com/Mvth1s/Verto/commit/dedcb59d0948df2fefd5a71384aaaee004dc5c79))
* **desktop:** rustfmt — inline vec! args in ffmpeg converter ([3a8870b](https://github.com/Mvth1s/Verto/commit/3a8870bc6b8d8196824a8b0597c147f9e58e245c))
* **frontend:** clicking ✓ on a done queue item removes it ([34d298a](https://github.com/Mvth1s/Verto/commit/34d298a7493832c2138c86f3830f4d40c8328d24))
* **frontend:** extract setLocale() — multi-line Vue template handlers crash compiler ([4882ef4](https://github.com/Mvth1s/Verto/commit/4882ef4e5069e4fb964cb6c3a2fe4435e4d91e24))
* **web:** consistent download buttons with fallback labels, add localStorage cache ([8423559](https://github.com/Mvth1s/Verto/commit/8423559a1430b98f7e9eaa9bd20324f937b97a7f))


### Features

* **desktop:** add file picker, browse, cancel conversion, retry on error ([a4bebc1](https://github.com/Mvth1s/Verto/commit/a4bebc16e76a93438444029e12fea6e68c939ec4))
* **desktop:** add HEIC/HEIF input support via FFmpeg ([59a7b38](https://github.com/Mvth1s/Verto/commit/59a7b388e520938f3148cc630490185742a0f255))
* **desktop:** add settings page with persistence ([8029b24](https://github.com/Mvth1s/Verto/commit/8029b24b5a149cc64f09edc28610479aa4af9500))
* **desktop:** add video conversion (MP4, MKV, WebM, MOV) ([381f741](https://github.com/Mvth1s/Verto/commit/381f7419d8925dcaee6a6783844348819ba7c114))
* **desktop:** add video thumbnail preview in queue ([2ed9b70](https://github.com/Mvth1s/Verto/commit/2ed9b7005ae78c1824664e47280764ee1ca01464))
* **desktop:** cancel active FFmpeg conversion immediately ([24cc933](https://github.com/Mvth1s/Verto/commit/24cc933b39f6ff24e52b26175afb66bc7cce7686))
* **desktop:** integrate Verto logo — icons, landing page, desktop app ([99a8b34](https://github.com/Mvth1s/Verto/commit/99a8b34eba6f3509b5bcfab319a2eece7388a356))
* **desktop:** pandoc sidecar, document conversion, batch via folders ([c725764](https://github.com/Mvth1s/Verto/commit/c725764f9878d649c2f96a72e823b58c2c5c473b))
* **desktop:** real-time FFmpeg progress bar for audio and video ([3712204](https://github.com/Mvth1s/Verto/commit/3712204ef430204d1b539433aee2797fdf9f6d9e))
* **desktop:** show system notification when conversion completes ([f39f6d1](https://github.com/Mvth1s/Verto/commit/f39f6d152e81d7bb7576fd6b464ba3281259ec44))
* **desktop:** v0.3.0 — audio conversion via FFmpeg sidecar ([e72a7ee](https://github.com/Mvth1s/Verto/commit/e72a7ee92cd33461bd5e817ca551678faac30b9f))
* **desktop:** v0.4.0 — AVIF, image resize, quality presets ([5c71273](https://github.com/Mvth1s/Verto/commit/5c712732a62bbb3d3b4db2c814d785241e647a32))
* **desktop:** v0.4.0 — image thumbnail preview in queue ([60329b4](https://github.com/Mvth1s/Verto/commit/60329b4517b0094ace5d69db6b5a7a1c6e6bb0e6))
* **desktop:** v1.0.0 — i18n, accessibilité, auto-updater, coverage, SEO ([c868c6b](https://github.com/Mvth1s/Verto/commit/c868c6bb61df6498ed53a38c0ae3887775f5fb45))
* **web:** add favicon, apple-touch-icon and logo in hero mockup ([1f0dea4](https://github.com/Mvth1s/Verto/commit/1f0dea44345d88ee7de2100f8ba330008003cb9e))
* **web:** dynamic download links from GitHub API, update README ([20fde27](https://github.com/Mvth1s/Verto/commit/20fde2717973bee3a3fe675c8c968c0f996b95f7))
* **web:** i18n EN/FR + restore CSS mockup in hero ([bc2b92b](https://github.com/Mvth1s/Verto/commit/bc2b92b2325f632aea0fc18d3517ea8441666603))
* **web:** polish landing page — nav, features grid, privacy details, CHANGELOG ([0c4f277](https://github.com/Mvth1s/Verto/commit/0c4f277e752179c2eaceb8f24949779e90c91516))

## [1.5.1](https://github.com/Mvth1s/Verto/compare/v1.5.0...v1.5.1) (2026-06-01)


### Bug Fixes

* **ci:** drop macOS x86_64 target, revert scripts to original simplicity ([852ff52](https://github.com/Mvth1s/Verto/commit/852ff52cd6733bd9d548b02eb5e8e7f7a2baa6f5))
* **ci:** remove invalid --no-deps flag from brew fetch + robust bottle path detection ([f3ec3e3](https://github.com/Mvth1s/Verto/commit/f3ec3e31c28256a0d207faca5f426c966189562e))
* **ci:** replace retired macos-13 runner with macos-latest + cross-compile x86_64 ([e596d5a](https://github.com/Mvth1s/Verto/commit/e596d5a27834ec430c9dd9b962be755a55bb13d3))
* **ci:** use brew fetch --bottle-tag to get Intel FFmpeg on ARM runner ([f4a2cec](https://github.com/Mvth1s/Verto/commit/f4a2cec5e474dfd78a54b4aad8b560189708a294))

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
