# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Role

You are the **Tech Lead** of the Verto project. The CTO is Mathis Aguado — he gives high-level directives, you translate them into concrete tasks and execute or delegate them.

---

## Project overview

**Verto** is a cross-platform desktop application for local file conversion (images, documents, audio, video). No internet connection required. No telemetry.

- **Stack**: Tauri v2 (Rust) + Vue 3 + Vite + Tailwind CSS + pnpm workspaces
- **Landing page**: Vue 3 + Vite → deployed on Vercel
- **External converters**: FFmpeg and Pandoc (bundled as Tauri sidecars)
- **Repo**: https://github.com/mathis-aguado/verto

---

## Development commands

```bash
# Install all dependencies
pnpm install

# Run desktop app in dev mode
pnpm --filter desktop tauri dev

# Run landing page in dev mode
pnpm --filter web dev

# Build desktop app (current platform)
pnpm --filter desktop tauri build

# Lint everything (ESLint + Prettier + Clippy)
pnpm lint

# Format everything
pnpm format
```

### Tests

```bash
# All Vue/TS tests (Vitest)
pnpm --filter desktop test

# Watch mode
pnpm --filter desktop test:watch

# Coverage
pnpm --filter desktop test:coverage

# All Rust tests (--lib skips doctests, évite l'erreur libLLVM système)
cd apps/desktop/src-tauri && cargo test --lib

# Single Rust test
cd apps/desktop/src-tauri && cargo test --lib test_jpeg_to_png
```

---

## Architecture

### Structure apps/desktop

```
apps/desktop/
├── src-tauri/          # Crate Rust (Cargo.toml, build.rs, tauri.conf.json)
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs      # Tauri builder + invoke_handler registration
│   │   ├── commands/   # Tauri commands: image.rs, document.rs, audio.rs, video.rs, fs.rs, updater.rs
│   │   └── converters/ # Wrappers: ffmpeg.rs, pandoc.rs, image.rs
│   └── capabilities/
│       └── default.json
├── ui/                 # Vue 3 frontend
│   ├── index.html
│   └── src/
│       ├── App.vue     # Single-component UI (no components/ or views/ split yet)
│       ├── main.ts
│       ├── i18n/       # vue-i18n: en.json, fr.json, index.ts (Locale = 'en' | 'fr')
│       └── stores/     # Pinia stores: conversion.ts, settings.ts
├── vite.config.ts      # root: 'ui' — pointe Vite vers ui/
├── package.json        # workspace package "desktop"
└── tsconfig*.json
```

### Structure apps/web (landing page)

```
apps/web/
├── src/
│   ├── App.vue     # Single-component landing page
│   ├── main.ts
│   ├── style.css
│   └── i18n/       # vue-i18n: en.json, fr.json, index.ts (même Locale type)
├── public/
├── vite.config.ts
└── package.json    # workspace package "web"
```

### Communication flow (desktop)

```
User (Vue 3 UI)
     │  invoke('convert_image' | 'convert_audio' | 'convert_document' | 'convert_video' | 'list_directory', { ... })
     ▼
Tauri IPC bridge
     ▼
Rust command (src-tauri/src/commands/*.rs)
     ├── Native: image crate (JPEG, PNG, WebP, BMP, TIFF, GIF)
     └── Sidecar: FFmpeg (AVIF, audio, video) / Pandoc (documents)
     ▼
Result<ConversionResult, String> → back to Vue
```

### Tauri commands

| Command | File | Delegates to |
|---|---|---|
| `convert_image` | `commands/image.rs` | `converters/image.rs` or `converters/ffmpeg.rs` (AVIF) |
| `convert_audio` | `commands/audio.rs` | `converters/ffmpeg.rs` |
| `convert_document` | `commands/document.rs` | `converters/pandoc.rs` |
| `convert_video` | `commands/video.rs` | `converters/ffmpeg.rs` |
| `get_video_thumbnail` | `commands/video.rs` | `converters/ffmpeg.rs` → base64 JPEG |
| `list_directory` | `commands/fs.rs` | `std::fs` (max 1000 files) |
| `check_for_updates` | `commands/updater.rs` | `tauri_plugin_updater` |
| `install_update` | `commands/updater.rs` | `tauri_plugin_updater` |

### Conversion strategy

| Format type | Tool |
|---|---|
| JPEG, PNG, WebP, BMP, TIFF, GIF | `image` Rust crate |
| AVIF | FFmpeg sidecar |
| PDF ↔ DOCX, MD ↔ HTML, MD ↔ PDF, RST, ODT, EPUB | Pandoc sidecar |
| Audio: MP3, FLAC, OGG, WAV, AAC | FFmpeg sidecar |
| Video: MP4, MKV, WebM, MOV (H.264, H.265, VP9) | FFmpeg sidecar |

### Pinia stores (desktop)

| Store | Key state |
|---|---|
| `useConversionStore` | `queue` (FileItem[]), `isConverting`, `cancelRequested` — drives the convert-all loop |
| `useSettingsStore` | `outputFormat`, `quality` (1–100), `bitrate` (kbps), `videoCodec` ('h264'/'h265'/'vp9'), `resizeEnabled/Width/Height/keepAspectRatio`, `outputDirectory`, `preserveMetadata`, `overwriteOriginals` |

`FileItem` has fields: `id`, `name`, `path`, `inputFormat`, `inputSize`, `status` (`waiting | converting | done | error`), `category` (`image | document | audio`), `outputPath?`, `outputSize?`, `savedBytes?`, `error?`.

