import { ref } from 'vue'

export type ConsentChoice = 'granted' | 'denied'

// Also read by the inline Google tag injected in index.html (vite.config.ts)
const CONSENT_KEY = 'verto_consent'

declare global {
  interface Window {
    gtag?: (...args: unknown[]) => void
  }
}

function readStoredChoice(): ConsentChoice | null {
  try {
    const value = localStorage.getItem(CONSENT_KEY)
    return value === 'granted' || value === 'denied' ? value : null
  } catch {
    return null
  }
}

// Shared between the banner and the footer link
const consent = ref<ConsentChoice | null>(readStoredChoice())
const bannerVisible = ref(consent.value === null)

// Removes the _ga cookies left by a previous consent
function clearGaCookies() {
  const domain = window.location.hostname
  for (const cookie of document.cookie.split(';')) {
    const name = cookie.split('=')[0].trim()
    if (name.startsWith('_ga')) {
      document.cookie = `${name}=; Max-Age=0; path=/`
      document.cookie = `${name}=; Max-Age=0; path=/; domain=.${domain}`
    }
  }
}

function setConsent(choice: ConsentChoice) {
  consent.value = choice
  bannerVisible.value = false
  try {
    localStorage.setItem(CONSENT_KEY, choice)
  } catch {
    // storage unavailable: the choice holds for this visit only
  }

  // gtag is absent in dev builds, where the Google tag is not injected
  window.gtag?.('consent', 'update', { analytics_storage: choice })
  if (choice === 'denied') clearGaCookies()
}

function openConsentBanner() {
  bannerVisible.value = true
}

export function useAnalytics() {
  return { consent, bannerVisible, setConsent, openConsentBanner }
}
