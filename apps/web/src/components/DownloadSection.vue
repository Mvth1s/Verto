<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { RELEASES_URL, useLatestRelease } from '../composables/useLatestRelease'

interface Fallback {
  ext: string
}

const LINUX_FALLBACKS: Fallback[] = [{ ext: '.AppImage' }, { ext: '.deb' }, { ext: '.rpm' }]
const WINDOWS_FALLBACKS: Fallback[] = [{ ext: '.exe' }, { ext: '.msi' }]
const MACOS_FALLBACKS: Fallback[] = [{ ext: '.dmg' }]

const { t } = useI18n()
const { version, findAssets } = useLatestRelease()

const linuxAssets = computed(() =>
  findAssets((n) => n.endsWith('.AppImage') || n.endsWith('.deb') || n.endsWith('.rpm')),
)

const windowsAssets = computed(() =>
  findAssets((n) => n.endsWith('-setup.exe') || n.endsWith('.msi')),
)

const macosAssets = computed(() => findAssets((n) => n.endsWith('.dmg')))

function sizeMB(bytes: number): string {
  return `~${Math.round(bytes / 1024 / 1024)} MB`
}

function extOf(name: string): string {
  if (name.endsWith('.AppImage')) return '.AppImage'
  if (name.endsWith('-setup.exe')) return '.exe'
  if (name.endsWith('.msi')) return '.msi'
  if (name.endsWith('.deb')) return '.deb'
  if (name.endsWith('.rpm')) return '.rpm'
  if (name.endsWith('.dmg')) return '.dmg'
  return '.' + (name.split('.').pop() ?? name)
}
</script>

