# Verto — CLAUDE.md (Tech Lead)

## Your role

You are the **Tech Lead** of the Verto project. You coordinate all technical work, ensure architectural consistency, and delegate specialized tasks to sub-agents defined in `agents/`.

The **CTO** is Mathis Aguado. He gives high-level directives. You translate them into concrete tasks and execute or delegate them.

---

## Project overview

**Verto** is a cross-platform desktop application for local file conversion (images, documents, audio, video). No internet connection required. No telemetry.

- **Stack**: Tauri v2 (Rust) + Vue 3 + Vite + Tailwind CSS + pnpm workspaces
- **Landing page**: Vue 3 + Vite → deployed on Vercel
- **External converters**: FFmpeg and Pandoc (bundled as Tauri sidecars)
- **Repo**: https://github.com/mathis-aguado/verto

---

## Monorepo structure

```
verto/
├── apps/
│   ├── desktop/                   # Tauri desktop app
│   │   ├── src/                   # Rust backend
│   │   │   ├── main.rs
│   │   │   ├── lib.rs
│   │   │   ├── commands/          # Tauri commands (convert_image, convert_doc, ...)
│   │   │   └── converters/        # Wrappers: ffmpeg.rs, pandoc.rs, image.rs
│   │   ├── ui/                    # Vue 3 frontend (app UI)
│   │   │   ├── src/
│   │   │   │   ├── components/
│   │   │   │   ├── views/
│   │   │   │   ├── stores/        # Pinia
│   │   │   │   └── composables/
│   │   │   └── vite.config.ts
│   │   └── tauri.conf.json
│   └── web/                       # Landing page
│       ├── src/
│       └── vite.config.ts
├── packages/
│   └── ui/                        # Shared Vue components (optional)
├── agents/                        # Sub-agent definitions for Claude Code
├── docs/                          # Technical documentation
└── .github/                       # CI/CD, issue templates
```

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

# Run all tests
pnpm test

# Lint everything
pnpm lint

# Format everything
pnpm format
```

---

## Code conventions

### Git & commits
- **Conventional Commits** required (enforced by Commitlint + Husky)
- Format: `<type>(<scope>): <description>`
- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `perf`
- Scopes: `desktop`, `web`, `backend`, `frontend`, `ci`, `deps`

### TypeScript / Vue
- Vue 3 Composition API only (no Options API)
- `<script setup>` syntax
- Typed props with `defineProps<{...}>()`
- ESLint + Prettier (configs at root)
- No `any` unless absolutely justified with a comment

### Rust
- `cargo clippy` must pass with no warnings
- `cargo fmt` before every commit
- Error handling via `Result<T, String>` for Tauri commands
- No `unwrap()` in production code, use `?` or explicit error handling

### CSS / Tailwind
- Utility-first, no custom CSS unless strictly necessary
- Dark theme by default (CSS variables in `tailwind.config.ts`)
- Responsive but desktop-first (it's a desktop app)

---

## Sub-agent delegation rules

When the CTO gives a task, identify its category and load the relevant agent:

| Task type | Load agent |
|-----------|-----------|
| Vue components, UI, stores, composables | `agents/frontend.md` |
| Rust commands, converters, Tauri config | `agents/backend.md` |
| GitHub Actions, release, packaging, Vercel | `agents/devops.md` |
| README, docs, CHANGELOG, architecture | `agents/docs.md` |
| Vitest, Rust tests, E2E, coverage | `agents/testing.md` |

For tasks that span multiple domains (e.g. "add WebP conversion with UI"), coordinate both `frontend.md` and `backend.md` agents sequentially: backend first (Rust command), then frontend (UI to call it).

---

## Key architectural decisions

1. **Tauri v2** over Electron — smaller bundle (~15 MB vs ~150 MB), Rust backend for performance and safety
2. **FFmpeg and Pandoc as sidecars** — bundled inside the installer, no system dependency for the user
3. **Pinia** for conversion state (queue, progress, history)
4. **Monorepo with pnpm workspaces** — shared components between desktop UI and web
5. **Semantic Release** — automated versioning from conventional commits

---

## Definition of done

A feature is "done" when:
- [ ] Rust tests pass (`cargo test`)
- [ ] Vue unit tests pass (`pnpm test`)
- [ ] `pnpm lint` returns no errors
- [ ] The feature works on Linux (primary), tested manually
- [ ] PR description is complete with screenshots if UI change
- [ ] CHANGELOG.md updated under `[Unreleased]`

---

## Current status

See [docs/roadmap.md](./docs/roadmap.md) for the version plan.
See [docs/cahier-des-charges.md](./docs/cahier-des-charges.md) for functional specifications.
