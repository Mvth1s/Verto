<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { getVersion } from '@tauri-apps/api/app'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { convertFileSrc, invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useConversionStore } from './stores/conversion'
import { useSettingsStore } from './stores/settings'
import { type Locale } from './i18n'

type Category = 'images' | 'documents' | 'audio' | 'video' | 'history'

const IMAGE_FORMATS = ['webp', 'jpeg', 'png', 'avif', 'bmp', 'tiff', 'gif', 'ico']

interface FormatGroup {
  label: string
  formats: string[]
}

const AUDIO_FORMAT_GROUPS: FormatGroup[] = [
  {
    label: 'Lossy',
    formats: ['mp3', 'aac', 'm4a', 'opus', 'ogg', 'wma', 'spx', 'amr', 'gsm', 'mp2'],
  },
  {
    label: 'Lossless',
    formats: ['flac', 'wav', 'aiff', 'aif', 'wv', 'ape', 'tta', 'caf', 'au'],
  },
  { label: 'Broadcast', formats: ['ac3', 'eac3', 'dts', 'mka'] },
]
const DOCUMENT_FORMAT_GROUPS: FormatGroup[] = [
  { label: 'Core', formats: ['html', 'pdf', 'docx', 'md', 'odt', 'epub', 'rst'] },
  { label: 'Text & Markup', formats: ['txt', 'tex', 'adoc', 'org', 'rtf'] },
  { label: 'Presentations', formats: ['pptx', 's5', 'slidy', 'slideous', 'revealjs'] },
  { label: 'Data', formats: ['ipynb', 'docbook', 'json', 'xml'] },
  {
    label: 'Wiki & Other',
    formats: [
      'wiki', 'dokuwiki', 'textile', 'muse', 'man', 'ms',
      'tei', 'fb2', 'icml', 'jira', 'markua', 'zimwiki',
    ],
  },
]
const VIDEO_FORMAT_GROUPS: FormatGroup[] = [
  { label: 'Modern', formats: ['mp4', 'mkv', 'mov', 'webm'] },
  { label: 'Common', formats: ['avi', 'm4v', 'ogv', 'gif', 'ts', 'flv'] },
  { label: 'Mobile', formats: ['3gp', 'f4v', '3g2'] },
  { label: 'Broadcast & Pro', formats: ['mts', 'mxf', 'mpg', 'vob', 'wmv'] },
  { label: 'Legacy', formats: ['asf', 'divx', 'rm', 'apng'] },
]

const VIDEO_CODECS_FOR_FORMAT: Record<string, string[]> = {
  mp4: ['h264', 'h265'],
  mkv: ['h264', 'h265', 'vp9'],
  webm: ['vp9'],
  mov: ['h264', 'h265'],
  avi: ['h264', 'h265'],
  m4v: ['h264', 'h265'],
  ts: ['h264', 'h265'],
  flv: ['h264'],
  f4v: ['h264'],
  '3gp': ['h264', 'h265'],
  '3g2': ['h264', 'h265'],
  mts: ['h264', 'h265'],
  mxf: ['h264', 'h265'],
  divx: ['h264'],
}
// Formats using a fixed encoder — codec selector is hidden for these
const VIDEO_FIXED_CODEC_FORMATS = new Set(['gif', 'apng', 'ogv', 'mpg', 'vob', 'wmv', 'asf', 'rm'])

const AUDIO_LOSSLESS_FORMATS = ['flac', 'wav', 'aiff', 'aif', 'wv', 'ape', 'tta', 'caf', 'au']

const { t, locale: i18nLocale } = useI18n()
const activeCategory = ref<Category>('images')
const isDragover = ref(false)
const thumbErrors = ref<Record<string, true>>({})
const videoThumbs = ref<Record<string, string>>({})
const locale = ref<Locale>('en')
const updateVersion = ref<string | null>(null)
const updateDismissedVersion = ref<string | null>(
  localStorage.getItem('verto.updateDismissedVersion'),
)
const showUpdateBanner = computed(
  () => updateVersion.value !== null && updateVersion.value !== updateDismissedVersion.value,
)
const showSettings = ref(false)
const appVersion = ref('')

function onThumbError(id: string) {
  thumbErrors.value[id] = true
}

function toggleLocale() {
  locale.value = locale.value === 'en' ? 'fr' : 'en'
  i18nLocale.value = locale.value
}

function setLocale(l: Locale) {
  locale.value = l
  i18nLocale.value = l
}

async function checkForUpdates() {
  try {
    const result = await invoke<{ version: string; body: string | null } | null>(
      'check_for_updates',
    )
    if (result) updateVersion.value = result.version
  } catch {
    // updater not configured — silent in dev
  }
}

async function installUpdate() {
  try {
    await invoke('install_update')
  } catch (e) {
    console.error('Update failed:', e)
  }
}

const conversion = useConversionStore()
const settings = useSettingsStore()

const categoryName = computed(() => {
  if (activeCategory.value === 'images') return t('nav.images')
  if (activeCategory.value === 'documents') return t('nav.documents')
  if (activeCategory.value === 'audio') return t('nav.audio')
  if (activeCategory.value === 'history') return t('history.title')
  return t('nav.video')
})

const activeFormats = computed(() => IMAGE_FORMATS)

const activeFormatGroups = computed((): FormatGroup[] | null => {
  if (activeCategory.value === 'audio') return AUDIO_FORMAT_GROUPS
  if (activeCategory.value === 'video') return VIDEO_FORMAT_GROUPS
  if (activeCategory.value === 'documents') return DOCUMENT_FORMAT_GROUPS
  return null
})

const activeFileCategory = computed(() => {
  if (activeCategory.value === 'images') return 'image' as const
  if (activeCategory.value === 'documents') return 'document' as const
  if (activeCategory.value === 'audio') return 'audio' as const
  return 'video' as const
})

const availableCodecs = computed(() => VIDEO_CODECS_FOR_FORMAT[settings.outputFormat] ?? ['h264'])

const activeQueue = computed(() =>
  conversion.queue.filter((f) => f.category === activeFileCategory.value),
)

const selectedFileId = ref<string | null>(null)

watch(
  () => conversion.queue.filter((f) => f.category === 'video'),
  (videoFiles) => {
    for (const f of videoFiles) {
      if (!videoThumbs.value[f.id]) {
        invoke<string>('get_video_thumbnail', { inputPath: f.path })
          .then((data) => {
            videoThumbs.value[f.id] = data
          })
          .catch(() => {})
      }
    }
  },
  { deep: true },
)

const activeWaiting = computed(() => activeQueue.value.filter((f) => f.status === 'waiting'))

const queueSummary = computed(() => {
  const total = activeQueue.value.length
  const doneCount = activeQueue.value.filter((f) => f.status === 'done').length
  if (total === 0) return t('queue.no_files')
  return t('queue.summary', { done: doneCount, total, saved: formatBytes(conversion.totalSaved) })
})

watch(activeCategory, (cat) => {
  selectedFileId.value = null
  if (cat === 'images') settings.outputFormat = IMAGE_FORMATS[0]
  else if (cat === 'documents') settings.outputFormat = DOCUMENT_FORMAT_GROUPS[0].formats[0]
  else if (cat === 'audio') settings.outputFormat = AUDIO_FORMAT_GROUPS[0].formats[0]
  else if (cat !== 'history') settings.outputFormat = VIDEO_FORMAT_GROUPS[0].formats[0]
})

watch(
  () => settings.outputFormat,
  (fmt) => {
    if (activeCategory.value === 'video') {
      const codecs = VIDEO_CODECS_FOR_FORMAT[fmt] ?? ['h264']
      if (!codecs.includes(settings.videoCodec)) settings.videoCodec = codecs[0]
    }
  },
)

function setCategory(cat: Category) {
  activeCategory.value = cat
}

function timeAgo(ts: number): string {
  const s = Math.floor((Date.now() - ts) / 1000)
  if (s < 60) return locale.value === 'fr' ? "À l'instant" : 'Just now'
  const m = Math.floor(s / 60)
  if (m < 60) return locale.value === 'fr' ? `Il y a ${m} min` : `${m} min ago`
  const h = Math.floor(m / 60)
  return locale.value === 'fr' ? `Il y a ${h} h` : `${h} h ago`
}

function formatBytes(bytes: number): string {
  if (bytes <= 0) return '0 B'
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
}

function formatPercent(input: number, output: number): string {
  if (input === 0) return ''
  const ratio = (input - output) / input
  return `−${(ratio * 100).toFixed(0)}% · ${formatBytes(output)}`
}

function isLikelyDirectory(path: string): boolean {
  const name = path.split('/').pop() ?? ''
  return !name.includes('.')
}

