import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useSettingsStore = defineStore('settings', () => {
  const outputFormat = ref('webp')
  const quality = ref(85)
  const bitrate = ref(192)
  const videoCodec = ref('h264')
  const resizeEnabled = ref(false)
  const resizeWidth = ref<number | null>(null)
  const resizeHeight = ref<number | null>(null)
  const keepAspectRatio = ref(true)
  const outputDirectory = ref<string | null>(null)
  const preserveMetadata = ref(false)
  const overwriteOriginals = ref(false)

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
