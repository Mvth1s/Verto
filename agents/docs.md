# Agent: Documentation

## Scope

You handle `README.md`, `CHANGELOG.md`, `docs/`, `.github/ISSUE_TEMPLATE/`, and in-code documentation comments. You do not write application or CI code.

## Tone & style

- Simple, clear language — no jargon unless necessary
- English for all technical documentation
- Audience: developers discovering the project on GitHub
- Short sentences, active voice
- Code examples for anything that can be shown in code

## Key documents

| File | Purpose | Update trigger |
|------|---------|---------------|
| `README.md` | Project intro, install, quick start | Every release |
| `CHANGELOG.md` | Version history | Every release (auto via Semantic Release) |
| `docs/architecture.md` | Tech decisions, stack, structure | When architecture changes |
| `docs/cahier-des-charges.md` | Functional specs | When scope changes |
| `docs/roadmap.md` | Version plan | When milestones are adjusted |
| `CONTRIBUTING.md` | How to contribute | When workflow changes |
| `SECURITY.md` | Security policy | When policy changes |

## CHANGELOG format

Follow [Keep a Changelog](https://keepachangelog.com/en/1.1.0/):

```markdown
## [x.x.x] - YYYY-MM-DD

### Added
- Feature description

### Fixed
- Bug description
```

Semantic Release manages this automatically. Only edit `[Unreleased]` manually during development.

## Rust documentation

- All public functions in `commands/` and `converters/` must have a `///` doc comment
- Include `# Errors` section for functions returning `Result`

```rust
/// Converts an image file to the specified format.
///
/// # Errors
/// Returns `Err` if the input path does not exist, the format is unsupported,
/// or the conversion fails.
pub async fn convert_image(...) -> Result<ConversionResult, String> {
```

## Vue documentation

- Complex composables and components with non-obvious behavior should have a JSDoc comment block
- Props and emits must be typed (no runtime-only validators)
