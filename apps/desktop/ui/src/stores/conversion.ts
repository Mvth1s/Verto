import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/plugin-notification'
import { useSettingsStore } from './settings'

async function notify(title: string, body: string) {
  try {
    let granted = await isPermissionGranted()
    if (!granted) {
      granted = (await requestPermission()) === 'granted'
    }
    if (granted) sendNotification({ title, body })
  } catch {
    // notification not available — silent
  }
}

export type FileStatus = 'waiting' | 'converting' | 'done' | 'error'
export type FileCategory = 'image' | 'document' | 'audio' | 'video'

export interface HistoryItem {
  id: string
  name: string
  inputFormat: string
  outputFormat: string
  inputSize: number
  outputSize: number
  savedBytes: number
  outputPath: string
  convertedAt: number
  category: FileCategory
}

export interface FileItem {
  id: string
  name: string
  path: string
  inputFormat: string
  inputSize: number
  status: FileStatus
  category: FileCategory
  progress?: number
  outputPath?: string
  outputSize?: number
  savedBytes?: number
  error?: string
}

interface ConversionResult {
  output_path: string
  input_size: number
  output_size: number
  saved_bytes: number
}

const IMAGE_EXTENSIONS = [
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
]
const DOCUMENT_EXTENSIONS = [
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
]
const AUDIO_EXTENSIONS = [
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
]
const VIDEO_EXTENSIONS = [
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
]

function supportedExtensions(category: FileCategory): string[] {
  if (category === 'image') return IMAGE_EXTENSIONS
  if (category === 'document') return DOCUMENT_EXTENSIONS
  if (category === 'audio') return AUDIO_EXTENSIONS
  return VIDEO_EXTENSIONS
}

function buildOutputPath(
  inputPath: string,
  outputFormat: string,
  outputDirectory: string | null,
): string {
  const base = inputPath.replace(/\.[^/.]+$/, '')
  const name = base.split('/').pop() ?? base
  const dir = outputDirectory ?? inputPath.replace(/\/[^/]+$/, '')
  return `${dir}/${name}.${outputFormat}`
}

