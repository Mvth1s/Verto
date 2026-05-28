import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore } from './settings'

export type FileStatus = 'waiting' | 'converting' | 'done' | 'error'

export interface FileItem {
  id: string
  name: string
  path: string
  inputFormat: string
  inputSize: number
  status: FileStatus
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
  const isConverting = ref(false)
  const cancelRequested = ref(false)

  const waiting = computed(() => queue.value.filter((f) => f.status === 'waiting'))
  const done = computed(() => queue.value.filter((f) => f.status === 'done'))
  const totalSaved = computed(() => queue.value.reduce((acc, f) => acc + (f.savedBytes ?? 0), 0))

  function addFiles(files: { name: string; path: string }[]) {
    for (const f of files) {
      const ext = f.name.split('.').pop()?.toLowerCase() ?? ''
      const supported = ['jpg', 'jpeg', 'png', 'webp', 'bmp', 'tiff', 'tif', 'gif']
      if (!supported.includes(ext)) continue
      if (queue.value.some((item) => item.path === f.path)) continue

      queue.value.push({
        id: crypto.randomUUID(),
        name: f.name,
        path: f.path,
        inputFormat: ext,
        inputSize: 0,
        status: 'waiting',
      })
    }
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

  function cancelConversion() {
    cancelRequested.value = true
  }

  async function convertAll() {
    const settings = useSettingsStore()
    const toConvert = queue.value.filter((f) => f.status === 'waiting')
    if (toConvert.length === 0) return

    cancelRequested.value = false
    isConverting.value = true

    for (const file of toConvert) {
      if (cancelRequested.value) break

      file.status = 'converting'

      const outputPath = buildOutputPath(file.path, settings.outputFormat, settings.outputDirectory)

      try {
        const result = await invoke<ConversionResult>('convert_image', {
          inputPath: file.path,
          outputFormat: settings.outputFormat,
          quality: settings.quality,
          outputPath,
        })

        file.status = 'done'
        file.outputPath = result.output_path
        file.inputSize = result.input_size
        file.outputSize = result.output_size
        file.savedBytes = result.saved_bytes
      } catch (err) {
        file.status = 'error'
        file.error = String(err)
      }
    }

    isConverting.value = false
    cancelRequested.value = false
  }

  return {
    queue,
    isConverting,
    waiting,
    done,
    totalSaved,
    addFiles,
    removeFile,
    retryFile,
    clearDone,
    cancelConversion,
    convertAll,
  }
})
