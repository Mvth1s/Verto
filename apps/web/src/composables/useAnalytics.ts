import { ref } from 'vue'

export type ConsentChoice = 'granted' | 'denied'

const CONSENT_KEY = 'verto_consent'
const GA_ID = 'G-VFWPEK2CB8'

declare global {
  interface Window {
    dataLayer?: unknown[]
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

// The Google tag is only downloaded once the visitor has accepted: nothing is sent to
// Google before that. Production builds only, to keep dev out of the stats.
function loadGoogleTag() {
  if (!import.meta.env.PROD || window.gtag) return
  window.dataLayer = window.dataLayer || []
  window.gtag = function gtag() {
    // gtag.js expects the `arguments` object, not an array
    // eslint-disable-next-line prefer-rest-params
    window.dataLayer!.push(arguments)
  }
  window.gtag('consent', 'default', {
    analytics_storage: 'granted',
    ad_storage: 'denied',
    ad_user_data: 'denied',
    ad_personalization: 'denied',
  })
  window.gtag('js', new Date())
  window.gtag('config', GA_ID)

  const script = document.createElement('script')
  script.async = true
  script.src = `https://www.googletagmanager.com/gtag/js?id=${GA_ID}`
  document.head.appendChild(script)
}

if (consent.value === 'granted') loadGoogleTag()

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

  if (choice === 'granted') {
    loadGoogleTag()
    window.gtag?.('consent', 'update', { analytics_storage: 'granted' })
  } else {
    // gtag only exists if consent was granted earlier in this visit
    window.gtag?.('consent', 'update', { analytics_storage: 'denied' })
    clearGaCookies()
  }
}

function openConsentBanner() {
  bannerVisible.value = true
}

export function useAnalytics() {
  return { consent, bannerVisible, setConsent, openConsentBanner }
}