export const useConversionStore = defineStore('conversion', () => {
  const queue = ref<FileItem[]>([])
  const history = ref<HistoryItem[]>([])
  const isConverting = ref(false)
  const cancelRequested = ref(false)
  const showOpenFolderPrompt = ref(false)
  const lastOutputDirectory = ref<string | null>(null)

  listen<{ id: string; percent: number }>('conversion-progress', ({ payload }) => {
    const file = queue.value.find((f) => f.id === payload.id)
    if (file) file.progress = payload.percent
  })

  const waiting = computed(() => queue.value.filter((f) => f.status === 'waiting'))
  const done = computed(() => queue.value.filter((f) => f.status === 'done'))
  const totalSaved = computed(() => queue.value.reduce((acc, f) => acc + (f.savedBytes ?? 0), 0))

  function addFiles(files: { name: string; path: string }[], category: FileCategory = 'image') {
    const supported = supportedExtensions(category)
    for (const f of files) {
      const ext = f.name.split('.').pop()?.toLowerCase() ?? ''
      if (!supported.includes(ext)) continue
      if (queue.value.some((item) => item.path === f.path)) continue

      queue.value.push({
        id: crypto.randomUUID(),
        name: f.name,
        path: f.path,
        inputFormat: ext,
        inputSize: 0,
        status: 'waiting',
        category,
      })
    }
  }

  async function addDirectory(dirPath: string, category: FileCategory) {
    const supported = supportedExtensions(category)
    const paths = await invoke<string[]>('list_directory', { dirPath, recursive: true })
    const files = paths
      .filter((p) => supported.includes(p.split('.').pop()?.toLowerCase() ?? ''))
      .map((p) => ({ name: p.split('/').pop() ?? p, path: p }))
    addFiles(files, category)
  }

  function removeFile(id: string) {
    const idx = queue.value.findIndex((f) => f.id === id)
    if (idx !== -1) queue.value.splice(idx, 1)
  }

  function retryFile(id: string) {
    const file = queue.value.find((f) => f.id === id)
    if (file && file.status === 'error') {
      file.status = 'waiting'
      file.error = undefined
    }
  }

  function clearDone() {
    queue.value = queue.value.filter((f) => f.status !== 'done')
  }

  function clearHistory() {
    history.value = []
  }

  function cancelConversion() {
    cancelRequested.value = true
    invoke('cancel_conversion').catch(() => {})
  }

  function dismissOpenFolderPrompt() {
    showOpenFolderPrompt.value = false
  }

  async function convertAll(category: FileCategory = 'image') {
    const settings = useSettingsStore()
    const toConvert = queue.value.filter((f) => f.status === 'waiting' && f.category === category)
    if (toConvert.length === 0) return

    cancelRequested.value = false
    isConverting.value = true

    for (const file of toConvert) {
      if (cancelRequested.value) break

      file.status = 'converting'
      file.progress = undefined

      const outputPath = buildOutputPath(file.path, settings.outputFormat, settings.outputDirectory)

      try {
        let result: ConversionResult
        if (category === 'document') {
          result = await invoke<ConversionResult>('convert_document', {
            inputPath: file.path,
            outputFormat: settings.outputFormat,
            outputPath,
          })
        } else if (category === 'audio') {
          const isLossless = ['flac', 'wav', 'aiff', 'aif', 'wv', 'ape', 'tta', 'caf', 'au'].includes(
            settings.outputFormat,
          )
          result = await invoke<ConversionResult>('convert_audio', {
            inputPath: file.path,
            outputFormat: settings.outputFormat,
            bitrate: isLossless ? undefined : settings.bitrate,
            outputPath,
            fileId: file.id,
          })
        } else if (category === 'video') {
          result = await invoke<ConversionResult>('convert_video', {
            inputPath: file.path,
            outputFormat: settings.outputFormat,
            codec: settings.videoCodec,
            resolutionWidth: settings.resizeEnabled ? settings.resizeWidth : null,
            resolutionHeight:
              settings.resizeEnabled && !settings.keepAspectRatio ? settings.resizeHeight : null,
            outputPath,
            fileId: file.id,
          })
        } else {
          result = await invoke<ConversionResult>('convert_image', {
            inputPath: file.path,
            outputFormat: settings.outputFormat,
            quality: settings.quality,
            resizeWidth: settings.resizeEnabled ? settings.resizeWidth : null,
            resizeHeight:
              settings.resizeEnabled && !settings.keepAspectRatio ? settings.resizeHeight : null,
            outputPath,
          })
        }

        file.status = 'done'
        file.outputPath = result.output_path
        file.inputSize = result.input_size
        file.outputSize = result.output_size
        file.savedBytes = result.saved_bytes
        history.value.unshift({
          id: file.id,
          name: file.name,
          inputFormat: file.inputFormat,
          outputFormat: settings.outputFormat,
          inputSize: result.input_size,
          outputSize: result.output_size,
          savedBytes: result.saved_bytes,
          outputPath: result.output_path,
          convertedAt: Date.now(),
          category: file.category,
        })
      } catch (err) {
        if (cancelRequested.value) {
          file.status = 'waiting'
          file.progress = undefined
        } else {
          file.status = 'error'
          file.error = String(err)
        }
      }
    }

    isConverting.value = false
    cancelRequested.value = false

    const doneCount = queue.value.filter(
      (f) => f.category === category && f.status === 'done',
    ).length
    const errorCount = queue.value.filter(
      (f) => f.category === category && f.status === 'error',
    ).length
    if (doneCount > 0) {
      const saved = queue.value
        .filter((f) => f.category === category)
        .reduce((acc, f) => acc + (f.savedBytes ?? 0), 0)
      const savedMb = saved > 0 ? ` · ${(saved / 1024 / 1024).toFixed(1)} MB saved` : ''
      const errors = errorCount > 0 ? ` (${errorCount} error${errorCount > 1 ? 's' : ''})` : ''
      notify('Verto', `${doneCount} file${doneCount > 1 ? 's' : ''} converted${savedMb}${errors}`)

      const firstDone = queue.value.find((f) => f.category === category && f.status === 'done')
      const outputDir =
        settings.outputDirectory ?? firstDone?.outputPath?.replace(/\/[^/]+$/, '') ?? null
      if (outputDir) {
        lastOutputDirectory.value = outputDir
        showOpenFolderPrompt.value = true
      }
    }
  }

  return {
    queue,
    history,
    isConverting,
    showOpenFolderPrompt,
    lastOutputDirectory,
    waiting,
    done,
    totalSaved,
    addFiles,
    addDirectory,
    removeFile,
    retryFile,
    clearDone,
    clearHistory,
    cancelConversion,
    dismissOpenFolderPrompt,
    convertAll,
  }
})
