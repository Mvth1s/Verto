import { readFileSync } from 'node:fs'
import { defineConfig, loadEnv, type Plugin } from 'vite'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'
import { buildStructuredData } from './src/seo/structured-data'

const DEFAULT_SITE_URL = 'https://verto-web.vercel.app'

// Released app version, bumped by Semantic Release in the root package.json.
function readAppVersion(): string | undefined {
  try {
    const pkg = JSON.parse(readFileSync(new URL('../../package.json', import.meta.url), 'utf8'))
    return typeof pkg.version === 'string' ? pkg.version : undefined
  } catch {
    return undefined
  }
}

function structuredData(siteUrl: string): Plugin {
  return {
    name: 'verto-structured-data',
    transformIndexHtml() {
      const json = JSON.stringify(buildStructuredData({ siteUrl, version: readAppVersion() }))
      return [
        {
          tag: 'script',
          attrs: { type: 'application/ld+json' },
          // Escape "<" so the payload can never close the script tag
          children: json.replace(/</g, '\\u003c'),
          injectTo: 'head',
        },
      ]
    },
  }
}

export default defineConfig(({ mode }) => {
  // Production URL used for canonical, Open Graph and structured data.
  // Exposed to index.html as %VITE_SITE_URL%, override it with a .env file if needed.
  const env = loadEnv(mode, process.cwd(), 'VITE_')
  const siteUrl = (env.VITE_SITE_URL || DEFAULT_SITE_URL).replace(/\/$/, '')
  process.env.VITE_SITE_URL = siteUrl

  return {
    plugins: [vue(), tailwindcss(), structuredData(siteUrl)],
  }
})
