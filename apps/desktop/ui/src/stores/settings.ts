import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

function load<T>(key: string, fallback: T): T {
  try {
    const v = localStorage.getItem(key)
    return v !== null ? (JSON.parse(v) as T) : fallback
  } catch {
    return fallback
  }
}

function persist<T>(key: string, value: T) {
  localStorage.setItem(key, JSON.stringify(value))
}

export const useSettingsStore = defineStore('settings', () => {
  const outputFormat = ref('webp')
  const quality = ref(load('verto.quality', 85))
  const bitrate = ref(load('verto.bitrate', 192))
  const videoCodec = ref(load('verto.videoCodec', 'h264'))
  const resizeEnabled = ref(false)
  const resizeWidth = ref<number | null>(null)
  const resizeHeight = ref<number | null>(null)
  const keepAspectRatio = ref(true)
  const outputDirectory = ref<string | null>(load('verto.outputDirectory', null))
  const preserveMetadata = ref(load('verto.preserveMetadata', false))
  const overwriteOriginals = ref(load('verto.overwriteOriginals', false))

  watch(quality, (v) => persist('verto.quality', v))
  watch(bitrate, (v) => persist('verto.bitrate', v))
  watch(videoCodec, (v) => persist('verto.videoCodec', v))
  watch(outputDirectory, (v) => persist('verto.outputDirectory', v))
  watch(preserveMetadata, (v) => persist('verto.preserveMetadata', v))
  watch(overwriteOriginals, (v) => persist('verto.overwriteOriginals', v))

  return {
    outputFormat,
    quality,
    bitrate,
    videoCodec,
    resizeEnabled,
    resizeWidth,
    resizeHeight,
    keepAspectRatio,
    outputDirectory,
    preserveMetadata,
    overwriteOriginals,
  }
})