async function openFilePicker() {
  let filters: { name: string; extensions: string[] }[]
  if (activeCategory.value === 'images') {
    filters = [
      {
        name: 'Images',
        extensions: [
          'jpg',
          'jpeg',
          'png',
          'webp',
          'bmp',
          'tiff',
          'tif',
          'gif',
          'avif',
          'heic',
          'heif',
          'ico',
          'psd',
          'dds',
          'exr',
          'qoi',
        ],
      },
    ]
  } else if (activeCategory.value === 'documents') {
    filters = [
      {
        name: 'Documents',
        extensions: [
          'md',
          'markdown',
          'docx',
          'html',
          'htm',
          'rst',
          'odt',
          'epub',
          'tex',
          'org',
          'txt',
          'csv',
          'wiki',
          'adoc',
          'asciidoc',
        ],
      },
    ]
  } else if (activeCategory.value === 'audio') {
    filters = [
      {
        name: 'Audio',
        extensions: [
          'mp3',
          'flac',
          'ogg',
          'wav',
          'aac',
          'm4a',
          'opus',
          'wma',
          'amr',
          'ape',
          'wv',
          'mka',
          'aiff',
          'aif',
          'caf',
        ],
      },
    ]
  } else {
    filters = [
      {
        name: 'Video',
        extensions: [
          'mp4',
          'mkv',
          'webm',
          'mov',
          'avi',
          'flv',
          'wmv',
          'm4v',
          'ts',
          'mts',
          'm2ts',
          'vob',
          '3gp',
          'ogv',
          'rm',
          'rmvb',
          'divx',
          'f4v',
        ],
      },
    ]
  }
  const selected = await open({ multiple: true, filters })
  if (!selected) return
  const paths = Array.isArray(selected) ? selected : [selected]
  conversion.addFiles(
    paths.map((p: string) => ({ name: p.split('/').pop() ?? p, path: p })),
    activeFileCategory.value,
  )
}

async function openFolderPicker() {
  const selected = await open({ directory: true, multiple: false })
  if (selected && typeof selected === 'string') {
    settings.outputDirectory = selected
  }
}

function applyPreset(preset: 'web' | 'print' | 'lossless') {
  if (preset === 'web') {
    settings.outputFormat = 'jpeg'
    settings.quality = 75
  } else if (preset === 'print') {
    settings.outputFormat = 'jpeg'
    settings.quality = 95
  } else {
    settings.outputFormat = 'png'
    settings.quality = 100
  }
}

function handleQueueAction(fileId: string, status: string) {
  if (status === 'error') {
    conversion.retryFile(fileId)
  } else if (status === 'waiting' || status === 'done') {
    conversion.removeFile(fileId)
  }
}

function handleKeydown(e: KeyboardEvent) {
  const el = e.target as HTMLElement
  const interactiveRoles = ['button', 'switch', 'checkbox', 'radio', 'textbox', 'slider']
  const isInteractive =
    ['INPUT', 'SELECT', 'TEXTAREA', 'BUTTON'].includes(el.tagName) ||
    interactiveRoles.includes(el.getAttribute('role') ?? '')

  if (e.key === 'Delete') {
    if (isInteractive) return
    if (selectedFileId.value) {
      conversion.removeFile(selectedFileId.value)
      selectedFileId.value = null
    }
  } else if (e.key === 'Enter') {
    if (isInteractive) return
    if (showSettings.value) return
    if (!conversion.isConverting && activeWaiting.value.length > 0) {
      conversion.convertAll(activeFileCategory.value)
    }
  } else if (e.key === 'Escape') {
    if (showSettings.value) {
      showSettings.value = false
      return
    }
    if (conversion.isConverting) {
      conversion.cancelConversion()
      return
    }
    selectedFileId.value = null
  }
}

function dismissUpdate() {
  if (updateVersion.value) {
    updateDismissedVersion.value = updateVersion.value
    localStorage.setItem('verto.updateDismissedVersion', updateVersion.value)
  }
}

// Open-folder toast timer
let openFolderTimer: ReturnType<typeof setTimeout> | null = null

watch(
  () => conversion.showOpenFolderPrompt,
  (visible) => {
    if (visible) {
      openFolderTimer = setTimeout(() => conversion.dismissOpenFolderPrompt(), 8000)
    } else {
      if (openFolderTimer) {
        clearTimeout(openFolderTimer)
        openFolderTimer = null
      }
    }
  },
)

async function handleOpenFolder() {
  if (conversion.lastOutputDirectory) {
    try {
      await invoke('open_output_folder', { path: conversion.lastOutputDirectory })
    } catch (e) {
      console.error('Failed to open folder:', e)
    }
  }
  conversion.dismissOpenFolderPrompt()
}

// Drag-and-drop via Tauri window events
let unlistenDrop: (() => void) | null = null

onMounted(async () => {
  window.addEventListener('keydown', handleKeydown)
  checkForUpdates()
  appVersion.value = await getVersion().catch(() => '—')
  const appWindow = getCurrentWebviewWindow()

  unlistenDrop = await appWindow.onDragDropEvent((event) => {
    if (event.payload.type === 'over') {
      isDragover.value = true
    } else if (event.payload.type === 'leave' || event.payload.type === 'cancelled') {
      isDragover.value = false
    } else if (event.payload.type === 'drop') {
      isDragover.value = false
      const paths: string[] = event.payload.paths ?? []
      for (const p of paths) {
        if (isLikelyDirectory(p)) {
          conversion.addDirectory(p, activeFileCategory.value)
        } else {
          conversion.addFiles(
            [{ name: p.split('/').pop() ?? p, path: p }],
            activeFileCategory.value,
          )
        }
      }
    }
  })
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
  unlistenDrop?.()
  if (openFolderTimer) clearTimeout(openFolderTimer)
})
</script>

