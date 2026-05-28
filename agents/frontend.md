# Agent: Frontend (Vue 3)

## Scope

You handle everything in `apps/desktop/ui/` and `apps/web/`. You write Vue 3 components, Pinia stores, composables, and Tailwind styles. You do not touch Rust code.

## Stack

- Vue 3 (Composition API, `<script setup>`)
- Vite
- Tailwind CSS (dark theme first)
- Pinia (state management)
- `@tauri-apps/api` (to call Rust commands via `invoke()`)

## Calling Rust commands

```typescript
import { invoke } from '@tauri-apps/api/core'

// Example: trigger an image conversion
const result = await invoke<ConversionResult>('convert_image', {
  inputPath: '/path/to/file.png',
  outputFormat: 'webp',
  quality: 85,
})
```

Always type the return value. Handle errors with try/catch and display them in the UI.

## Component conventions

```vue
<script setup lang="ts">
// 1. imports
// 2. props / emits
// 3. stores
// 4. reactive state
// 5. computed
// 6. functions
// 7. lifecycle hooks
</script>

<template>
  <!-- Single root element -->
</template>
```

- Components in `components/` are kebab-case file names, PascalCase in template
- Views (pages/routes) go in `views/`
- Reusable logic goes in `composables/use*.ts`

## Design system

- Dark background: `bg-zinc-900`
- Surface: `bg-zinc-800`
- Border: `border-zinc-700`
- Primary accent: `text-emerald-400` / `bg-emerald-500`
- Text primary: `text-zinc-100`
- Text secondary: `text-zinc-400`
- Font: Inter (system stack fallback)

## Key components to implement

- `DropZone.vue` — drag & drop area, file picker fallback
- `FormatSelector.vue` — dropdown for output format
- `ConversionQueue.vue` — list of pending/done conversions with progress
- `ConversionOptions.vue` — quality slider, output path picker
- `CategorySidebar.vue` — Images / Documents / Audio / Video navigation

## Constraints

- No inline styles
- No Options API
- No `any` in TypeScript
- All user-facing text goes through i18n keys (even if only FR/EN at first)
- Accessible: keyboard navigation, ARIA labels on interactive elements
