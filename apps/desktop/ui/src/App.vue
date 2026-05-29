<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { open } from '@tauri-apps/plugin-dialog'
import { useConversionStore } from './stores/conversion'
import { useSettingsStore } from './stores/settings'

type Category = 'images' | 'documents' | 'audio'

const IMAGE_FORMATS = ['webp', 'jpeg', 'png', 'bmp', 'tiff', 'gif']
const DOCUMENT_FORMATS = ['html', 'docx', 'md', 'epub', 'odt', 'rst']
const AUDIO_FORMATS = ['mp3', 'flac', 'ogg', 'wav', 'aac']

const activeCategory = ref<Category>('images')
const isDragover = ref(false)

const conversion = useConversionStore()
const settings = useSettingsStore()

const categoryName = computed(() => {
  if (activeCategory.value === 'images') return 'Images'
  if (activeCategory.value === 'documents') return 'Documents'
  return 'Audio'
})

const activeFormats = computed(() => {
  if (activeCategory.value === 'images') return IMAGE_FORMATS
  if (activeCategory.value === 'documents') return DOCUMENT_FORMATS
  return AUDIO_FORMATS
})

const activeFileCategory = computed(() => {
  if (activeCategory.value === 'images') return 'image' as const
  if (activeCategory.value === 'documents') return 'document' as const
  return 'audio' as const
})

const activeQueue = computed(() =>
  conversion.queue.filter((f) => f.category === activeFileCategory.value),
)

const activeWaiting = computed(() => activeQueue.value.filter((f) => f.status === 'waiting'))

const queueSummary = computed(() => {
  const total = activeQueue.value.length
  const doneCount = activeQueue.value.filter((f) => f.status === 'done').length
  if (total === 0) return 'No files'
  return `${doneCount} of ${total} complete · ${formatBytes(conversion.totalSaved)} saved`
})

watch(activeCategory, (cat) => {
  if (cat === 'images') settings.outputFormat = IMAGE_FORMATS[0]
  else if (cat === 'documents') settings.outputFormat = DOCUMENT_FORMATS[0]
  else settings.outputFormat = AUDIO_FORMATS[0]
})

