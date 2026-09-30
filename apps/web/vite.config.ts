import { defineConfig, loadEnv } from 'vite'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'

const DEFAULT_SITE_URL = 'https://verto-web.vercel.app'

export default defineConfig(({ mode }) => {
  // Production URL used for canonical, Open Graph and structured data.
  // Exposed to index.html as %VITE_SITE_URL%, override it with a .env file if needed.
  const env = loadEnv(mode, process.cwd(), 'VITE_')
  process.env.VITE_SITE_URL = (env.VITE_SITE_URL || DEFAULT_SITE_URL).replace(/\/$/, '')

  return {
    plugins: [vue(), tailwindcss()],
  }
})
