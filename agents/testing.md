# Agent: Testing

## Scope

You write and maintain tests across the full stack: Rust unit tests, Vue unit tests (Vitest), and integration/E2E tests. You do not write production application code.

## Stack

- **Rust**: built-in `#[test]` + `#[tokio::test]` for async
- **Vue / TypeScript**: Vitest + Vue Test Utils
- **E2E**: Tauri's WebDriver integration (future, v1.0+)

## Rust tests

Located alongside the source in `src/converters/*.rs` and `src/commands/*.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_format_detection() {
        assert!(is_supported_image_format("webp"));
        assert!(!is_supported_image_format("xyz"));
    }

    #[tokio::test]
    async fn test_jpeg_to_png_conversion() {
        // Use fixtures in tests/fixtures/
        let result = convert("tests/fixtures/sample.jpg", "png", None).await;
        assert!(result.is_ok());
    }
}
```

Test fixtures go in `apps/desktop/tests/fixtures/` (small sample files per format).

## Vitest tests

Located in `apps/desktop/ui/src/**/__tests__/` or `*.test.ts` alongside the file:

```typescript
import { mount } from '@vue/test-utils'
import { describe, it, expect } from 'vitest'
import DropZone from '../DropZone.vue'

describe('DropZone', () => {
  it('emits files-dropped when files are dropped', async () => {
    const wrapper = mount(DropZone)
    // ...
    expect(wrapper.emitted('files-dropped')).toBeTruthy()
  })
})
```

Mock `@tauri-apps/api/core` invoke calls in tests:

```typescript
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue({ success: true, outputPath: '/tmp/out.webp' }),
}))
```

## Coverage targets

| Layer | Target |
|-------|--------|
| Rust converters | ≥ 80% |
| Tauri commands | ≥ 70% |
| Vue stores | ≥ 80% |
| Vue components | ≥ 60% |

## Running tests

```bash
# All Vue tests
pnpm --filter desktop test

# Watch mode
pnpm --filter desktop test:watch

# Coverage
pnpm --filter desktop test:coverage

# Rust tests
cd apps/desktop && cargo test
```

## Test conventions

- One `describe` block per component/function
- Test names: "should <expected behavior> when <condition>"
- No network calls, no real file system writes — use temp dirs or mocks
- Each test must be independent (no shared mutable state)
