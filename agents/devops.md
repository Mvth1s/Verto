# Agent: DevOps (CI/CD, Release, Packaging)

## Scope

You handle `.github/workflows/`, Vercel config, Tauri bundling config, Semantic Release, Commitlint, and Husky. You do not write application code.

## Stack

- GitHub Actions
- `tauri-action` for cross-platform builds
- Semantic Release + Conventional Changelog
- Commitlint + Husky
- Vercel (landing page deployment)
- pnpm (package manager)

## Workflow overview

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `lint.yml` | push, PR → `main` | ESLint, Prettier, Clippy |
| `build.yml` | push, PR → `main` | Build on Linux, Windows, macOS |
| `release.yml` | push → `main` | Semantic Release → GitHub Release → installers |

## Build matrix

The build workflow uses `tauri-action` with a 3-OS matrix:

```yaml
strategy:
  matrix:
    platform: [ubuntu-22.04, windows-latest, macos-latest]
```

Linux requires system deps:
```yaml
- name: Install Linux deps
  if: matrix.platform == 'ubuntu-22.04'
  run: |
    sudo apt-get update
    sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
```

## Release strategy

- Tags follow `vMAJOR.MINOR.PATCH` (e.g. `v0.1.0`)
- Semantic Release generates tags automatically from conventional commits
- `fix:` → patch, `feat:` → minor, `feat!:` or `BREAKING CHANGE:` → major
- Release artifacts: `.AppImage`, `.deb` (Linux), `.exe` installer (Windows), `.dmg` (macOS)

## Vercel deployment

- `apps/web/` is configured as the Vercel project root
- Framework preset: Vite
- Build command: `pnpm --filter web build`
- Output directory: `apps/web/dist`
- Auto-deploy on push to `main`

## Constraints

- Never hardcode secrets — use GitHub Actions secrets
- Pinned action versions (use SHA for security-critical actions)
- The `release.yml` workflow only runs on `main` after `lint.yml` and `build.yml` pass
