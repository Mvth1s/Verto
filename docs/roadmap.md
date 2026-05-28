# Roadmap

## Version strategy

- `v0.x` — alpha/beta, features in progress, breaking changes possible
- `v1.0` — stable, all core features, cross-platform
- `v1.x` — stable improvements, no breaking changes
- `v2.0` — new surface (TUI, CLI) if validated

---

## v0.1.0 — Foundation *(alpha)* ✅

**Goal**: Project works, image conversion functional, CI in place.

- [x] Monorepo initialized (Tauri v2 + Vue 3 + pnpm workspaces)
- [x] Desktop app boots on Linux
- [x] Drag & drop working (images only)
- [x] Image conversion: JPEG ↔ PNG ↔ WebP (via `image` crate)
- [x] Basic conversion queue with per-file status
- [x] Output format selector
- [x] GitHub Actions: lint + build (Linux only)
- [ ] Landing page deployed on Vercel

---

## v0.2.0 — Documents + Batch *(beta)*

**Goal**: Documents conversion, batch support, Windows/macOS builds.

- [ ] Document conversion: Markdown → PDF, HTML; DOCX → PDF (via Pandoc sidecar)
- [ ] Pandoc bundled as Tauri sidecar (Linux, Windows, macOS)
- [ ] Batch conversion: drag & drop of folders
- [x] Cancel pending conversion
- [x] Retry failed conversion
- [ ] Per-category navigation (sidebar: Images / Documents)
- [ ] Build matrix: Linux + Windows + macOS in CI
- [x] Preferences: default output format, default output directory
- [ ] Landing page complete with download links

---

## v0.3.0 — Audio *(beta)*

**Goal**: Audio conversion via FFmpeg sidecar.

- [ ] FFmpeg bundled as Tauri sidecar (Linux, Windows, macOS)
- [ ] Audio conversion: MP3, FLAC, OGG, WAV, AAC
- [ ] Audio category in sidebar
- [ ] Bitrate option

---

## v0.4.0 — Image quality *(beta)*

**Goal**: Advanced image options.

- [ ] Image resize (width × height, keep ratio)
- [ ] AVIF support (via FFmpeg)
- [ ] Quality presets ("Web", "Print", "Lossless")
- [ ] Preview before conversion (thumbnail)

---

## v1.0.0 — Stable release

**Goal**: Polished, tested, documented, cross-platform.

- [ ] All v0.x features stable
- [ ] Coverage ≥ targets (see `agents/testing.md`)
- [ ] Accessibility: keyboard nav, ARIA
- [ ] i18n: French + English
- [ ] Auto-updater (Tauri built-in)
- [ ] Full landing page with screenshots, proper SEO
- [ ] `CHANGELOG.md` complete
- [ ] GitHub release with all platform artifacts

---

## v1.1.0 — Video

- [ ] Video conversion: MP4, MKV, WebM, MOV (via FFmpeg)
- [ ] Resolution option
- [ ] Codec selection (H.264, H.265, VP9)

---

## v2.0.0 — New surfaces *(if validated)*

- [ ] TUI interface (Ratatui / Rust)
- [ ] CLI interface (`verto convert input.png --to webp`)
- [ ] Plugin system for community formats

---

## Not planned

- Cloud sync
- Mobile app
- SaaS/subscription model
- Telemetry of any kind
