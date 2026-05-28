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

# All Rust tests
cd apps/desktop && cargo test

# Single Rust test
cd apps/desktop && cargo test test_jpeg_to_png_conversion
```

---

## Architecture

### Communication flow (desktop)

```
User (Vue 3 UI)
     │  invoke('convert_image', { ... })
     ▼
Tauri IPC bridge
     ▼
Rust command (src/commands/image.rs)
     ├── Native: image crate (JPEG, PNG, WebP, BMP, TIFF, GIF)
     └── Sidecar: FFmpeg (AVIF, HEIF, audio, video) / Pandoc (documents)
     ▼
Result<ConversionResult, String> → back to Vue
```

### Conversion strategy

| Format type | Tool |
|---|---|
| JPEG, PNG, WebP, BMP, TIFF, GIF | `image` Rust crate |
| AVIF, HEIF | FFmpeg sidecar |
| PDF ↔ DOCX, MD ↔ HTML, MD ↔ PDF | Pandoc sidecar |
| Audio (v0.3+) | FFmpeg sidecar |
| Video (v1.1+) | FFmpeg sidecar |

### Pinia stores (desktop)

| Store | State |
|---|---|
| `useConversionStore` | queue, progress per file, history |
| `useSettingsStore` | default output format, output dir, theme |

### CI/CD

| Trigger | Workflows |
|---|---|
| Push to branch | `lint.yml` |
| PR → main | `lint.yml` + `build.yml` (Linux, Windows, macOS matrix) |
| Merge → main | `lint.yml` + `build.yml` + `release.yml` (Semantic Release) |

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

### TypeScript / Vue

- Vue 3 Composition API only — `<script setup lang="ts">` syntax
- Typed props with `defineProps<{...}>()`
- No `any` unless justified with a comment
- Component internal order: imports → props/emits → stores → reactive state → computed → functions → lifecycle hooks
- Components in `components/` are kebab-case files, PascalCase in templates
- Views (routes) in `views/`, reusable logic in `composables/use*.ts`
- All user-facing text goes through i18n keys

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
  outputFormat: 'webp',
  quality: 85,
})
```

Always type the return value. Handle errors with try/catch and surface them in the UI.

### Rust

- `cargo clippy` must pass with no warnings before commit
- `cargo fmt` before every commit
- Tauri commands always return `Result<T, String>` (String errors serialize automatically)
- No `unwrap()` in production code — use `?` or explicit error handling
- No blocking calls on the main thread — use `async`
- Validate all input paths (no path traversal)
- Never delete source files automatically

#### Tauri command pattern

```rust
#[tauri::command]
pub async fn convert_image(
    input_path: String,
    output_format: String,
    quality: Option<u8>,
    output_path: Option<String>,
) -> Result<ConversionResult, String> {
    // validate → convert → map errors to String
}
```

#### Sidecar pattern (FFmpeg / Pandoc)

```rust
let sidecar_command = app.shell().sidecar("ffmpeg").unwrap();
let (mut rx, mut child) = sidecar_command
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

- [ ] `cargo test` passes
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
