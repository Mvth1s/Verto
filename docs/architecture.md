# Architecture

## Overview

Verto is a monorepo containing two applications:

- **`apps/desktop`** — the Tauri v2 desktop app
- **`apps/web`** — the landing page (Vite + Vue 3 → Vercel)

---

## Desktop app

### Communication flow

```
User (Vue 3 UI)
     │
     │  invoke('convert_image' | 'convert_audio' | 'convert_document' | 'convert_video' | ...)
     ▼
Tauri IPC bridge
     │
     ▼
Rust command (src/commands/*.rs)
     │
     ├── Native: image crate (basic image formats, no deps)
     └── Sidecar: FFmpeg / Pandoc (advanced formats)
     │
     ▼
Result<ConversionResult, String> → back to Vue
```

### Why Tauri v2?

| Criterion | Tauri v2 | Electron |
|-----------|----------|---------|
| Installer size | ~15 MB | ~150 MB |
| Memory usage | Low (Rust core) | High (Node.js) |
| Performance | Native | Good |
| Security model | Strong (capability system) | Weaker |
| Backend language | Rust | Node.js |

### Conversion strategy

| Format type | Tool | Reason |
|-------------|------|--------|
| JPEG, PNG, WebP, BMP, TIFF, GIF | `image` Rust crate | Fast, no sidecar needed |
| AVIF | FFmpeg sidecar | Complex codec, library too large to compile |
| PDF ↔ DOCX, MD ↔ HTML, MD ↔ PDF, RST, ODT, EPUB | Pandoc sidecar | Industry standard, comprehensive |
| Audio: MP3, FLAC, OGG, WAV, AAC | FFmpeg sidecar | Universal |
| Video: MP4, MKV, WebM, MOV (H.264, H.265, VP9) | FFmpeg sidecar | Universal |

### Tauri commands

| Command | File | Description |
|---------|------|-------------|
| `convert_image` | `commands/image.rs` | Image conversion + optional resize |
| `convert_audio` | `commands/audio.rs` | Audio conversion with bitrate |
| `convert_document` | `commands/document.rs` | Document conversion via Pandoc |
| `convert_video` | `commands/video.rs` | Video conversion with codec/resolution |
| `get_video_thumbnail` | `commands/video.rs` | Returns base64 JPEG thumbnail via FFmpeg |
| `list_directory` | `commands/fs.rs` | Recursively lists files (max 1000) |
| `check_for_updates` | `commands/updater.rs` | Checks for a new release via `tauri_plugin_updater` |
| `install_update` | `commands/updater.rs` | Downloads and installs the update |

### Sidecar bundling

FFmpeg and Pandoc binaries are bundled as Tauri sidecars: included in the installer, extracted to a temp/app directory on first launch. The user has no system dependency to install.

Binary sizes (approximate, stripped):
- FFmpeg: ~50 MB (per platform)
- Pandoc: ~25 MB (per platform)

---

## Landing page

Vite + Vue 3 static site deployed on Vercel. No SSR. Automatic preview deployments per branch.

Sections: Nav (with EN/FR language toggle) → Hero (CSS mockup) → Features → Privacy promise → Download → Footer

Dynamic download links are fetched from the GitHub Releases API and cached in `localStorage` for 1 hour.

---

## Monorepo setup

pnpm workspaces. All scripts are run from root:

```bash
pnpm --filter desktop <script>   # desktop app
pnpm --filter web <script>       # landing page
```

---

## i18n

Both apps use `vue-i18n` with `legacy: false` (Composition API mode).

- Locale type: `'en' | 'fr'` (default: `'en'`)
- Message files: `src/i18n/en.json` and `src/i18n/fr.json` in each app
- The desktop app persists locale selection across sessions

---

## State management (desktop app)

Pinia stores:

| Store | Key state |
|-------|-----------|
| `useConversionStore` | `queue` (FileItem[]), `isConverting`, `cancelRequested` |
| `useSettingsStore` | `outputFormat`, `quality`, `bitrate`, `videoCodec`, `resizeEnabled/Width/Height/keepAspectRatio`, `outputDirectory`, `preserveMetadata`, `overwriteOriginals` |

`FileItem`: `id`, `name`, `path`, `inputFormat`, `inputSize`, `status` (`waiting | converting | done | error`), `category` (`image | document | audio | video`), `outputPath?`, `outputSize?`, `savedBytes?`, `error?`

Settings are persisted via `localStorage` (keys prefixed with `verto.*`). Format and resize state are not persisted (reset on launch).

---

## CI/CD

```
Push to branch
  └── lint.yml (ESLint + Clippy)

PR to main
  ├── lint.yml
  └── build.yml (matrix: Linux, Windows, macOS ARM)

Merge to main
  ├── lint.yml
  ├── build.yml
  └── release.yml (Semantic Release → tag → GitHub Release with artifacts)
```

Semantic Release manages versioning entirely: git tag, CHANGELOG.md, GitHub release, `package.json` bump.