<template>
  <div v-if="showUpdateBanner" class="update-banner" role="alert" aria-live="assertive">
    <span>{{ t('update.available', { version: updateVersion }) }}</span>
    <div class="update-actions">
      <button class="update-btn-install" @click="installUpdate">{{ t('update.install') }}</button>
      <button class="update-btn-dismiss" :aria-label="t('update.dismiss')" @click="dismissUpdate">
        ✕
      </button>
    </div>
  </div>
  <div
    v-if="conversion.showOpenFolderPrompt"
    class="open-folder-toast"
    role="alert"
    aria-live="polite"
  >
    <span>{{ t('open_folder.prompt') }}</span>
    <div class="update-actions">
      <button class="update-btn-install" @click="handleOpenFolder">
        {{ t('open_folder.open') }}
      </button>
      <button
        class="update-btn-dismiss"
        :aria-label="t('open_folder.dismiss')"
        @click="conversion.dismissOpenFolderPrompt"
      >
        ✕
      </button>
    </div>
  </div>

  <!-- SETTINGS MODAL -->
  <Teleport to="body">
    <div
      v-if="showSettings"
      class="settings-overlay"
      role="dialog"
      :aria-label="t('settings_page.title')"
      aria-modal="true"
      @click.self="showSettings = false"
      @keydown.escape="showSettings = false"
    >
      <div class="settings-modal">
        <div class="settings-header">
          <div class="settings-title">{{ t('settings_page.title') }}</div>
          <button
            class="settings-close"
            :aria-label="t('settings_page.close')"
            @click="showSettings = false"
          >
            <svg viewBox="0 0 24 24" aria-hidden="true">
              <line x1="18" y1="6" x2="6" y2="18" />
              <line x1="6" y1="6" x2="18" y2="18" />
            </svg>
          </button>
        </div>

        <div class="settings-body">
          <!-- Interface -->
          <div class="settings-section">
            <div class="settings-section-title">{{ t('settings_page.interface') }}</div>
            <div class="settings-row">
              <div class="settings-row-label">{{ t('settings_page.language') }}</div>
              <div class="settings-lang-btns">
                <button
                  class="lang-choice"
                  :class="{ active: locale === 'en' }"
                  @click="setLocale('en')"
                >
                  EN
                </button>
                <button
                  class="lang-choice"
                  :class="{ active: locale === 'fr' }"
                  @click="setLocale('fr')"
                >
                  FR
                </button>
              </div>
            </div>
          </div>

          <!-- Conversion defaults -->
          <div class="settings-section">
            <div class="settings-section-title">{{ t('settings_page.defaults') }}</div>

            <div class="settings-row">
              <div class="settings-row-label">{{ t('settings_page.default_output_dir') }}</div>
              <div class="settings-folder-row">
                <div class="settings-folder-path">
                  {{ settings.outputDirectory ?? t('settings.same_as_source') }}
                </div>
                <button class="settings-folder-browse" @click="openFolderPicker">
                  {{ t('settings.browse') }}
                </button>
                <button
                  v-if="settings.outputDirectory"
                  class="settings-folder-clear"
                  :aria-label="t('settings_page.reset_defaults')"
                  @click="settings.outputDirectory = null"
                >
                  ✕
                </button>
              </div>
            </div>

            <div class="settings-row">
              <label class="settings-row-label" for="sp-quality">
                {{ t('settings_page.default_quality') }}
                <span class="settings-val">{{ settings.quality }}%</span>
              </label>
              <input
                id="sp-quality"
                v-model.number="settings.quality"
                type="range"
                min="1"
                max="100"
                class="settings-slider"
              />
            </div>

            <div class="settings-row">
              <label class="settings-row-label" for="sp-bitrate">{{
                t('settings_page.default_bitrate')
              }}</label>
              <select id="sp-bitrate" v-model.number="settings.bitrate" class="settings-select">
                <option :value="64">64 kbps</option>
                <option :value="128">128 kbps</option>
                <option :value="192">192 kbps</option>
                <option :value="256">256 kbps</option>
                <option :value="320">320 kbps</option>
              </select>
            </div>

            <div class="settings-row">
              <label class="settings-row-label" for="sp-codec">{{
                t('settings_page.default_codec')
              }}</label>
              <select id="sp-codec" v-model="settings.videoCodec" class="settings-select">
                <option value="h264">{{ t('settings.codec_h264') }}</option>
                <option value="h265">{{ t('settings.codec_h265') }}</option>
                <option value="vp9">{{ t('settings.codec_vp9') }}</option>
              </select>
            </div>
          </div>

          <!-- Behavior -->
          <div class="settings-section">
            <div class="settings-section-title">{{ t('settings_page.behavior') }}</div>

            <div class="settings-row">
              <label class="settings-row-label" for="sp-metadata">{{
                t('settings.preserve_metadata')
              }}</label>
              <div
                id="sp-metadata"
                class="toggle"
                :class="{ on: settings.preserveMetadata }"
                role="switch"
                tabindex="0"
                :aria-checked="settings.preserveMetadata"
                @click="settings.preserveMetadata = !settings.preserveMetadata"
                @keydown.enter.space.prevent="
                  settings.preserveMetadata = !settings.preserveMetadata
                "
              ></div>
            </div>

            <div class="settings-row">
              <label class="settings-row-label" for="sp-overwrite">{{
                t('settings.overwrite_originals')
              }}</label>
              <div
                id="sp-overwrite"
                class="toggle"
                :class="{ on: settings.overwriteOriginals }"
                role="switch"
                tabindex="0"
                :aria-checked="settings.overwriteOriginals"
                @click="settings.overwriteOriginals = !settings.overwriteOriginals"
                @keydown.enter.space.prevent="
                  settings.overwriteOriginals = !settings.overwriteOriginals
                "
              ></div>
            </div>
          </div>

          <!-- About -->
          <div class="settings-section">
            <div class="settings-section-title">{{ t('settings_page.about') }}</div>

            <div class="settings-row">
              <div class="settings-row-label">{{ t('settings_page.version') }}</div>
              <div class="settings-about-val">v{{ appVersion }}</div>
            </div>

            <div class="settings-row">
              <div class="settings-row-label">{{ t('settings_page.source_code') }}</div>
              <a
                class="settings-link"
                href="https://github.com/Mvth1s/Verto"
                target="_blank"
                rel="noopener"
                >GitHub ↗</a
              >
            </div>

            <div class="settings-row">
              <div class="settings-row-label">{{ t('settings_page.changelog') }}</div>
              <a
                class="settings-link"
                href="https://github.com/Mvth1s/Verto/releases"
                target="_blank"
                rel="noopener"
                >Releases ↗</a
              >
            </div>
          </div>
        </div>
      </div>
    </div>
  </Teleport>

  <div class="shell" role="application" aria-label="Verto Desktop">
    <!-- SIDEBAR -->
    <aside class="sidebar" aria-label="Navigation">
      <div class="brand">
        <img src="/logo.jpeg" alt="Verto" class="brand-logo" />
      </div>

      <div class="nav-label" aria-hidden="true">{{ t('nav.convert') }}</div>
      <nav class="nav" :aria-label="t('nav.convert')">
        <div
          class="nav-item"
          :class="{ active: activeCategory === 'images' }"
          role="button"
          tabindex="0"
          :aria-pressed="activeCategory === 'images'"
          :aria-label="t('nav.images')"
          @click="setCategory('images')"
          @keydown.enter.space.prevent="setCategory('images')"
        >
          <svg viewBox="0 0 24 24" aria-hidden="true">
            <rect x="3" y="3" width="18" height="18" rx="2" />
            <circle cx="9" cy="9" r="2" />
            <path d="M21 15l-5-5L5 21" />
          </svg>
          <span>{{ t('nav.images') }}</span>
        </div>
        <div
          class="nav-item"
          :class="{ active: activeCategory === 'documents' }"
          role="button"
          tabindex="0"
          :aria-pressed="activeCategory === 'documents'"
          :aria-label="t('nav.documents')"
          @click="setCategory('documents')"
          @keydown.enter.space.prevent="setCategory('documents')"
        >
          <svg viewBox="0 0 24 24" aria-hidden="true">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
            <path d="M14 2v6h6" />
            <path d="M8 13h8M8 17h5" />
          </svg>
          <span>{{ t('nav.documents') }}</span>
        </div>
        <div
          class="nav-item"
          :class="{ active: activeCategory === 'audio' }"
          role="button"
          tabindex="0"
          :aria-pressed="activeCategory === 'audio'"
          :aria-label="t('nav.audio')"
          @click="setCategory('audio')"
          @keydown.enter.space.prevent="setCategory('audio')"
        >
          <svg viewBox="0 0 24 24" aria-hidden="true">
            <path d="M9 18V5l12-2v13" />
            <circle cx="6" cy="18" r="3" />
            <circle cx="18" cy="16" r="3" />
          </svg>
          <span>{{ t('nav.audio') }}</span>
        </div>
        <div
          class="nav-item"
          :class="{ active: activeCategory === 'video' }"
          role="button"
          tabindex="0"
          :aria-pressed="activeCategory === 'video'"
          :aria-label="t('nav.video')"
          @click="setCategory('video')"
          @keydown.enter.space.prevent="setCategory('video')"
        >
          <svg viewBox="0 0 24 24" aria-hidden="true">
            <rect x="2" y="6" width="14" height="12" rx="2" />
            <path d="M22 8l-6 4 6 4z" />
          </svg>
          <span>{{ t('nav.video') }}</span>
        </div>
      </nav>

      <div class="nav-divider" aria-hidden="true"></div>

      <div
        class="nav-item"
        :class="{ active: activeCategory === 'history' }"
        role="button"
        tabindex="0"
        :aria-pressed="activeCategory === 'history'"
        :aria-label="t('history.title')"
        @click="setCategory('history')"
        @keydown.enter.space.prevent="setCategory('history')"
      >
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <circle cx="12" cy="12" r="10" />
          <path d="M12 6v6l4 2" />
        </svg>
        <span>{{ t('history.title') }}</span>
        <span v-if="conversion.history.length > 0" class="nav-badge">{{
          conversion.history.length
        }}</span>
      </div>

      <div class="sidebar-spacer"></div>

      <div class="sidebar-footer">
        <div class="pill-version"><span class="dot" aria-hidden="true"></span>v0.4.0</div>
        <button
          class="icon-btn lang-btn"
          :aria-label="`Language: ${locale === 'en' ? 'English' : 'Français'}`"
          :title="locale === 'en' ? 'Switch to French' : 'Passer en anglais'"
          @click="toggleLocale"
        >
          {{ locale.toUpperCase() }}
        </button>
        <button
          class="icon-btn"
          :aria-label="t('settings_page.title')"
          @click="showSettings = true"
        >
          <svg viewBox="0 0 24 24" aria-hidden="true">
            <circle cx="12" cy="12" r="3" />
            <path
              d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09a1.65 1.65 0 0 0-1-1.51 1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09a1.65 1.65 0 0 0 1.51-1 1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33h0a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51h0a1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82v0a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"
            />
          </svg>
        </button>
      </div>
    </aside>

    <!-- MAIN -->
    <main class="main">
      <div class="main-header">
        <div class="main-title">{{ categoryName }}</div>
        <div class="main-sub" aria-live="polite" aria-atomic="true">
          {{ activeCategory === 'history' ? '' : queueSummary }}
        </div>
      </div>

      <!-- HISTORY VIEW -->
      <template v-if="activeCategory === 'history'">
        <div v-if="conversion.history.length === 0" class="history-empty">
          {{ t('history.empty') }}
        </div>
        <div v-else class="history-list">
          <div class="history-header">
            <button class="queue-clear" @click="conversion.clearHistory">
              {{ t('history.clear') }}
            </button>
          </div>
          <div v-for="item in conversion.history" :key="item.id" class="history-row">
            <div class="history-name">{{ item.name }}</div>
            <div class="history-formats">
              {{ item.inputFormat.toUpperCase() }} → {{ item.outputFormat.toUpperCase() }}
            </div>
            <div class="history-sizes">
              <span v-if="item.savedBytes > 0" class="history-saved">
                {{ t('history.saved', { saved: formatBytes(item.savedBytes) }) }}
              </span>
              <span v-else class="history-neutral">{{ t('history.same_size') }}</span>
              · {{ formatBytes(item.outputSize) }}
            </div>
            <div class="history-time">{{ timeAgo(item.convertedAt) }}</div>
          </div>
        </div>
      </template>

      <template v-else>
        <div
          class="dropzone"
          :class="{ dragover: isDragover }"
          role="button"
          tabindex="0"
          :aria-label="t('dropzone.title')"
          @click="openFilePicker"
          @keydown.enter.space.prevent="openFilePicker"
          @dragenter.prevent="isDragover = true"
          @dragover.prevent="isDragover = true"
          @dragleave.prevent="isDragover = false"
          @drop.prevent="isDragover = false"
        >
          <div class="drop-icon" aria-hidden="true">
            <svg viewBox="0 0 24 24">
              <path d="M12 3v12" />
              <path d="M7 8l5-5 5 5" />
              <path d="M5 21h14" />
            </svg>
          </div>
          <div>
            <div class="drop-title">{{ t('dropzone.title') }}</div>
            <div class="drop-sub" style="text-align: center; margin-top: 4px">
              {{ t('dropzone.sub') }}
            </div>
          </div>
          <div class="drop-kbd" aria-hidden="true"><kbd>⌘</kbd><kbd>O</kbd></div>
        </div>

        <div v-if="activeQueue.length > 0" class="queue" role="list" :aria-label="t('queue.title')">
          <div class="queue-header">
            <div class="queue-title" aria-hidden="true">{{ t('queue.title') }}</div>
            <div class="queue-actions">
              <button
                v-if="activeQueue.some((f) => f.status === 'done')"
                class="queue-clear"
                @click="conversion.clearDone"
              >
                {{ t('queue.clear_done') }}
              </button>
              <div class="queue-meta" aria-hidden="true">{{ queueSummary }}</div>
            </div>
          </div>

          <div
            v-for="file in activeQueue"
            :key="file.id"
            class="queue-row"
            :class="{
              'with-progress': file.status === 'converting',
              selected: selectedFileId === file.id,
            }"
            role="listitem"
            @click="selectedFileId = file.id"
          >
            <img
              v-if="activeCategory === 'images' && !thumbErrors[file.id]"
              :src="convertFileSrc(file.path)"
              class="thumb-img"
              :alt="file.inputFormat.toUpperCase()"
              loading="lazy"
              @error="onThumbError(file.id)"
            />
            <img
              v-else-if="activeCategory === 'video' && videoThumbs[file.id]"
              :src="videoThumbs[file.id]"
              class="thumb-img"
              :alt="file.inputFormat.toUpperCase()"
            />
            <div v-else class="ftype" aria-hidden="true">
              {{ file.inputFormat.toUpperCase().slice(0, 4) }}
            </div>
            <div class="fname">
              <span>{{ file.name }}</span>
              <span class="arrow" aria-hidden="true">→</span>
              <span class="to"
                >{{ file.name.replace(/\.[^/.]+$/, '') }}.{{ settings.outputFormat }}</span
              >
            </div>
            <div
              class="fstatus"
              :class="{
                done: file.status === 'done',
                progress: file.status === 'converting',
                error: file.status === 'error',
              }"
              aria-live="polite"
            >
              <template v-if="file.status === 'done'">
                {{ formatPercent(file.inputSize!, file.outputSize!) }}
              </template>
              <template v-else-if="file.status === 'converting'">{{
                t('queue.converting')
              }}</template>
              <template v-else-if="file.status === 'error'">{{ t('queue.error') }}</template>
              <template v-else>{{ t('queue.waiting') }}</template>
            </div>
            <div
              class="qaction"
              :class="{ check: file.status === 'done', retry: file.status === 'error' }"
              role="button"
              tabindex="0"
              :aria-label="
                file.status === 'error'
                  ? t('actions.retry', { error: file.error })
                  : file.status === 'waiting' || file.status === 'done'
                    ? t('actions.remove')
                    : undefined
              "
              @click="handleQueueAction(file.id, file.status)"
              @keydown.enter.space.prevent="handleQueueAction(file.id, file.status)"
            >
              <svg v-if="file.status === 'done'" viewBox="0 0 24 24" aria-hidden="true">
                <polyline points="20 6 9 17 4 12" />
              </svg>
              <svg v-else-if="file.status === 'converting'" viewBox="0 0 24 24" aria-hidden="true">
                <circle cx="12" cy="12" r="9" />
                <path d="M12 8v4l3 2" />
              </svg>
              <svg v-else-if="file.status === 'error'" viewBox="0 0 24 24" aria-hidden="true">
                <polyline points="1 4 1 10 7 10" />
                <path d="M3.51 15a9 9 0 1 0 .49-4.5" />
              </svg>
              <svg v-else viewBox="0 0 24 24" aria-hidden="true">
                <line x1="18" y1="6" x2="6" y2="18" />
                <line x1="6" y1="6" x2="18" y2="18" />
              </svg>
            </div>
            <div
              v-if="file.status === 'converting'"
              class="progress-row"
              role="progressbar"
              :aria-valuenow="file.progress"
              :aria-valuemin="0"
              :aria-valuemax="100"
              :aria-label="file.name"
            >
              <div
                class="bar"
                :class="{ indeterminate: file.progress == null }"
                :style="file.progress != null ? { width: file.progress + '%' } : {}"
              ></div>
            </div>
          </div>
        </div>
      </template>
    </main>

    <!-- RIGHT PANEL -->
    <aside v-if="activeCategory !== 'history'" class="panel" :aria-label="t('settings.title')">
      <div class="panel-title" aria-hidden="true">{{ t('settings.title') }}</div>

      <div class="field">
        <label class="field-label" for="format-select">{{ t('settings.format') }}</label>
        <select id="format-select" v-model="settings.outputFormat" class="select">
          <template v-if="activeFormatGroups">
            <optgroup
              v-for="group in activeFormatGroups"
              :key="group.label"
              :label="group.label"
            >
              <option v-for="fmt in group.formats" :key="fmt" :value="fmt">
                {{ fmt.toUpperCase() }}
              </option>
            </optgroup>
          </template>
          <template v-else>
            <option v-for="fmt in activeFormats" :key="fmt" :value="fmt">
              {{ fmt.toUpperCase() }}
            </option>
          </template>
        </select>
      </div>

      <div v-if="activeCategory === 'images'" class="field">
        <div id="presets-label" class="field-label">{{ t('settings.presets') }}</div>
        <div class="presets" role="group" :aria-labelledby="'presets-label'">
          <button class="preset-btn" @click="applyPreset('web')">
            {{ t('settings.preset_web') }}
          </button>
          <button class="preset-btn" @click="applyPreset('print')">
            {{ t('settings.preset_print') }}
          </button>
          <button class="preset-btn" @click="applyPreset('lossless')">
            {{ t('settings.preset_lossless') }}
          </button>
        </div>
      </div>

      <div v-if="activeCategory === 'audio'" class="field">
        <label class="field-label" for="bitrate-select">
          <span>{{ t('settings.bitrate') }}</span>
          <span class="val">{{ settings.bitrate }} kbps</span>
        </label>
        <select
          id="bitrate-select"
          v-model.number="settings.bitrate"
          class="select"
          :disabled="AUDIO_LOSSLESS_FORMATS.includes(settings.outputFormat)"
          :aria-describedby="
            AUDIO_LOSSLESS_FORMATS.includes(settings.outputFormat) ? 'bitrate-hint' : undefined
          "
        >
          <option :value="64">64 kbps</option>
          <option :value="128">128 kbps</option>
          <option :value="192">192 kbps</option>
          <option :value="256">256 kbps</option>
          <option :value="320">320 kbps</option>
        </select>
        <div
          v-if="AUDIO_LOSSLESS_FORMATS.includes(settings.outputFormat)"
          id="bitrate-hint"
          class="field-hint"
        >
          {{ t('settings.bitrate_lossy_only') }}
        </div>
      </div>

      <div
        v-if="activeCategory === 'video' && !VIDEO_FIXED_CODEC_FORMATS.has(settings.outputFormat)"
        class="field"
      >
        <label class="field-label" for="codec-select">{{ t('settings.codec') }}</label>
        <select id="codec-select" v-model="settings.videoCodec" class="select">
          <option v-for="c in availableCodecs" :key="c" :value="c">
            {{ t(`settings.codec_${c}`) }}
          </option>
        </select>
      </div>

      <div v-if="activeCategory === 'images'" class="field">
        <label class="field-label" for="quality-slider">
          <span>{{ t('settings.quality') }}</span>
          <span class="val">{{ settings.quality }}%</span>
        </label>
        <div class="slider-track-wrap">
          <input
            id="quality-slider"
            v-model.number="settings.quality"
            class="slider"
            type="range"
            min="1"
            max="100"
            :disabled="!['jpeg', 'jpg'].includes(settings.outputFormat)"
            :aria-valuemin="1"
            :aria-valuemax="100"
            :aria-valuenow="settings.quality"
            :aria-describedby="
              !['jpeg', 'jpg'].includes(settings.outputFormat) ? 'quality-hint' : undefined
            "
          />
        </div>
        <div class="ticks" aria-hidden="true"><span>1</span><span>50</span><span>100</span></div>
        <div
          v-if="!['jpeg', 'jpg'].includes(settings.outputFormat)"
          id="quality-hint"
          class="field-hint"
        >
          {{ t('settings.quality_jpeg_only') }}
        </div>
      </div>

      <div v-if="activeCategory === 'images' || activeCategory === 'video'" class="field">
        <div class="field-label">
          <span>{{ t('settings.resize') }}</span>
          <div
            class="toggle"
            :class="{ on: settings.resizeEnabled }"
            role="switch"
            tabindex="0"
            :aria-checked="settings.resizeEnabled"
            :aria-label="t('settings.resize')"
            @click="settings.resizeEnabled = !settings.resizeEnabled"
            @keydown.enter.space.prevent="settings.resizeEnabled = !settings.resizeEnabled"
          ></div>
        </div>
        <template v-if="settings.resizeEnabled">
          <div class="resize-row">
            <input
              v-model.number="settings.resizeWidth"
              class="resize-input"
              type="number"
              placeholder="W"
              min="1"
              :aria-label="`${t('settings.resize')} width`"
            />
            <button
              class="ratio-btn"
              :class="{ active: settings.keepAspectRatio }"
              :aria-pressed="settings.keepAspectRatio"
              :aria-label="
                settings.keepAspectRatio ? t('settings.ratio_locked') : t('settings.free_resize')
              "
              :title="
                settings.keepAspectRatio ? t('settings.ratio_locked') : t('settings.free_resize')
              "
              @click="settings.keepAspectRatio = !settings.keepAspectRatio"
            >
              <svg viewBox="0 0 24 24" aria-hidden="true">
                <rect x="3" y="11" width="18" height="10" rx="2" />
                <path v-if="settings.keepAspectRatio" d="M7 11V7a5 5 0 0 1 10 0v4" />
                <path v-else d="M7 11V7a5 5 0 0 1 4.9-5M17 11V7a5 5 0 0 0-1.9-3.9" />
              </svg>
            </button>
            <input
              v-model.number="settings.resizeHeight"
              class="resize-input"
              type="number"
              placeholder="H"
              min="1"
              :disabled="settings.keepAspectRatio"
              :aria-label="`${t('settings.resize')} height`"
            />
          </div>
          <div class="field-hint">
            {{ settings.keepAspectRatio ? t('settings.width_hint') : t('settings.exact_hint') }}
          </div>
        </template>
      </div>

      <div class="field">
        <div id="folder-label" class="field-label">{{ t('settings.output_folder') }}</div>
        <div class="folder-row" role="group" aria-labelledby="folder-label">
          <div
            class="folder-path"
            :title="settings.outputDirectory ?? t('settings.same_as_source')"
          >
            {{ settings.outputDirectory ?? t('settings.same_as_source') }}
          </div>
          <button class="folder-browse" @click="openFolderPicker">
            {{ t('settings.browse') }}
          </button>
        </div>
      </div>

      <div class="field">
        <div class="toggle-row">
          <label class="toggle-label" for="toggle-metadata">{{
            t('settings.preserve_metadata')
          }}</label>
          <div
            id="toggle-metadata"
            class="toggle"
            :class="{ on: settings.preserveMetadata }"
            role="switch"
            tabindex="0"
            :aria-checked="settings.preserveMetadata"
            :aria-label="t('settings.preserve_metadata')"
            @click="settings.preserveMetadata = !settings.preserveMetadata"
            @keydown.enter.space.prevent="settings.preserveMetadata = !settings.preserveMetadata"
          ></div>
        </div>
        <div class="toggle-row" style="margin-top: 8px">
          <label class="toggle-label" for="toggle-overwrite">{{
            t('settings.overwrite_originals')
          }}</label>
          <div
            id="toggle-overwrite"
            class="toggle"
            :class="{ on: settings.overwriteOriginals }"
            role="switch"
            tabindex="0"
            :aria-checked="settings.overwriteOriginals"
            :aria-label="t('settings.overwrite_originals')"
            @click="settings.overwriteOriginals = !settings.overwriteOriginals"
            @keydown.enter.space.prevent="
              settings.overwriteOriginals = !settings.overwriteOriginals
            "
          ></div>
        </div>
      </div>

      <div class="panel-spacer"></div>

      <div class="summary" aria-label="Conversion summary">
        <div class="summary-item">
          <div class="summary-key" aria-hidden="true">{{ t('summary.files') }}</div>
          <div class="summary-val" :aria-label="`${t('summary.files')}: ${activeQueue.length}`">
            {{ activeQueue.length }}
          </div>
        </div>
        <div class="summary-item">
          <div class="summary-key" aria-hidden="true">{{ t('summary.waiting') }}</div>
          <div class="summary-val" :aria-label="`${t('summary.waiting')}: ${activeWaiting.length}`">
            {{ activeWaiting.length }}
          </div>
        </div>
        <div class="summary-item">
          <div class="summary-key" aria-hidden="true">{{ t('summary.format') }}</div>
          <div class="summary-val">{{ settings.outputFormat.toUpperCase() }}</div>
        </div>
        <div class="summary-item">
          <div class="summary-key" aria-hidden="true">{{ t('summary.saved') }}</div>
          <div class="summary-val">{{ formatBytes(conversion.totalSaved) }}</div>
        </div>
      </div>

      <button
        v-if="!conversion.isConverting"
        class="btn-primary"
        :disabled="activeWaiting.length === 0"
        :aria-label="`${t('actions.convert')} ${activeWaiting.length} ${t('summary.waiting').toLowerCase()}`"
        @click="conversion.convertAll(activeFileCategory)"
      >
        <svg viewBox="0 0 24 24" aria-hidden="true"><polyline points="5 12 10 17 19 8" /></svg>
        <span>{{ t('actions.convert') }}</span>
        <span class="shortcut" aria-hidden="true">↵</span>
      </button>
      <button
        v-else
        class="btn-cancel"
        :aria-label="t('actions.cancel')"
        @click="conversion.cancelConversion"
      >
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <rect x="6" y="6" width="12" height="12" rx="1" />
        </svg>
        <span>{{ t('actions.cancel') }}</span>
        <span class="shortcut shortcut-cancel" aria-hidden="true">Esc</span>
      </button>
    </aside>
  </div>
