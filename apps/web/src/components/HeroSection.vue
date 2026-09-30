<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useLatestRelease } from '../composables/useLatestRelease'

const { t } = useI18n()
const { release, findAssets } = useLatestRelease()

type DetectedOS = 'windows' | 'macos' | 'linux' | 'unknown'

function detectOS(): DetectedOS {
  const ua = navigator.userAgent
  if (/Windows/i.test(ua)) return 'windows'
  if (/Mac OS X|Macintosh/i.test(ua)) return 'macos'
  if (/Linux/i.test(ua)) return 'linux'
  return 'unknown'
}

const detectedOS = detectOS()

const heroPrimaryLabel = computed(() => {
  if (detectedOS === 'unknown') return t('download.generic')
  const osLabel: Record<Exclude<DetectedOS, 'unknown'>, string> = {
    windows: t('os.windows'),
    macos: t('os.macos'),
    linux: t('os.linux'),
  }
  return t('download.forOs', { os: osLabel[detectedOS] })
})

const heroPrimaryHref = computed((): string => {
  if (detectedOS === 'unknown' || !release.value) return '#download'
  const assets = findAssets(() => true)
  const find = (pred: (n: string) => boolean) => assets.find((a) => pred(a.name))
  if (detectedOS === 'windows') {
    return (
      find((n) => n.endsWith('-setup.exe') || n.endsWith('.msi'))?.browser_download_url ??
      '#download'
    )
  }
  if (detectedOS === 'macos') {
    return find((n) => n.endsWith('.dmg'))?.browser_download_url ?? '#download'
  }
  return find((n) => n.endsWith('.AppImage'))?.browser_download_url ?? '#download'
})
</script>

<template>
  <section class="hero">
    <div class="container hero-grid">
      <div>
        <div class="eyebrow"><span class="dot"></span>{{ t('hero.eyebrow') }}</div>
        <h1 class="headline">
          {{ t('hero.headline') }}
          <span class="subline">{{ t('hero.subline') }}</span>
        </h1>
        <p class="hero-body">{{ t('hero.body') }}</p>
        <div class="hero-ctas">
          <a
            class="btn btn-primary btn-lg"
            :href="heroPrimaryHref"
            v-bind="heroPrimaryHref !== '#download' ? { target: '_blank', rel: 'noopener' } : {}"
          >
            <svg viewBox="0 0 24 24">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
              <polyline points="7 10 12 15 17 10" />
              <line x1="12" y1="15" x2="12" y2="3" />
            </svg>
            {{ heroPrimaryLabel }}
          </a>
          <a class="btn btn-outline btn-lg" href="#download">
            {{ t('download.allPlatforms') }}
          </a>
          <a
            class="btn btn-outline btn-lg"
            href="https://github.com/Mvth1s/Verto"
            target="_blank"
            rel="noopener"
          >
            <img src="/github.svg" alt="" width="16" height="16" class="btn-icon" />
            {{ t('hero.cta_github') }}
          </a>
        </div>
        <div class="os-row">
          <span class="label">{{ t('hero.available_for') }}</span>
          <div class="os-badges">
            <span class="os-badge">
              <svg viewBox="0 0 24 24">
                <path
                  d="M12.504 0c-.155 0-.315.008-.48.021-4.226.333-3.105 4.807-3.17 6.298-.076 1.092-.3 1.953-1.05 3.02-.885 1.051-2.127 2.75-2.716 4.521-.278.832-.41 1.684-.287 2.489.005.04.011.08.018.118-.078.224-.144.452-.183.692-.317 1.96.46 4.005 2.13 5.605 1.81 1.732 4.357 2.633 6.96 2.483 2.45-.137 4.62-1.157 6.18-2.825.78-.836 1.39-1.836 1.74-2.95.36-1.14.42-2.35.21-3.61-.06-.42-.18-.84-.36-1.26.06-.45.06-.9-.06-1.35-.24-.96-.78-1.86-1.5-2.55-.6-.6-1.32-1.05-2.1-1.32-.42-.15-.84-.24-1.26-.27-.045-.524-.06-1.05-.15-1.575-.36-2.1-1.32-4.2-2.97-5.355-.795-.555-1.755-.81-2.715-.81z"
                />
              </svg>
              {{ t('os.linux') }}
            </span>
            <span class="os-badge">
              <svg viewBox="0 0 24 24">
                <path
                  d="M0 3.449L9.75 2.1v9.451H0m10.949-9.602L24 0v11.4H10.949M0 12.6h9.75v9.451L0 20.699M10.949 12.6H24V24l-13.051-1.351"
                />
              </svg>
              {{ t('os.windows') }}
            </span>
            <span class="os-badge">
              <svg viewBox="0 0 24 24">
                <path
                  d="M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.34 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z"
                />
              </svg>
              {{ t('os.macos') }}
            </span>
          </div>
        </div>
      </div>

      <!-- App screenshot -->
      <div class="mockup">
        <div class="win">
          <div class="win-bar">
            <div class="traffic">
              <span class="r"></span><span class="y"></span><span class="g"></span>
            </div>
            <div class="label">{{ t('hero.mockup_title') }}</div>
          </div>
          <div class="win-body">
            <div class="ws-sidebar">
              <div class="ws-brand">
                <img src="/verto.png" alt="Verto" width="44" height="44" class="ws-brand-img" />
              </div>
              <div class="ws-item active">
                <span class="ico"></span>{{ t('features.images_title') }}
              </div>
              <div class="ws-item">
                <span class="ico"></span>{{ t('features.documents_title') }}
              </div>
              <div class="ws-item" style="opacity: 0.4">
                <span class="ico"></span>{{ t('features.audio_title') }}
              </div>
              <div class="ws-item" style="opacity: 0.4">
                <span class="ico"></span>{{ t('features.video_title') }}
              </div>
            </div>
            <div class="ws-main">
              <div class="ws-drop">
                <div class="icon-box">
                  <svg viewBox="0 0 24 24">
                    <path d="M12 3v12" />
                    <path d="M7 8l5-5 5 5" />
                    <path d="M5 21h14" />
                  </svg>
                </div>
                <div>{{ t('hero.mockup_drop') }}</div>
              </div>
              <div class="ws-row">
                <span class="name">diagram.png</span>
                <span class="arrow">→</span>
                <span class="to">.webp</span>
                <span class="ws-bar"><span class="fill"></span></span>
              </div>
            </div>
            <div class="ws-panel">
              <div>
                <div class="ws-label" style="margin-bottom: 6px">{{ t('hero.mockup_format') }}</div>
                <div class="ws-field"><span>WebP</span><span class="arrow-d">▾</span></div>
              </div>
              <div>
                <div
                  class="ws-label"
                  style="margin-bottom: 6px; display: flex; justify-content: space-between"
                >
                  <span>{{ t('hero.mockup_quality') }}</span
                  ><span style="color: var(--text); font-family: 'JetBrains Mono', monospace"
                    >85%</span
                  >
                </div>
                <div class="ws-slider"></div>
              </div>
              <div class="ws-btn">{{ t('hero.mockup_convert') }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
