import { createI18n } from 'vue-i18n'
import en from './en.json'
import fr from './fr.json'

export type Locale = 'en' | 'fr'

export const i18n = createI18n({
  legacy: false,
  locale: 'en',
  fallbackLocale: 'en',
  messages: { en, fr },
})
