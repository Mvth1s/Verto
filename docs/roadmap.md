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
- [x] Landing page deployed on Vercel

---

## v0.2.0 — Documents + Batch *(beta)*

**Goal**: Documents conversion, batch support, Windows/macOS builds.

- [x] Document conversion: Markdown → PDF, HTML; DOCX → PDF (via Pandoc sidecar)
- [x] Pandoc bundled as Tauri sidecar (Linux, Windows, macOS)
- [x] Batch conversion: drag & drop of folders
- [x] Cancel pending conversion
- [x] Retry failed conversion
- [x] Per-category navigation (sidebar: Images / Documents)
- [x] Build matrix: Linux + Windows + macOS in CI
- [x] Preferences: default output format, default output directory
- [x] Landing page complete with download links

---

## v0.3.0 — Audio *(beta)*

**Goal**: Audio conversion via FFmpeg sidecar.

- [x] FFmpeg bundled as Tauri sidecar (Linux, Windows, macOS)
- [x] Audio conversion: MP3, FLAC, OGG, WAV, AAC
- [x] Audio category in sidebar
- [x] Bitrate option

---

## v0.4.0 — Image quality *(beta)*

**Goal**: Advanced image options.

- [x] Image resize (width × height, keep ratio)
- [x] AVIF support (via FFmpeg)
- [x] Quality presets ("Web", "Print", "Lossless")
- [x] Preview before conversion (thumbnail)

---

## v1.0.0 — Stable release ✅

**Goal**: Polished, tested, documented, cross-platform.

- [x] All v0.x features stable
- [x] Coverage ≥ targets (App.vue 81%, stores 100%, Rust 26 tests)
- [x] Accessibility: keyboard nav, ARIA
- [x] i18n: French + English (desktop app)
- [x] Auto-updater (Tauri built-in)
- [x] Full landing page with SEO
- [x] `CHANGELOG.md` complete
- [x] GitHub release with all platform artifacts

---

## v1.1.0 — Video ✅

- [x] Video conversion: MP4, MKV, WebM, MOV (via FFmpeg)
- [x] Resolution option (width × height, keep-ratio)
- [x] Codec selection (H.264, H.265, VP9) — filtered by output format
- [x] Video thumbnail preview in queue
- [x] Settings page with persistence (output format, quality, bitrate, codec, directory)
- [x] System notification when batch conversion completes
- [x] i18n EN/FR on landing page (CSS mockup in hero)

---

## v1.6.0 — UX & formats ✅

- [x] Barre de progression FFmpeg en temps réel (audio + vidéo)
- [x] Cancel immédiat du process FFmpeg actif (`child.kill()`)
- [x] Suppression du fichier de sortie partiel après échec ou cancel
- [x] Support HEIC/HEIF en entrée (décodage via FFmpeg)
- [x] i18n EN/FR sur la landing page

---

## v1.7.0 — Qualité & historique *(en cours)*

- [x] Messages d'erreur FFmpeg lisibles (extraction de la ligne significative)
- [x] Historique de conversion session (onglet History, badge, clear)
- [ ] Raccourcis clavier : `Suppr` retirer fichier, `Entrée` lancer, `Échap` cancel

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