</template>

<style>
:root {
  --bg: #18181b;
  --surface: #27272a;
  --surface-2: #1f1f22;
  --border: #3f3f46;
  --border-soft: #2e2e33;
  --text: #f4f4f5;
  --text-2: #a1a1aa;
  --text-3: #71717a;
  --accent: #10b981;
  --accent-soft: rgba(16, 185, 129, 0.12);
  --accent-soft-2: rgba(16, 185, 129, 0.06);
  --accent-bright: #34d399;
  --danger: #ef4444;
}

* {
  box-sizing: border-box;
}

/* Scrollbars — fines et discrètes */
::-webkit-scrollbar {
  width: 5px;
  height: 5px;
}
::-webkit-scrollbar-track {
  background: transparent;
}
::-webkit-scrollbar-thumb {
  background: var(--border);
  border-radius: 99px;
}
::-webkit-scrollbar-thumb:hover {
  background: var(--text-3);
}
* {
  scrollbar-width: thin;
  scrollbar-color: var(--border) transparent;
}
html,
body {
  margin: 0;
  padding: 0;
  background: var(--bg);
  color: var(--text);
  font-family: 'Inter', system-ui, sans-serif;
  -webkit-font-smoothing: antialiased;
  height: 100%;
  overflow: hidden;
}
#app {
  width: 100%;
  height: 100%;
}