### CI/CD

| Trigger | Workflows |
|---|---|
| Push to branch | `lint.yml` |
| PR → main | `lint.yml` + `build.yml` (Linux, Windows, macOS matrix) |
| Merge → main | `lint.yml` + `build.yml` + `release.yml` (Semantic Release) |

Semantic Release gère intégralement les versions : tag git, CHANGELOG.md, GitHub release, bump de `package.json`. Le dernier tag est `v1.3.0`. La prochaine merge `dev→main` produira `v1.4.0` (présence de commits `feat:`).

---

## Code conventions

### Git & commits

Conventional Commits enforced by Commitlint + Husky:

```
<type>(<scope>): <description>
```

- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `perf`
- Scopes: `desktop`, `web`, `backend`, `frontend`, `ci`, `deps`
- Versioning: `fix:` → patch, `feat:` → minor, `feat!:` / `BREAKING CHANGE:` → major
- **Ne jamais mettre de numéro de version dans le titre du commit** — c'est SR qui calcule la version. Écrire `feat(desktop): add AVIF support` et non `feat(desktop): v0.4.0 — AVIF support`.

### TypeScript / Vue

- Vue 3 Composition API only — `<script setup lang="ts">` syntax
- Typed props with `defineProps<{...}>()`
- No `any` unless justified with a comment
- Component internal order: imports → props/emits → stores → reactive state → computed → functions → lifecycle hooks
- Components in `components/` are kebab-case files, PascalCase in templates
- Views (routes) in `views/`, reusable logic in `composables/use*.ts`

### Design system (Tailwind tokens)

| Token | Value |
|---|---|
| Background | `bg-zinc-900` |
| Surface | `bg-zinc-800` |
| Border | `border-zinc-700` |
| Primary accent | `text-emerald-400` / `bg-emerald-500` |
| Text primary | `text-zinc-100` |
| Text secondary | `text-zinc-400` |

Dark theme by default. Desktop-first responsive. No inline styles, no custom CSS unless strictly necessary.

### Calling Rust commands from Vue

```typescript
import { invoke } from '@tauri-apps/api/core'

const result = await invoke<ConversionResult>('convert_image', {
  inputPath: '/path/to/file.png',
  outputFormat: 'avif',
  quality: 85,
  resizeWidth: 1920,
  resizeHeight: null,   // null = keep aspect ratio
  outputPath: '/path/to/file.avif',
})
```

Always type the return value. Handle errors with try/catch and surface them in the UI.

### Rust

- `cargo clippy` must pass with no warnings before commit
- `cargo fmt` before every commit
- Tauri commands always return `Result<T, String>` (String errors serialize automatically)
- No `unwrap()` in production code — use `?` or explicit error handling
- No blocking calls on the main thread — use `async`
- Validate all input paths (no path traversal); paths must be absolute
- Never delete source files automatically

#### Tauri command pattern

```rust
#[tauri::command]
pub async fn convert_image(
    app: tauri::AppHandle,
    input_path: String,
    output_format: String,
    quality: Option<u8>,
    resize_width: Option<u32>,
    resize_height: Option<u32>,
    output_path: Option<String>,
) -> Result<ConversionResult, String> {
    // validate → convert → map errors to String
}
```

#### Sidecar pattern (FFmpeg / Pandoc)

```rust
let sidecar_cmd = app.shell().sidecar("ffmpeg").map_err(|e| e.to_string())?;
let (mut rx, mut child) = sidecar_cmd
    .args(["-i", &input, "-q:v", "2", &output])
    .spawn()
    .map_err(|e| e.to_string())?;
```

Sidecars declared in `tauri.conf.json` under `bundle.externalBin`.

### Testing

- Rust test fixtures: `apps/desktop/tests/fixtures/` (small sample files per format)
- Vitest tests: `apps/desktop/ui/src/**/__tests__/` or `*.test.ts` alongside source
- Mock `@tauri-apps/api/core` in Vitest: `vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }))`
- Coverage targets: Rust converters ≥ 80%, Tauri commands ≥ 70%, Vue stores ≥ 80%, Vue components ≥ 60%

---

## Sub-agent delegation

| Task type | Agent |
|---|---|
| Vue components, UI, stores, composables | `agents/frontend.md` |
| Rust commands, converters, Tauri config | `agents/backend.md` |
| GitHub Actions, release, packaging, Vercel | `agents/devops.md` |
| README, docs, CHANGELOG, architecture | `agents/docs.md` |
| Vitest, Rust tests, E2E, coverage | `agents/testing.md` |

For cross-domain tasks (e.g. "add WebP conversion with UI"): backend first (Rust command), then frontend (Vue UI).

---

## Definition of done

- [ ] `cargo test --lib` passes (use `--lib` to avoid the libLLVM system error)
- [ ] `pnpm test` passes
- [ ] `pnpm lint` returns no errors
- [ ] Feature works on Linux (primary target), tested manually
- [ ] PR description complete with screenshots if UI changed
- [ ] `CHANGELOG.md` updated under `[Unreleased]`

---

## References

- [docs/roadmap.md](./docs/roadmap.md) — version plan
- [docs/cahier-des-charges.md](./docs/cahier-des-charges.md) — functional specifications
- [docs/architecture.md](./docs/architecture.md) — technical architecture
