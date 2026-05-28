# Verto — Design Prompts

Ces deux prompts sont à coller dans une conversation Claude avec le skill **canvas-design** activé.
Utilise-les séparément, un par conversation.

---

## Prompt 1 — Maquette de l'application desktop

```
Create a UI mockup for a desktop file conversion application called "Verto".

Design philosophy: Functional Minimalism — every element earns its place.
The interface should feel like a precision tool, not a consumer app.
Dark, focused, with one clear accent color (emerald green).

Layout (single window, ~1280×800px):

LEFT SIDEBAR (narrow, ~200px):
- App logo/name "Verto" at the top (small, wordmark)
- Navigation categories with icons:
  · Images (active state: emerald accent, bold icon)
  · Documents
  · Audio (grayed out, coming soon)
  · Video (grayed out, coming soon)
- Settings icon at the bottom

MAIN AREA (center, ~700px wide):
- Large drag-and-drop zone occupying most of the space
  · Dashed border, slightly rounded corners
  · Icon + text: "Drop files here" in secondary text color
  · Subtle background differentiation from the rest
- Below it: a compact conversion queue showing 3 files:
  · File 1: "photo.jpg → WebP" — status: done (green checkmark)
  · File 2: "diagram.png → WebP" — status: converting (progress bar, 60%)
  · File 3: "logo.tiff → WebP" — status: waiting (clock icon)

RIGHT PANEL (~280px):
- Section title: "Output settings"
- Format selector dropdown (showing "WebP")
- Quality slider (85%)
- Output folder: row with folder path and a "Browse" button
- Large primary button: "Convert" in emerald

Color palette:
- Background: zinc-900 (#18181b)
- Surface/cards: zinc-800 (#27272a)
- Borders: zinc-700 (#3f3f46)
- Primary accent: emerald-500 (#10b981)
- Text primary: zinc-100 (#f4f4f5)
- Text secondary: zinc-400 (#a1a1aa)
- Success: emerald-400
- Progress bar: emerald-500

Typography: Inter — clean, neutral, legible at small sizes.

The result should feel like a tool a developer or designer would be proud to use daily.
Meticulous spacing, perfect alignment, every pixel intentional.
This is not a mockup with lorem ipsum — it shows real file names, real states, real data.
```

---

## Prompt 2 — Maquette de la landing page

```
Create a landing page mockup for an open-source desktop application called "Verto".
Verto converts files locally — images, documents, audio, video — with no internet connection.

Design philosophy: Clarity Over Everything — a developer landing page that builds instant trust.
Dark background, minimal copy, the app does the talking.

Page structure (full viewport width, desktop ~1440px):

HEADER (sticky, minimal):
- Left: "Verto" wordmark + small logo
- Right: "GitHub" link with star count placeholder, "Download" CTA button (emerald, pill shape)

HERO SECTION (full viewport height):
- Left half (text):
  · Eyebrow tag: "Open source · Local · Free"
  · Headline (large, bold): "Convert anything."
    Subline below: "Locally."
  · Body copy (small, zinc-400): "No internet. No account. No telemetry.
    Just drag, drop, and convert."
  · Two CTA buttons:
    — Primary: "Download for Linux" (emerald, filled)
    — Secondary: "View on GitHub" (outline)
  · Below buttons: small OS badges (Linux / Windows / macOS)
- Right half: app screenshot/mockup in a subtle window frame,
  slightly tilted or with a shadow glow, showing the drag zone with a file being converted

FEATURES SECTION:
- Section title: "Everything you need. Nothing you don't."
- 4 feature cards in a 2×2 grid:
  · "Images" — icon + "JPEG, PNG, WebP, AVIF and more"
  · "Documents" — icon + "PDF, DOCX, Markdown, HTML"
  · "100% Local" — icon + "Your files never leave your machine"
  · "Open source" — icon + "MIT licensed. No black boxes."

PRIVACY SECTION (full-width, dark surface):
- Large centered text: "Your files stay on your machine."
- Supporting copy: one short paragraph about no uploads, no logs, no accounts
- A simple visual: a laptop icon with a lock, no cloud, no server

DOWNLOAD SECTION:
- Title: "Ready to install?"
- 3 download cards side by side:
  · Linux (.AppImage / .deb)
  · Windows (.exe)
  · macOS (.dmg)
  Each card has an OS logo, version number placeholder, and a download button

FOOTER:
- Left: "Verto · MIT License · 2025"
- Right: GitHub link, link to CHANGELOG

Color palette: same as the app (zinc-900 base, emerald-500 accent)
Typography: Inter throughout

The page should look like a tool made by a developer for developers —
not a SaaS marketing page. Trust signals: open source, local-first, MIT.
Clean, fast, no fluff. Meticulously crafted. Every section purposeful.
```