/* Main grid */
.shell {
  display: grid;
  grid-template-columns: 200px 1fr 280px;
  height: 100vh;
  overflow: hidden;
}

/* Sidebar */
.sidebar {
  border-right: 1px solid var(--border-soft);
  display: flex;
  flex-direction: column;
  padding: 18px 12px 14px;
  background: #161618;
  overflow-y: auto;
  min-width: 0;
}
.brand {
  padding: 6px 10px 22px;
}
.brand-logo {
  width: 88px;
  height: 88px;
  border-radius: 10px;
  display: block;
  object-fit: cover;
}

.nav-label {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  color: var(--text-3);
  padding: 4px 10px 8px;
  font-weight: 500;
}

.nav {
  display: flex;
  flex-direction: column;
  gap: 2px;
}
.nav-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 10px;
  border-radius: 6px;
  color: var(--text-2);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  border: 1px solid transparent;
  transition:
    background 120ms ease,
    color 120ms ease;
  user-select: none;
}
.nav-item:hover {
  background: rgba(255, 255, 255, 0.03);
  color: var(--text);
}
.nav-item.active {
  background: var(--accent-soft);
  color: var(--accent-bright);
  border-color: rgba(16, 185, 129, 0.18);
}
.nav-item.active svg {
  stroke: var(--accent-bright);
}
.nav-item.disabled {
  color: var(--text-3);
  cursor: not-allowed;
  opacity: 0.55;
  pointer-events: none;
}
.nav-item .soon {
  margin-left: auto;
  font-size: 9px;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-3);
  border: 1px solid var(--border);
  padding: 2px 5px;
  border-radius: 3px;
  font-weight: 500;
}
.nav-item svg {
  width: 15px;
  height: 15px;
  stroke: currentColor;
  stroke-width: 1.6;
  fill: none;
  flex-shrink: 0;
}

