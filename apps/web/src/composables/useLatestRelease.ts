import { ref, computed } from 'vue'

export interface GithubAsset {
  name: string
  browser_download_url: string
  size: number
}

interface GithubRelease {
  tag_name: string
  assets: GithubAsset[]
}

export const RELEASES_URL = 'https://github.com/Mvth1s/Verto/releases/latest'
const API_URL = 'https://api.github.com/repos/Mvth1s/Verto/releases/latest'
const CACHE_KEY = 'verto_release'
const CACHE_TTL = 3_600_000 // 1 hour

// Shared between the hero and the download section, fetched once per page load
const release = ref<GithubRelease | null>(null)

async function loadLatestRelease() {
  // Serve from cache when fresh
  try {
    const raw = localStorage.getItem(CACHE_KEY)
    if (raw) {
      const { data, ts } = JSON.parse(raw) as { data: GithubRelease; ts: number }
      if (Date.now() - ts < CACHE_TTL && data.tag_name && data.assets?.length) {
        release.value = data
        return
      }
    }
  } catch {
    // ignore, fallback to API or static links
  }

  // Fetch from GitHub API
  try {
    const res = await fetch(API_URL, { headers: { Accept: 'application/vnd.github+json' } })
    if (res.ok) {
      const data: GithubRelease = await res.json()
      if (data.tag_name) {
        release.value = data
        localStorage.setItem(CACHE_KEY, JSON.stringify({ data, ts: Date.now() }))
      }
    }
  } catch {
    // ignore, fallback to API or static links
  }
}

const version = computed(() => release.value?.tag_name ?? null)

function findAssets(pred: (name: string) => boolean): GithubAsset[] {
  return (
    release.value?.assets.filter(
      (a) =>
        pred(a.name) &&
        !a.name.endsWith('.sig') &&
        !a.name.endsWith('.tar.gz') &&
        !a.name.endsWith('.zip'),
    ) ?? []
  )
}

export function useLatestRelease() {
  return { release, version, findAssets, loadLatestRelease }
}
