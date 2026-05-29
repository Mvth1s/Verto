import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useSettingsStore = defineStore('settings', () => {
  const outputFormat = ref('webp')
  const quality = ref(85)
  const bitrate = ref(192)
  const outputDirectory = ref<string | null>(null)
  const preserveMetadata = ref(false)
  const overwriteOriginals = ref(false)

  return { outputFormat, quality, bitrate, outputDirectory, preserveMetadata, overwriteOriginals }
})