<template>
  <section id="download">
    <div class="container">
      <div class="section-head" style="text-align: center; margin-left: auto; margin-right: auto">
        <div class="section-eyebrow">{{ t('download.eyebrow') }}</div>
        <h2 class="section-title">{{ t('download.section_title') }}</h2>
        <p class="section-sub" style="margin-left: auto; margin-right: auto">
          {{ t('download.section_sub') }}
        </p>
      </div>
      <div class="download-grid">
        <!-- Linux -->
        <div class="dl-card">
          <div class="dl-os">
            <div class="dl-os-icon">
              <img
                src="/tux.png"
                alt="Linux"
                width="18"
                height="18"
                loading="lazy"
                class="dl-os-icon-img"
              />
            </div>
            <div>
              <div class="dl-os-name">{{ t('download.linux') }}</div>
              <div class="dl-os-version">{{ version ? `${version} · x86_64` : 'x86_64' }}</div>
            </div>
          </div>
          <div class="dl-formats">
            <template v-if="linuxAssets.length">
              <div v-for="a in linuxAssets" :key="a.name" class="row">
                <span>{{ extOf(a.name) }}</span
                ><span class="size">{{ sizeMB(a.size) }}</span>
              </div>
            </template>
            <template v-else>
              <div v-for="f in LINUX_FALLBACKS" :key="f.ext" class="row">
                <span>{{ f.ext }}</span>
              </div>
            </template>
          </div>
          <div class="dl-links">
            <template v-if="linuxAssets.length">
              <a
                v-for="(a, i) in linuxAssets"
                :key="a.name"
                :class="['dl-btn', i > 0 ? 'outline' : '']"
                :href="a.browser_download_url"
                target="_blank"
                rel="noopener"
              >
                <svg viewBox="0 0 24 24">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                  <polyline points="7 10 12 15 17 10" />
                  <line x1="12" y1="15" x2="12" y2="3" />
                </svg>
                {{ t('download.format', { ext: extOf(a.name) }) }}
              </a>
            </template>
            <template v-else>
              <a
                v-for="(f, i) in LINUX_FALLBACKS"
                :key="f.ext"
                :class="['dl-btn', i > 0 ? 'outline' : '']"
                :href="RELEASES_URL"
                target="_blank"
                rel="noopener"
              >
                <svg viewBox="0 0 24 24">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                  <polyline points="7 10 12 15 17 10" />
                  <line x1="12" y1="15" x2="12" y2="3" />
                </svg>
                {{ t('download.format', { ext: f.ext }) }}
              </a>
            </template>
          </div>
        </div>

        <!-- Windows -->
        <div class="dl-card">
          <div class="dl-os">
            <div class="dl-os-icon">
              <svg viewBox="0 0 24 24">
                <path
                  d="M0 3.449L9.75 2.1v9.451H0m10.949-9.602L24 0v11.4H10.949M0 12.6h9.75v9.451L0 20.699M10.949 12.6H24V24l-13.051-1.351"
                />
              </svg>
            </div>
            <div>
              <div class="dl-os-name">{{ t('download.windows') }}</div>
              <div class="dl-os-version">{{ version ? `${version} · x86_64` : 'x86_64' }}</div>
            </div>
          </div>
          <div class="dl-formats">
            <template v-if="windowsAssets.length">
              <div v-for="a in windowsAssets" :key="a.name" class="row">
                <span>{{ extOf(a.name) }}</span
                ><span class="size">{{ sizeMB(a.size) }}</span>
              </div>
            </template>
            <template v-else>
              <div v-for="f in WINDOWS_FALLBACKS" :key="f.ext" class="row">
                <span>{{ f.ext }}</span>
              </div>
            </template>
          </div>
          <div class="dl-links">
            <template v-if="windowsAssets.length">
              <a
                v-for="(a, i) in windowsAssets"
                :key="a.name"
                :class="['dl-btn', i > 0 ? 'outline' : '']"
                :href="a.browser_download_url"
                target="_blank"
                rel="noopener"
              >
                <svg viewBox="0 0 24 24">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                  <polyline points="7 10 12 15 17 10" />
                  <line x1="12" y1="15" x2="12" y2="3" />
                </svg>
                {{ t('download.format', { ext: extOf(a.name) }) }}
              </a>
            </template>
            <template v-else>
              <a
                v-for="(f, i) in WINDOWS_FALLBACKS"
                :key="f.ext"
                :class="['dl-btn', i > 0 ? 'outline' : '']"
                :href="RELEASES_URL"
                target="_blank"
                rel="noopener"
              >
                <svg viewBox="0 0 24 24">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                  <polyline points="7 10 12 15 17 10" />
                  <line x1="12" y1="15" x2="12" y2="3" />
                </svg>
                {{ t('download.format', { ext: f.ext }) }}
              </a>
            </template>
          </div>
        </div>

        <!-- macOS -->
        <div class="dl-card">
          <div class="dl-os">
            <div class="dl-os-icon">
              <svg viewBox="0 0 24 24">
                <path
                  d="M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.34 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z"
                />
              </svg>
            </div>
            <div>
              <div class="dl-os-name">{{ t('download.macos') }}</div>
              <div class="dl-os-version">
                {{ version ? `${version} · Apple Silicon` : 'Apple Silicon' }}
              </div>
            </div>
          </div>
          <div class="dl-formats">
            <template v-if="macosAssets.length">
              <div v-for="a in macosAssets" :key="a.name" class="row">
                <span>{{ extOf(a.name) }}</span
                ><span class="size">{{ sizeMB(a.size) }}</span>
              </div>
            </template>
            <template v-else>
              <div v-for="f in MACOS_FALLBACKS" :key="f.ext" class="row">
                <span>{{ f.ext }}</span>
              </div>
            </template>
          </div>
          <div class="dl-links">
            <template v-if="macosAssets.length">
              <a
                v-for="(a, i) in macosAssets"
                :key="a.name"
                :class="['dl-btn', i > 0 ? 'outline' : '']"
                :href="a.browser_download_url"
                target="_blank"
                rel="noopener"
              >
                <svg viewBox="0 0 24 24">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                  <polyline points="7 10 12 15 17 10" />
                  <line x1="12" y1="15" x2="12" y2="3" />
                </svg>
                {{ t('download.format', { ext: extOf(a.name) }) }}
              </a>
            </template>
            <template v-else>
              <a
                v-for="(f, i) in MACOS_FALLBACKS"
                :key="f.ext"
                :class="['dl-btn', i > 0 ? 'outline' : '']"
                :href="RELEASES_URL"
                target="_blank"
                rel="noopener"
              >
                <svg viewBox="0 0 24 24">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                  <polyline points="7 10 12 15 17 10" />
                  <line x1="12" y1="15" x2="12" y2="3" />
                </svg>
                {{ t('download.format', { ext: f.ext }) }}
              </a>
            </template>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