.nav-divider {
  height: 1px;
  background: var(--border-soft);
  margin: 6px 8px;
}
.nav-badge {
  margin-left: auto;
  background: var(--accent);
  color: #0a0a0a;
  font-size: 10px;
  font-weight: 700;
  border-radius: 99px;
  padding: 1px 6px;
  line-height: 1.4;
}

/* History panel */
.history-empty {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  font-size: 13px;
}
.history-list {
  display: flex;
  flex-direction: column;
  gap: 1px;
  overflow-y: auto;
  flex: 1;
}
.history-header {
  display: flex;
  justify-content: flex-end;
  padding: 0 0 8px;
}
.history-row {
  display: grid;
  grid-template-columns: 1fr auto auto auto;
  align-items: center;
  gap: 10px;
  padding: 8px 10px;
  background: var(--surface);
  border-radius: 6px;
  font-size: 12px;
}
.history-name {
  font-size: 13px;
  color: var(--text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.history-formats {
  color: var(--text-muted);
  white-space: nowrap;
  font-family: monospace;
}
.history-sizes {
  color: var(--text-muted);
  white-space: nowrap;
}
.history-saved {
  color: var(--accent);
}
.history-neutral {
  color: var(--text-muted);
}
.history-time {
  color: var(--text-muted);
  white-space: nowrap;
  font-size: 11px;
}

.sidebar-spacer {
  flex: 1;
}
.sidebar-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 4px 0;
  border-top: 1px solid var(--border-soft);
}
.pill-version {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  font-family: 'JetBrains Mono', monospace;
  font-size: 10px;
  color: var(--text-3);
  padding: 4px 8px;
  border-radius: 4px;
}
.pill-version .dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--accent);
  box-shadow: 0 0 8px var(--accent);
}
.icon-btn {
  width: 28px;
  height: 28px;
  border-radius: 6px;
  display: grid;
  place-items: center;
  color: var(--text-2);
  cursor: pointer;
  border: 1px solid transparent;
  background: transparent;
}
.icon-btn:hover {
  background: rgba(255, 255, 255, 0.04);
  color: var(--text);
}
.icon-btn svg {
  width: 15px;
  height: 15px;
  stroke: currentColor;
  stroke-width: 1.6;
  fill: none;
}

/* Main */
.main {
  padding: 28px 32px;
  display: flex;
  flex-direction: column;
  min-width: 0;
  overflow-y: auto;
}
.main-header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  margin-bottom: 20px;
}
.main-title {
  font-size: 19px;
  font-weight: 600;
  letter-spacing: -0.015em;
}
.main-sub {
  font-size: 12px;
  color: var(--text-3);
  font-family: 'JetBrains Mono', monospace;
}

/* Drop zone */
.dropzone {
  border: 1.5px dashed var(--border);
  background: var(--surface-2);
  border-radius: 10px;
  flex: 1;
  min-height: 140px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 14px;
  color: var(--text-2);
  transition:
    border-color 160ms ease,
    background 160ms ease;
  cursor: pointer;
  position: relative;
  overflow: hidden;
}
.dropzone::before {
  content: '';
  position: absolute;
  inset: 0;
  background: radial-gradient(circle at 50% 40%, rgba(16, 185, 129, 0.05), transparent 60%);
  opacity: 0;
  transition: opacity 200ms ease;
  pointer-events: none;
}
.dropzone:hover,
.dropzone.dragover {
  border-color: var(--accent);
  background: rgba(16, 185, 129, 0.04);
  color: var(--text);
}
.dropzone:hover::before,
.dropzone.dragover::before {
  opacity: 1;
}
.drop-icon {
  width: 56px;
  height: 56px;
  border-radius: 12px;
  background: var(--surface);
  border: 1px solid var(--border-soft);
  display: grid;
  place-items: center;
  color: var(--text-2);
}
.drop-icon svg {
  width: 26px;
  height: 26px;
  stroke: currentColor;
  stroke-width: 1.5;
  fill: none;
}
.drop-title {
  font-size: 15px;
  font-weight: 500;
  color: var(--text);
  letter-spacing: -0.005em;
}
.drop-sub {
  font-size: 12px;
  color: var(--text-3);
}
.drop-kbd {
  font-family: 'JetBrains Mono', monospace;
  font-size: 11px;
  color: var(--text-3);
  display: inline-flex;
  gap: 4px;
  align-items: center;
}
.drop-kbd kbd {
  background: var(--surface);
  border: 1px solid var(--border-soft);
  padding: 1px 6px;
  border-radius: 4px;
  font-family: inherit;
  font-size: 10px;
  color: var(--text-2);
}

/* Queue */
.queue {
  margin-top: 18px;
  border: 1px solid var(--border-soft);
  border-radius: 8px;
  background: var(--surface);
  overflow: hidden;
  max-height: 320px;
  overflow-y: auto;
}
.queue-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 9px 14px;
  border-bottom: 1px solid var(--border-soft);
  background: rgba(255, 255, 255, 0.015);
  position: sticky;
  top: 0;
  z-index: 1;
}
.queue-title {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  color: var(--text-3);
  font-weight: 500;
}
.queue-meta {
  font-family: 'JetBrains Mono', monospace;
  font-size: 11px;
  color: var(--text-3);
}
.queue-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}
.queue-clear {
  background: transparent;
  border: none;
  color: var(--text-3);
  font: inherit;
  font-size: 11px;
  cursor: pointer;
  padding: 2px 6px;
  border-radius: 4px;
}
.queue-clear:hover {
  color: var(--text-2);
  background: rgba(255, 255, 255, 0.04);
}
.queue-row {
  display: grid;
  grid-template-columns: 32px 1fr 80px 22px;
  align-items: center;
  gap: 12px;
  padding: 10px 14px;
  font-size: 13px;
  border-top: 1px solid var(--border-soft);
}
.queue-row:first-of-type {
  border-top: none;
}
.queue-row .ftype {
  width: 32px;
  height: 32px;
  border-radius: 5px;
  background: var(--surface-2);
  display: grid;
  place-items: center;
  font-family: 'JetBrains Mono', monospace;
  font-size: 9px;
  color: var(--text-2);
  font-weight: 500;
}
.thumb-img {
  width: 32px;
  height: 32px;
  object-fit: cover;
  border-radius: 5px;
  display: block;
  border: 1px solid var(--border-soft);
}
.queue-row .fname {
  font-family: 'JetBrains Mono', monospace;
  font-size: 12px;
  color: var(--text);
  display: flex;
  align-items: center;
  gap: 10px;
  overflow: hidden;
  white-space: nowrap;
}
.queue-row .fname span:first-child {
  overflow: hidden;
  text-overflow: ellipsis;
}
.queue-row .fname .arrow {
  color: var(--text-3);
  flex-shrink: 0;
}
.queue-row .fname .to {
  color: var(--accent-bright);
  flex-shrink: 0;
}
.queue-row .fstatus {
  font-size: 11px;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-3);
  text-align: right;
}
.queue-row .fstatus.done {
  color: var(--accent-bright);
}
.queue-row .fstatus.progress {
  color: var(--text-2);
}
.queue-row .fstatus.error {
  color: var(--danger);
}
.qaction {
  display: grid;
  place-items: center;
  width: 22px;
  height: 22px;
  border-radius: 4px;
  color: var(--text-3);
  cursor: pointer;
}
.qaction:hover {
  background: rgba(255, 255, 255, 0.06);
  color: var(--text-2);
}
.qaction.check {
  color: var(--accent-bright);
}
.qaction svg {
  width: 14px;
  height: 14px;
  stroke: currentColor;
  stroke-width: 1.8;
  fill: none;
}

