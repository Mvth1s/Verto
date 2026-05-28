# Architecture

## Overview

Verto is a monorepo containing two applications sharing a common component package:

- **`apps/desktop`** — the Tauri v2 desktop app
- **`apps/web`** — the landing page (Vite + Vue 3 → Vercel)
- **`packages/ui`** — shared Vue components (optional, added when needed)

---

## Desktop app

### Communication flow

```
User (Vue 3 UI)
     │
     │  invoke('convert_image', { ... })
     ▼
Tauri IPC bridge
     │
     ▼
Rust command (src/commands/image.rs)
     │
     ├── Native: image crate (basic formats, no deps)
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
| AVIF, HEIF | FFmpeg sidecar | Complex codec, library too large to compile |
| PDF ↔ DOCX, MD ↔ HTML, MD ↔ PDF | Pandoc sidecar | Industry standard, comprehensive |
| Audio (v0.3+) | FFmpeg sidecar | Universal |
| Video (v1.1+) | FFmpeg sidecar | Universal |

### Sidecar bundling

FFmpeg and Pandoc binaries are bundled as Tauri sidecars: included in the installer, extracted to a temp/app directory on first launch. The user has no system dependency to install.

Binary sizes (approximate, stripped):
- FFmpeg: ~50 MB (per platform)
- Pandoc: ~25 MB (per platform)

These are included only in the relevant install targets. A "lite" build without sidecars is planned for users who have FFmpeg/Pandoc system-wide.

---

## Landing page

Simple Vite + Vue 3 static site. No SSR needed initially. Deployed on Vercel with automatic preview deployments per branch.

Sections: Hero → Features → Privacy promise → Download → Footer

---

## Monorepo setup

pnpm workspaces. All scripts are run from root:

```bash
pnpm --filter desktop <script>   # desktop app
pnpm --filter web <script>       # landing page
pnpm --filter ui <script>        # shared components
```

---

## State management (desktop app)

Pinia stores:

| Store | State |
|-------|-------|
| `useConversionStore` | queue, progress per file, history |
| `useSettingsStore` | default output format, output dir, theme |

---

## CI/CD

```
Push to branch
  └── lint.yml (ESLint + Clippy)

PR to main
  ├── lint.yml
  └── build.yml (matrix: Linux, Windows, macOS)

Merge to main
  ├── lint.yml
  ├── build.yml
  └── release.yml (Semantic Release → tag → GitHub Release with artifacts)
```
