# Contributing to Verto

Thank you for your interest in contributing! Verto is an open-source project and contributions of all kinds are welcome.

## Ways to contribute

- 🐛 Report a bug
- 💡 Suggest a feature
- 🔧 Fix a bug or implement a feature
- 📚 Improve documentation
- 🌍 Add or improve a translation

## Before you start

- Check the [existing issues](https://github.com/mathis-aguado/verto/issues) to avoid duplicates.
- For large changes, open an issue first to discuss the approach.
- Read [docs/architecture.md](./docs/architecture.md) to understand the project structure.

## Setup

```bash
git clone https://github.com/mathis-aguado/verto.git
cd verto
pnpm install
pnpm tauri dev
```

## Branch naming

| Type        | Pattern                   | Example                      |
|-------------|---------------------------|------------------------------|
| Feature     | `feat/<short-description>`  | `feat/webp-support`          |
| Bug fix     | `fix/<short-description>`   | `fix/pdf-encoding`           |
| Docs        | `docs/<short-description>`  | `docs/update-architecture`   |
| Chore       | `chore/<short-description>` | `chore/update-dependencies`  |

## Commit convention

This project uses [Conventional Commits](https://www.conventionalcommits.org/).

```
<type>(<scope>): <short description>

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `perf`

Scopes: `desktop`, `web`, `backend`, `frontend`, `ci`, `deps`

Examples:

```
feat(backend): add WebP to AVIF conversion
fix(frontend): correct drag zone highlight on DnD
docs: update installation instructions
```

Commits that don't follow this convention will be rejected by the pre-commit hooks (Commitlint + Husky).

## Pull requests

1. Fork the repository and create your branch from `main`.
2. Make sure `pnpm lint` and `pnpm test` pass.
3. Fill in the PR template fully.
4. Link the related issue if applicable.

## Code style

- **TypeScript / Vue**: ESLint + Prettier (config in root)
- **Rust**: `cargo clippy` + `cargo fmt`

Run all checks:

```bash
pnpm lint         # ESLint + Prettier
cargo clippy      # Rust linter (in apps/desktop/src)
cargo fmt --check # Rust formatter
```

## Reporting a security vulnerability

Please do **not** open a public issue. See [SECURITY.md](./SECURITY.md).