.progress-row {
  grid-column: 1 / -1;
  height: 3px;
  background: var(--border-soft);
  border-radius: 99px;
  overflow: hidden;
  margin-top: 4px;
}
.progress-row .bar {
  height: 100%;
  background: var(--accent);
  border-radius: 99px;
  transition: width 200ms ease;
  box-shadow: 0 0 8px rgba(16, 185, 129, 0.4);
}
.progress-row .bar.indeterminate {
  width: 40%;
  animation: indeterminate 1.4s ease-in-out infinite;
}
@keyframes indeterminate {
  0% {
    transform: translateX(-150%);
  }
  100% {
    transform: translateX(300%);
  }
}
.queue-row.with-progress {
  grid-template-columns: 32px 1fr 80px 22px;
  grid-template-rows: auto auto;
}
.queue-row.selected {
  background: var(--accent-soft-2);
  outline: 1px solid rgba(16, 185, 129, 0.18);
  outline-offset: -1px;
}

/* Right panel */
.panel {
  border-left: 1px solid var(--border-soft);
  padding: 28px 24px;
  display: flex;
  flex-direction: column;
  background: #161618;
  overflow-y: auto;
  min-width: 0;
}
.panel-title {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  color: var(--text-3);
  font-weight: 500;
  margin-bottom: 22px;
}
.field {
  margin-bottom: 22px;
}
.field-label {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: var(--text-2);
  margin-bottom: 8px;
  font-weight: 500;
}
.field-label .val {
  font-family: 'JetBrains Mono', monospace;
  color: var(--text);
}
.field-hint {
  font-size: 10px;
  color: var(--text-3);
  margin-top: 5px;
  font-style: italic;
}

.select {
  width: 100%;
  appearance: none;
  background: var(--surface);
  border: 1px solid var(--border);
  color: var(--text);
  border-radius: 6px;
  padding: 9px 12px;
  font: inherit;
  font-size: 13px;
  cursor: pointer;
  background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='10' height='6' viewBox='0 0 10 6' fill='none'><path d='M1 1L5 5L9 1' stroke='%23a1a1aa' stroke-width='1.5' stroke-linecap='round' stroke-linejoin='round'/></svg>");
  background-repeat: no-repeat;
  background-position: right 12px center;
  background-size: 10px;
  padding-right: 30px;
}
.select:focus {
  outline: none;
  border-color: var(--accent);
}

/* Slider */
.slider {
  -webkit-appearance: none;
  appearance: none;
  width: 100%;
  height: 4px;
  border-radius: 99px;
  background: var(--border);
  outline: none;
  cursor: pointer;
}
.slider:disabled {
  opacity: 0.35;
  cursor: not-allowed;
}
.slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 14px;
  height: 14px;
  border-radius: 50%;
  background: var(--accent);
  cursor: pointer;
  border: 2px solid var(--bg);
  box-shadow: 0 0 0 1px var(--accent);
}
.slider::-moz-range-thumb {
  width: 14px;
  height: 14px;
  border-radius: 50%;
  background: var(--accent);
  cursor: pointer;
  border: 2px solid var(--bg);
  box-shadow: 0 0 0 1px var(--accent);
}
.slider-track-wrap {
  position: relative;
  padding: 6px 0;
}
.ticks {
  display: flex;
  justify-content: space-between;
  font-family: 'JetBrains Mono', monospace;
  font-size: 10px;
  color: var(--text-3);
  margin-top: 6px;
}

/* Folder row */
.folder-row {
  display: flex;
  align-items: center;
  border: 1px solid var(--border);
  background: var(--surface);
  border-radius: 6px;
  overflow: hidden;
}
.folder-path {
  flex: 1;
  padding: 9px 12px;
  font-family: 'JetBrains Mono', monospace;
  font-size: 11px;
  color: var(--text-2);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.folder-browse {
  border: none;
  border-left: 1px solid var(--border);
  background: transparent;
  color: var(--text-2);
  font: inherit;
  font-size: 12px;
  padding: 9px 12px;
  cursor: pointer;
  font-weight: 500;
}
.folder-browse:hover {
  color: var(--text);
  background: rgba(255, 255, 255, 0.03);
}

/* Toggle */
.toggle-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 4px 0;
}
.toggle-label {
  font-size: 12px;
  color: var(--text-2);
}
.toggle {
  position: relative;
  width: 28px;
  height: 16px;
  background: var(--border);
  border-radius: 99px;
  cursor: pointer;
  transition: background 160ms ease;
  flex-shrink: 0;
}
.toggle::after {
  content: '';
  position: absolute;
  top: 2px;
  left: 2px;
  width: 12px;
  height: 12px;
  background: var(--text);
  border-radius: 50%;
  transition: transform 160ms ease;
}
.toggle.on {
  background: var(--accent);
}
.toggle.on::after {
  transform: translateX(12px);
}

.panel-spacer {
  flex: 1;
}

/* Summary */
.summary {
  border-top: 1px solid var(--border-soft);
  padding-top: 16px;
  margin-bottom: 14px;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px 16px;
}
.summary-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
}
.summary-key {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  color: var(--text-3);
  font-weight: 500;
}
.summary-val {
  font-family: 'JetBrains Mono', monospace;
  font-size: 12px;
  color: var(--text);
}

/* Convert button */
.btn-primary {
  width: 100%;
  padding: 12px 16px;
  background: var(--accent);
  color: #052e22;
  border: none;
  border-radius: 7px;
  font: inherit;
  font-size: 13px;
  font-weight: 600;
  letter-spacing: -0.005em;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  transition:
    background 120ms ease,
    transform 60ms ease,
    opacity 120ms ease;
  box-shadow: 0 4px 12px -2px rgba(16, 185, 129, 0.35);
}
.btn-primary:hover:not(:disabled) {
  background: var(--accent-bright);
}
.btn-primary:active:not(:disabled) {
  transform: translateY(1px);
}
.btn-primary:disabled {
  opacity: 0.4;
  cursor: not-allowed;
  box-shadow: none;
}
.btn-primary svg {
  width: 14px;
  height: 14px;
  stroke: currentColor;
  stroke-width: 2.2;
  fill: none;
}
.btn-primary .shortcut {
  margin-left: auto;
  font-family: 'JetBrains Mono', monospace;
  font-size: 10px;
  opacity: 0.6;
  background: rgba(5, 46, 34, 0.18);
  padding: 1px 6px;
  border-radius: 3px;
  font-weight: 500;
}

.btn-cancel {
  width: 100%;
  padding: 12px 16px;
  background: transparent;
  color: var(--text-2);
  border: 1px solid var(--border);
  border-radius: 7px;
  font: inherit;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  transition:
    background 120ms ease,
    color 120ms ease,
    border-color 120ms ease;
}
.btn-cancel:hover {
  background: rgba(239, 68, 68, 0.08);
  color: var(--danger);
  border-color: rgba(239, 68, 68, 0.35);
}
.btn-cancel svg {
  width: 14px;
  height: 14px;
  stroke: currentColor;
  stroke-width: 1.8;
  fill: currentColor;
}
.shortcut-cancel {
  margin-left: auto;
  font-family: 'JetBrains Mono', monospace;
  font-size: 10px;
  opacity: 0.5;
  background: rgba(255, 255, 255, 0.05);
  padding: 1px 6px;
  border-radius: 3px;
  font-weight: 500;
}

/* Quality presets */
.presets {
  display: flex;
  gap: 6px;
}
.preset-btn {
  flex: 1;
  background: var(--surface);
  border: 1px solid var(--border);
  color: var(--text-2);
  font: inherit;
  font-size: 11px;
  font-weight: 500;
  padding: 6px 0;
  border-radius: 5px;
  cursor: pointer;
  transition:
    background 120ms ease,
    color 120ms ease,
    border-color 120ms ease;
}
.preset-btn:hover {
  background: var(--accent-soft);
  border-color: rgba(16, 185, 129, 0.3);
  color: var(--accent-bright);
}