function setCategory(cat: Category) {
  activeCategory.value = cat
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
      { name: 'Images', extensions: ['jpg', 'jpeg', 'png', 'webp', 'bmp', 'tiff', 'tif', 'gif'] },
    ]
  } else if (activeCategory.value === 'documents') {
    filters = [
      {
        name: 'Documents',
        extensions: ['md', 'markdown', 'docx', 'html', 'htm', 'rst', 'odt', 'epub'],
      },
    ]
  } else {
    filters = [{ name: 'Audio', extensions: ['mp3', 'flac', 'ogg', 'wav', 'aac', 'm4a', 'opus'] }]
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

function handleQueueAction(fileId: string, status: string) {
  if (status === 'error') {
    conversion.retryFile(fileId)
  } else if (status === 'waiting') {
    conversion.removeFile(fileId)
  }
}

// Drag-and-drop via Tauri window events
let unlistenDrop: (() => void) | null = null

onMounted(async () => {
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
  unlistenDrop?.()
})
</script>

<template>
  <div class="shell" role="application" aria-label="Verto Desktop">
    <!-- SIDEBAR -->
    <aside class="sidebar">
      <div class="brand">
        <img src="/logo.jpeg" alt="Verto" class="brand-logo" />
      </div>

      <div class="nav-label">Convert</div>
      <nav class="nav">
        <div
          class="nav-item"
          :class="{ active: activeCategory === 'images' }"
          @click="setCategory('images')"
        >
          <svg viewBox="0 0 24 24">
            <rect x="3" y="3" width="18" height="18" rx="2" />
            <circle cx="9" cy="9" r="2" />
            <path d="M21 15l-5-5L5 21" />
          </svg>
          <span>Images</span>
        </div>
        <div
          class="nav-item"
          :class="{ active: activeCategory === 'documents' }"
          @click="setCategory('documents')"
        >
          <svg viewBox="0 0 24 24">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
            <path d="M14 2v6h6" />
            <path d="M8 13h8M8 17h5" />
          </svg>
          <span>Documents</span>
        </div>
        <div
          class="nav-item"
          :class="{ active: activeCategory === 'audio' }"
          @click="setCategory('audio')"
        >
          <svg viewBox="0 0 24 24">
            <path d="M9 18V5l12-2v13" />
            <circle cx="6" cy="18" r="3" />
            <circle cx="18" cy="16" r="3" />
          </svg>
          <span>Audio</span>
        </div>
        <div class="nav-item disabled">
          <svg viewBox="0 0 24 24">
            <rect x="2" y="6" width="14" height="12" rx="2" />
            <path d="M22 8l-6 4 6 4z" />
          </svg>
          <span>Video</span>
          <span class="soon">Soon</span>
        </div>
      </nav>

      <div class="sidebar-spacer"></div>

      <div class="sidebar-footer">
        <div class="pill-version"><span class="dot"></span>v0.3.0</div>
        <button class="icon-btn" aria-label="Settings">
          <svg viewBox="0 0 24 24">
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
        <div class="main-sub">{{ queueSummary }}</div>
      </div>

      <div
        class="dropzone"
        :class="{ dragover: isDragover }"
        @click="openFilePicker"
        @dragenter.prevent="isDragover = true"
        @dragover.prevent="isDragover = true"
        @dragleave.prevent="isDragover = false"
        @drop.prevent="isDragover = false"
      >
        <div class="drop-icon">
          <svg viewBox="0 0 24 24">
            <path d="M12 3v12" />
            <path d="M7 8l5-5 5 5" />
            <path d="M5 21h14" />
          </svg>
        </div>
        <div>
          <div class="drop-title">Drop files here</div>
          <div class="drop-sub" style="text-align: center; margin-top: 4px">
            or click to browse · max 100 files at once
          </div>
        </div>
        <div class="drop-kbd"><kbd>⌘</kbd><kbd>O</kbd></div>
      </div>

      <div v-if="activeQueue.length > 0" class="queue">
        <div class="queue-header">
          <div class="queue-title">Queue</div>
          <div class="queue-actions">
            <button
              v-if="activeQueue.some((f) => f.status === 'done')"
              class="queue-clear"
              @click="conversion.clearDone"
            >
              Clear done
            </button>
            <div class="queue-meta">{{ queueSummary }}</div>
          </div>
        </div>

        <div
          v-for="file in activeQueue"
          :key="file.id"
          class="queue-row"
          :class="{ 'with-progress': file.status === 'converting' }"
        >
          <div class="ftype">{{ file.inputFormat.toUpperCase().slice(0, 4) }}</div>
          <div class="fname">
            <span>{{ file.name }}</span>
            <span class="arrow">→</span>
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
          >
            <template v-if="file.status === 'done'">
              {{ formatPercent(file.inputSize!, file.outputSize!) }}
            </template>
            <template v-else-if="file.status === 'converting'"> converting… </template>
            <template v-else-if="file.status === 'error'"> error </template>
            <template v-else> waiting </template>
          </div>
          <div
            class="qaction"
            :class="{ check: file.status === 'done', retry: file.status === 'error' }"
            role="button"
            :title="
              file.status === 'error'
                ? `Retry · ${file.error}`
                : file.status === 'waiting'
                  ? 'Remove'
                  : undefined
            "
            @click="handleQueueAction(file.id, file.status)"
          >
            <svg v-if="file.status === 'done'" viewBox="0 0 24 24">
              <polyline points="20 6 9 17 4 12" />
            </svg>
            <svg v-else-if="file.status === 'converting'" viewBox="0 0 24 24">
              <circle cx="12" cy="12" r="9" />
              <path d="M12 8v4l3 2" />
            </svg>
            <svg v-else-if="file.status === 'error'" viewBox="0 0 24 24">
              <polyline points="1 4 1 10 7 10" />
              <path d="M3.51 15a9 9 0 1 0 .49-4.5" />
            </svg>
            <svg v-else viewBox="0 0 24 24">
              <line x1="18" y1="6" x2="6" y2="18" />
              <line x1="6" y1="6" x2="18" y2="18" />
            </svg>
          </div>
          <div v-if="file.status === 'converting'" class="progress-row">
            <div class="bar indeterminate"></div>
          </div>
        </div>
      </div>
    </main>

    <!-- RIGHT PANEL -->
    <aside class="panel">
      <div class="panel-title">Output settings</div>

      <div class="field">
        <div class="field-label">Format</div>
        <select v-model="settings.outputFormat" class="select">
          <option v-for="fmt in activeFormats" :key="fmt" :value="fmt">
            {{ fmt.toUpperCase() }}
          </option>
        </select>
      </div>

      <div v-if="activeCategory === 'audio'" class="field">
        <div class="field-label">
          <span>Bitrate</span>
          <span class="val">{{ settings.bitrate }} kbps</span>
        </div>
        <select
          v-model.number="settings.bitrate"
          class="select"
          :disabled="['flac', 'wav'].includes(settings.outputFormat)"
        >
          <option :value="64">64 kbps</option>
          <option :value="128">128 kbps</option>
          <option :value="192">192 kbps</option>
          <option :value="256">256 kbps</option>
          <option :value="320">320 kbps</option>
        </select>
        <div v-if="['flac', 'wav'].includes(settings.outputFormat)" class="field-hint">
          Bitrate applies to lossy formats only
        </div>
      </div>

      <div v-if="activeCategory === 'images'" class="field">
        <div class="field-label">
          <span>Quality</span>
          <span class="val">{{ settings.quality }}%</span>
        </div>
        <div class="slider-track-wrap">
          <input
            v-model.number="settings.quality"
            class="slider"
            type="range"
            min="1"
            max="100"
            :disabled="!['jpeg', 'jpg'].includes(settings.outputFormat)"
          />
        </div>
        <div class="ticks"><span>1</span><span>50</span><span>100</span></div>
        <div v-if="!['jpeg', 'jpg'].includes(settings.outputFormat)" class="field-hint">
          Quality applies to JPEG only
        </div>
      </div>

      <div class="field">
        <div class="field-label">Output folder</div>
        <div class="folder-row">
          <div class="folder-path">
            {{ settings.outputDirectory ?? 'Same as source' }}
          </div>
          <button class="folder-browse" @click="openFolderPicker">Browse</button>
        </div>
      </div>

      <div class="field">
        <div class="toggle-row">
          <div class="toggle-label">Preserve metadata</div>
          <div
            class="toggle"
            :class="{ on: settings.preserveMetadata }"
            @click="settings.preserveMetadata = !settings.preserveMetadata"
          ></div>
        </div>
        <div class="toggle-row" style="margin-top: 8px">
          <div class="toggle-label">Overwrite originals</div>
          <div
            class="toggle"
            :class="{ on: settings.overwriteOriginals }"
            @click="settings.overwriteOriginals = !settings.overwriteOriginals"
          ></div>
        </div>
      </div>

      <div class="panel-spacer"></div>

      <div class="summary">
        <div class="summary-item">
          <div class="summary-key">Files</div>
          <div class="summary-val">{{ activeQueue.length }}</div>
        </div>
        <div class="summary-item">
          <div class="summary-key">Waiting</div>
          <div class="summary-val">{{ activeWaiting.length }}</div>
        </div>
        <div class="summary-item">
          <div class="summary-key">Format</div>
          <div class="summary-val">{{ settings.outputFormat.toUpperCase() }}</div>
        </div>
        <div class="summary-item">
          <div class="summary-key">Saved</div>
          <div class="summary-val">{{ formatBytes(conversion.totalSaved) }}</div>
        </div>
      </div>

      <button
        v-if="!conversion.isConverting"
        class="btn-primary"
        :disabled="activeWaiting.length === 0"
        @click="conversion.convertAll(activeFileCategory)"
      >
        <svg viewBox="0 0 24 24"><polyline points="5 12 10 17 19 8" /></svg>
        <span>Convert</span>
        <span class="shortcut">⌘↵</span>
      </button>
      <button v-else class="btn-cancel" @click="conversion.cancelConversion">
        <svg viewBox="0 0 24 24">
          <rect x="6" y="6" width="12" height="12" rx="1" />
        </svg>
        <span>Cancel</span>
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
  grid-template-columns: 24px 1fr 80px 22px;
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
  width: 24px;
  height: 24px;
  border-radius: 4px;
  background: var(--surface-2);
  display: grid;
  place-items: center;
  font-family: 'JetBrains Mono', monospace;
  font-size: 9px;
  color: var(--text-2);
  font-weight: 500;
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
  grid-template-columns: 24px 1fr 80px 22px;
  grid-template-rows: auto auto;
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

.qaction.retry {
  color: var(--accent-bright);
}
.qaction.retry:hover {
  background: var(--accent-soft);
  color: var(--accent-bright);
}
</style>
