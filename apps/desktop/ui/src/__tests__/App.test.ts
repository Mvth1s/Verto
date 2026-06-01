import { beforeEach, describe, expect, it, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import { createI18n } from 'vue-i18n'
import App from '../App.vue'
import en from '../i18n/en.json'
import fr from '../i18n/fr.json'

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue(null),
  convertFileSrc: (path: string) => `asset://localhost${path}`,
}))
vi.mock('@tauri-apps/api/webviewWindow', () => ({
  getCurrentWebviewWindow: () => ({
    onDragDropEvent: vi.fn().mockResolvedValue(() => {}),
  }),
}))
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn().mockResolvedValue(null),
}))

function buildI18n() {
  return createI18n({ legacy: false, locale: 'en', messages: { en, fr } })
}

function mountApp() {
  const pinia = createPinia()
  setActivePinia(pinia)
  return mount(App, {
    global: {
      plugins: [pinia, buildI18n()],
    },
  })
}

describe('App — category navigation', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('renders Images category by default', () => {
    const wrapper = mountApp()
    expect(wrapper.find('.main-title').text()).toBe('Images')
  })

  it('switches to Documents on nav click', async () => {
    const wrapper = mountApp()
    const docItem = wrapper.findAll('.nav-item').find((el) => el.text().includes('Documents'))
    await docItem!.trigger('click')
    expect(wrapper.find('.main-title').text()).toBe('Documents')
  })

  it('switches to Audio on nav click', async () => {
    const wrapper = mountApp()
    const audioItem = wrapper.findAll('.nav-item').find((el) => el.text().includes('Audio'))
    await audioItem!.trigger('click')
    expect(wrapper.find('.main-title').text()).toBe('Audio')
  })

  it('activates category via Enter key', async () => {
    const wrapper = mountApp()
    const docItem = wrapper.findAll('.nav-item').find((el) => el.text().includes('Documents'))
    await docItem!.trigger('keydown', { key: 'Enter' })
    expect(wrapper.find('.main-title').text()).toBe('Documents')
  })
})

describe('App — dropzone', () => {
  it('shows drop title in English', () => {
    const wrapper = mountApp()
    expect(wrapper.find('.drop-title').text()).toBe('Drop files here')
  })

  it('adds dragover class on dragenter', async () => {
    const wrapper = mountApp()
    await wrapper.find('.dropzone').trigger('dragenter')
    expect(wrapper.find('.dropzone').classes()).toContain('dragover')
  })

  it('removes dragover class on dragleave', async () => {
    const wrapper = mountApp()
    await wrapper.find('.dropzone').trigger('dragenter')
    await wrapper.find('.dropzone').trigger('dragleave')
    expect(wrapper.find('.dropzone').classes()).not.toContain('dragover')
  })
})

describe('App — i18n toggle', () => {
  it('switches UI to French when toggle clicked', async () => {
    const wrapper = mountApp()
    const langBtn = wrapper.find('.lang-btn')
    await langBtn.trigger('click')
    expect(wrapper.find('.drop-title').text()).toBe('Déposez vos fichiers ici')
  })

  it('toggles back to English on second click', async () => {
    const wrapper = mountApp()
    const langBtn = wrapper.find('.lang-btn')
    await langBtn.trigger('click')
    await langBtn.trigger('click')
    expect(wrapper.find('.drop-title').text()).toBe('Drop files here')
  })
})

describe('App — queue display', () => {
  it('does not render queue when empty', () => {
    const wrapper = mountApp()
    expect(wrapper.find('.queue').exists()).toBe(false)
  })

  it('shows queue summary with 0 files', () => {
    const wrapper = mountApp()
    expect(wrapper.find('.main-sub').text()).toBe('No files')
  })
})

describe('App — right panel', () => {
  it('shows Output settings title', () => {
    const wrapper = mountApp()
    expect(wrapper.find('.panel-title').text()).toBe('Output settings')
  })

  it('shows Presets only for images category', async () => {
    const wrapper = mountApp()
    expect(wrapper.find('.presets').exists()).toBe(true)
    const docItem = wrapper.findAll('.nav-item').find((el) => el.text().includes('Documents'))
    await docItem!.trigger('click')
    expect(wrapper.find('.presets').exists()).toBe(false)
  })

  it('shows Bitrate only for audio category', async () => {
    const wrapper = mountApp()
    expect(wrapper.find('#bitrate-select').exists()).toBe(false)
    const audioItem = wrapper.findAll('.nav-item').find((el) => el.text().includes('Audio'))
    await audioItem!.trigger('click')
    expect(wrapper.find('#bitrate-select').exists()).toBe(true)
  })

  it('Convert button is disabled when queue is empty', () => {
    const wrapper = mountApp()
    const btn = wrapper.find('.btn-primary')
    expect((btn.element as HTMLButtonElement).disabled).toBe(true)
  })
})

describe('App — update banner', () => {
  it('does not show update banner by default', () => {
    const wrapper = mountApp()
    expect(wrapper.find('.update-banner').exists()).toBe(false)
  })
})

describe('App — accessibility', () => {
  it('all nav items have role=button', () => {
    const wrapper = mountApp()
    const navItems = wrapper.findAll('.nav-item:not(.disabled)')
    navItems.forEach((item) => {
      expect(item.attributes('role')).toBe('button')
    })
  })

  it('dropzone has role=button', () => {
    const wrapper = mountApp()
    expect(wrapper.find('.dropzone').attributes('role')).toBe('button')
  })

  it('queue status has aria-live=polite', () => {
    const wrapper = mountApp()
    expect(wrapper.find('.main-sub').attributes('aria-live')).toBe('polite')
  })

  it('toggles have role=switch', () => {
    const wrapper = mountApp()
    const toggles = wrapper.findAll('[role="switch"]')
    expect(toggles.length).toBeGreaterThan(0)
  })
})