/* Resize */
.resize-row {
  display: flex;
  align-items: center;
  gap: 6px;
}
.resize-input {
  width: 0;
  flex: 1;
  background: var(--surface);
  border: 1px solid var(--border);
  color: var(--text);
  border-radius: 6px;
  padding: 8px 10px;
  font: inherit;
  font-size: 13px;
  font-family: 'JetBrains Mono', monospace;
}
.resize-input:disabled {
  opacity: 0.35;
  cursor: not-allowed;
}
.resize-input:focus {
  outline: none;
  border-color: var(--accent);
}
/* hide number spinner arrows */
.resize-input::-webkit-inner-spin-button,
.resize-input::-webkit-outer-spin-button {
  -webkit-appearance: none;
}
.resize-input[type='number'] {
  -moz-appearance: textfield;
}
.ratio-btn {
  width: 28px;
  height: 28px;
  flex-shrink: 0;
  display: grid;
  place-items: center;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 5px;
  cursor: pointer;
  color: var(--text-3);
  transition:
    background 120ms ease,
    color 120ms ease,
    border-color 120ms ease;
}
.ratio-btn.active {
  background: var(--accent-soft);
  border-color: rgba(16, 185, 129, 0.3);
  color: var(--accent-bright);
}
.ratio-btn svg {
  width: 13px;
  height: 13px;
  stroke: currentColor;
  stroke-width: 1.8;
  fill: none;
}

.qaction.retry {
  color: var(--accent-bright);
}
.qaction.retry:hover {
  background: var(--accent-soft);
  color: var(--accent-bright);
}

/* Language toggle button */
.lang-btn {
  font-family: 'JetBrains Mono', monospace;
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.05em;
  color: var(--text-3);
  padding: 0 6px;
  width: auto;
}
.lang-btn:hover {
  color: var(--accent-bright);
}

/* Keyboard focus ring — visible for all focusable elements */
:focus-visible {
  outline: 2px solid var(--accent);
  outline-offset: 2px;
  border-radius: 4px;
}

/* Update banner */
.update-banner {
  position: fixed;
  bottom: 16px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 100;
  background: var(--surface);
  border: 1px solid var(--accent);
  border-radius: 8px;
  padding: 10px 16px;
  display: flex;
  align-items: center;
  gap: 16px;
  font-size: 13px;
  color: var(--text);
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.4);
  white-space: nowrap;
}
.update-actions {
  display: flex;
  gap: 8px;
  align-items: center;
}
.update-btn-install {
  background: var(--accent);
  color: #052e22;
  border: none;
  border-radius: 5px;
  padding: 5px 12px;
  font: inherit;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
}
.update-btn-install:hover {
  background: var(--accent-bright);
}
.update-btn-dismiss {
  background: transparent;
  border: none;
  color: var(--text-3);
  cursor: pointer;
  font: inherit;
  font-size: 14px;
  padding: 2px 4px;
  border-radius: 3px;
}
.update-btn-dismiss:hover {
  color: var(--text);
  background: rgba(255, 255, 255, 0.06);
}

/* ── Settings modal ─────────────────────────────────────────────────── */
.settings-overlay {
  position: fixed;
  inset: 0;
  z-index: 200;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  backdrop-filter: blur(2px);
}
.settings-modal {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 12px;
  width: 480px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 24px 64px rgba(0, 0, 0, 0.5);
  overflow: hidden;
}
.settings-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18px 20px 16px;
  border-bottom: 1px solid var(--border-soft);
  flex-shrink: 0;
}
.settings-title {
  font-size: 15px;
  font-weight: 600;
  letter-spacing: -0.01em;
}
.settings-close {
  width: 28px;
  height: 28px;
  display: grid;
  place-items: center;
  background: transparent;
  border: none;
  color: var(--text-3);
  cursor: pointer;
  border-radius: 5px;
  transition:
    background 120ms,
    color 120ms;
}
.settings-close:hover {
  background: rgba(255, 255, 255, 0.06);
  color: var(--text);
}
.settings-close svg {
  width: 15px;
  height: 15px;
  stroke: currentColor;
  stroke-width: 1.8;
  fill: none;
}
.settings-body {
  overflow-y: auto;
  padding: 8px 0 16px;
}
.settings-section {
  padding: 12px 20px;
}
.settings-section + .settings-section {
  border-top: 1px solid var(--border-soft);
  margin-top: 4px;
  padding-top: 16px;
}
.settings-section-title {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  color: var(--text-3);
  font-weight: 500;
  margin-bottom: 12px;
}
.settings-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 6px 0;
}
.settings-row-label {
  font-size: 13px;
  color: var(--text-2);
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}
.settings-val {
  font-family: 'JetBrains Mono', monospace;
  font-size: 11px;
  color: var(--text-3);
}
.settings-slider {
  flex: 1;
  max-width: 160px;
  -webkit-appearance: none;
  appearance: none;
  height: 4px;
  border-radius: 99px;
  background: var(--border);
  outline: none;
  cursor: pointer;
}
.settings-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 14px;
  height: 14px;
  border-radius: 50%;
  background: var(--accent);
  cursor: pointer;
  border: 2px solid var(--bg);
  box-shadow: 0 0 0 1px var(--accent);
}
.settings-slider::-moz-range-thumb {
  width: 14px;
  height: 14px;
  border-radius: 50%;
  background: var(--accent);
  cursor: pointer;
  border: 2px solid var(--bg);
}
.settings-select {
  flex: 1;
  max-width: 160px;
  appearance: none;
  background: var(--surface-2);
  border: 1px solid var(--border);
  color: var(--text);
  border-radius: 6px;
  padding: 7px 10px;
  font: inherit;
  font-size: 12px;
  cursor: pointer;
  background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='10' height='6' viewBox='0 0 10 6' fill='none'><path d='M1 1L5 5L9 1' stroke='%23a1a1aa' stroke-width='1.5' stroke-linecap='round' stroke-linejoin='round'/></svg>");
  background-repeat: no-repeat;
  background-position: right 10px center;
  background-size: 10px;
  padding-right: 28px;
}
.settings-select:focus {
  outline: none;
  border-color: var(--accent);
}
.settings-lang-btns {
  display: flex;
  gap: 4px;
}
.lang-choice {
  padding: 5px 12px;
  border-radius: 5px;
  border: 1px solid var(--border);
  background: transparent;
  color: var(--text-2);
  font: inherit;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  font-family: 'JetBrains Mono', monospace;
  transition:
    background 120ms,
    color 120ms,
    border-color 120ms;
}
.lang-choice.active {
  background: var(--accent-soft);
  border-color: rgba(16, 185, 129, 0.3);
  color: var(--accent-bright);
}
.settings-folder-row {
  display: flex;
  align-items: center;
  flex: 1;
  min-width: 0;
  gap: 6px;
  background: var(--surface-2);
  border: 1px solid var(--border);
  border-radius: 6px;
  overflow: hidden;
}
.settings-folder-path {
  flex: 1;
  padding: 7px 10px;
  font-family: 'JetBrains Mono', monospace;
  font-size: 10px;
  color: var(--text-2);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.settings-folder-browse {
  border: none;
  border-left: 1px solid var(--border);
  background: transparent;
  color: var(--text-2);
  font: inherit;
  font-size: 11px;
  padding: 7px 10px;
  cursor: pointer;
  white-space: nowrap;
}
.settings-folder-browse:hover {
  color: var(--text);
  background: rgba(255, 255, 255, 0.03);
}
.settings-folder-clear {
  border: none;
  border-left: 1px solid var(--border);
  background: transparent;
  color: var(--text-3);
  font: inherit;
  font-size: 12px;
  padding: 7px 8px;
  cursor: pointer;
}
.settings-folder-clear:hover {
  color: var(--danger);
}
.settings-about-val {
  font-family: 'JetBrains Mono', monospace;
  font-size: 12px;
  color: var(--text-3);
}
.settings-link {
  font-size: 12px;
  color: var(--accent-bright);
  text-decoration: none;
  font-family: 'JetBrains Mono', monospace;
}
.settings-link:hover {
  text-decoration: underline;
}

/* Open-folder toast */
.open-folder-toast {
  position: fixed;
  bottom: 16px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 99;
  background: var(--surface);
  border: 1px solid var(--accent);
  border-radius: 8px;
  padding: 10px 16px;
  display: flex;
  align-items: center;
  gap: 16px;
  font-size: 13px;
  color: var(--text);
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.4);
  white-space: nowrap;
}
</style>
