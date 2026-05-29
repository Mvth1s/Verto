import { beforeEach, describe, expect, it } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useSettingsStore } from '../settings'

describe('useSettingsStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('has correct default values', () => {
    const store = useSettingsStore()
    expect(store.outputFormat).toBe('webp')
    expect(store.quality).toBe(85)
    expect(store.outputDirectory).toBeNull()
    expect(store.preserveMetadata).toBe(false)
    expect(store.overwriteOriginals).toBe(false)
  })

  it('can update outputFormat', () => {
    const store = useSettingsStore()
    store.outputFormat = 'png'
    expect(store.outputFormat).toBe('png')
  })

  it('can update quality', () => {
    const store = useSettingsStore()
    store.quality = 60
    expect(store.quality).toBe(60)
  })

  it('can set outputDirectory', () => {
    const store = useSettingsStore()
    store.outputDirectory = '/home/user/output'
    expect(store.outputDirectory).toBe('/home/user/output')
  })
})
