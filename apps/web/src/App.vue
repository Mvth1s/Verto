<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

interface GithubAsset {
  name: string
  browser_download_url: string
  size: number
}

interface GithubRelease {
  tag_name: string
  assets: GithubAsset[]
}

const RELEASES_URL = 'https://github.com/Mvth1s/Verto/releases/latest'
const release = ref<GithubRelease | null>(null)

onMounted(async () => {
  try {
    const res = await fetch('https://api.github.com/repos/Mvth1s/Verto/releases/latest', {
      headers: { Accept: 'application/vnd.github+json' },
    })
    if (res.ok) release.value = await res.json()
  } catch {
    // silently fallback to releases page links
  }
})

const version = computed(() => release.value?.tag_name ?? null)

function assets(pred: (name: string) => boolean): GithubAsset[] {
  return (
    release.value?.assets.filter(
      (a) => pred(a.name) && !a.name.endsWith('.sig') && !a.name.endsWith('.tar.gz'),
    ) ?? []
  )
}

function sizeMB(bytes: number): string {
  return `~${Math.round(bytes / 1024 / 1024)} MB`
}

const linuxAssets = computed(() =>
  assets((n) => n.endsWith('.AppImage') || n.endsWith('.deb') || n.endsWith('.rpm')),
)

const windowsAssets = computed(() =>
  assets((n) => n.endsWith('-setup.exe') || (n.endsWith('.msi') && !n.endsWith('.msi.zip'))),
)

const macosAssets = computed(() => assets((n) => n.endsWith('.dmg')))

function ext(name: string): string {
  if (name.endsWith('.AppImage')) return '.AppImage'
  if (name.endsWith('-setup.exe')) return '.exe'
  if (name.endsWith('.msi')) return '.msi'
  if (name.endsWith('.deb')) return '.deb'
  if (name.endsWith('.rpm')) return '.rpm'
  if (name.endsWith('.dmg')) return '.dmg'
  return name.split('.').pop() ?? name
}
</script>

<template>
  <header class="nav">
    <div class="container nav-inner">
      <a class="brand" href="#" style="text-decoration: none; color: inherit">
        <img src="/logo.jpeg" alt="Verto" class="brand-logo" />
        <div class="brand-name">Verto</div>
      </a>
      <div class="nav-right">
        <a class="nav-link" href="#features">Features</a>
        <a class="nav-link" href="#download">Download</a>
        <a class="nav-link" href="https://github.com/Mvth1s/Verto" target="_blank" rel="noopener">
          <svg viewBox="0 0 24 24">
            <path
              d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.4 3-.405 1.02.005 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"
            />
          </svg>
          GitHub
          <span class="star-count">★ 2.4k</span>
        </a>
        <a class="btn btn-primary" href="#download">
          <svg viewBox="0 0 24 24">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
            <polyline points="7 10 12 15 17 10" />
            <line x1="12" y1="15" x2="12" y2="3" />
          </svg>
          Download
        </a>
      </div>
    </div>
  </header>

  <!-- HERO -->
  <section class="hero">
    <div class="container hero-grid">
      <div>
        <div class="eyebrow">
          <span class="dot"></span>Open source<span class="sep">·</span>Local<span class="sep"
            >·</span
          >Free
        </div>
        <h1 class="headline">
          Convert anything.
          <span class="subline">Locally.</span>
        </h1>
        <p class="hero-body">
          No internet. No account. No telemetry. Just drag, drop, and convert — on your own machine.
        </p>
        <div class="hero-ctas">
          <a class="btn btn-primary btn-lg" href="#download">
            <svg viewBox="0 0 24 24">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
              <polyline points="7 10 12 15 17 10" />
              <line x1="12" y1="15" x2="12" y2="3" />
            </svg>
            Download for Linux
          </a>
          <a
            class="btn btn-outline btn-lg"
            href="https://github.com/Mvth1s/Verto"
            target="_blank"
            rel="noopener"
          >
            <svg viewBox="0 0 24 24" fill="currentColor" stroke="none">
              <path
                d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.4 3-.405 1.02.005 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"
              />
            </svg>
            View on GitHub
          </a>
        </div>
        <div class="os-row">
          <span class="label">Available for</span>
          <div class="os-badges">
            <span class="os-badge">
              <svg viewBox="0 0 24 24">
                <path
                  d="M12.504 0c-.155 0-.315.008-.48.021-4.226.333-3.105 4.807-3.17 6.298-.076 1.092-.3 1.953-1.05 3.02-.885 1.051-2.127 2.75-2.716 4.521-.278.832-.41 1.684-.287 2.489.005.04.011.08.018.118-.078.224-.144.452-.183.692-.317 1.96.46 4.005 2.13 5.605 1.81 1.732 4.357 2.633 6.96 2.483 2.45-.137 4.62-1.157 6.18-2.825.78-.836 1.39-1.836 1.74-2.95.36-1.14.42-2.35.21-3.61-.06-.42-.18-.84-.36-1.26.06-.45.06-.9-.06-1.35-.24-.96-.78-1.86-1.5-2.55-.6-.6-1.32-1.05-2.1-1.32-.42-.15-.84-.24-1.26-.27-.045-.524-.06-1.05-.15-1.575-.36-2.1-1.32-4.2-2.97-5.355-.795-.555-1.755-.81-2.715-.81z"
                />
              </svg>
              Linux
            </span>
            <span class="os-badge">
              <svg viewBox="0 0 24 24">
                <path
                  d="M0 3.449L9.75 2.1v9.451H0m10.949-9.602L24 0v11.4H10.949M0 12.6h9.75v9.451L0 20.699M10.949 12.6H24V24l-13.051-1.351"
                />
              </svg>
              Windows
            </span>
            <span class="os-badge">
              <svg viewBox="0 0 24 24">
                <path
                  d="M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.34 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z"
                />
              </svg>
              macOS
            </span>
          </div>
        </div>
        <div class="terminal-snippet">
          <span class="prompt">$</span>
          <span>brew install --cask verto<span class="caret"></span></span>
        </div>
      </div>

      <!-- App mockup -->
      <div class="mockup">
        <div class="win">
          <div class="win-bar">
            <div class="traffic">
              <span class="r"></span><span class="y"></span><span class="g"></span>
            </div>
            <div class="label">Verto — Images</div>
          </div>
          <div class="win-body">
            <div class="ws-sidebar">
              <div class="ws-brand">
                <img src="/logo.jpeg" alt="Verto" class="ws-brand-img" />
              </div>
              <div class="ws-item active"><span class="ico"></span>Images</div>
              <div class="ws-item"><span class="ico"></span>Documents</div>
              <div class="ws-item" style="opacity: 0.4"><span class="ico"></span>Audio</div>
              <div class="ws-item" style="opacity: 0.4"><span class="ico"></span>Video</div>
            </div>
            <div class="ws-main">
              <div class="ws-drop">
                <div class="icon-box">
                  <svg viewBox="0 0 24 24">
                    <path d="M12 3v12" />
                    <path d="M7 8l5-5 5 5" />
                    <path d="M5 21h14" />
                  </svg>
                </div>
                <div>Drop files here</div>
              </div>
              <div class="ws-row">
                <span class="name">diagram.png</span>
                <span class="arrow">→</span>
                <span class="to">.webp</span>
                <span class="ws-bar"><span class="fill"></span></span>
              </div>
            </div>
            <div class="ws-panel">
              <div>
                <div class="ws-label" style="margin-bottom: 6px">Format</div>
                <div class="ws-field"><span>WebP</span><span class="arrow-d">▾</span></div>
              </div>
              <div>
                <div
                  class="ws-label"
                  style="margin-bottom: 6px; display: flex; justify-content: space-between"
                >
                  <span>Quality</span
                  ><span style="color: var(--text); font-family: 'JetBrains Mono', monospace"
                    >85%</span
                  >
                </div>
                <div class="ws-slider"></div>
              </div>
              <div class="ws-btn">Convert</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>

  <!-- FEATURES -->
  <section id="features">
    <div class="container">
      <div class="section-head">
        <div class="section-eyebrow">Features</div>
        <h2 class="section-title">Everything you need.<br />Nothing you don't.</h2>
        <p class="section-sub">
          A single binary. Four file types. Zero network calls. Verto does one thing — convert files
          — and refuses to be anything else.
        </p>
      </div>
      <div class="features-grid">
        <div class="feature">
          <div class="feature-icon">
            <svg viewBox="0 0 24 24">
              <rect x="3" y="3" width="18" height="18" rx="2" />
              <circle cx="9" cy="9" r="2" />
              <path d="M21 15l-5-5L5 21" />
            </svg>
          </div>
          <div class="feature-title">Images</div>
          <div class="feature-body">
            JPEG, PNG, WebP, AVIF, TIFF, HEIC, GIF and more. Batch-convert hundreds of files in
            seconds.
          </div>
          <div class="feature-list">
            <span class="chip">.jpeg</span><span class="chip">.png</span
            ><span class="chip">.webp</span><span class="chip">.avif</span
            ><span class="chip">.tiff</span><span class="chip">.heic</span
            ><span class="chip">.gif</span>
          </div>
        </div>
        <div class="feature">
          <div class="feature-icon">
            <svg viewBox="0 0 24 24">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
              <path d="M14 2v6h6" />
              <path d="M8 13h8M8 17h5" />
            </svg>
          </div>
          <div class="feature-title">Documents</div>
          <div class="feature-body">
            PDF, DOCX, Markdown, HTML, EPUB, RTF. Round-trip your text without a cloud service in
            the loop.
          </div>
          <div class="feature-list">
            <span class="chip">.pdf</span><span class="chip">.docx</span
            ><span class="chip">.md</span><span class="chip">.html</span
            ><span class="chip">.epub</span><span class="chip">.rtf</span>
          </div>
        </div>
        <div class="feature">
          <div class="feature-icon">
            <svg viewBox="0 0 24 24">
              <rect x="3" y="11" width="18" height="10" rx="2" />
              <path d="M7 11V7a5 5 0 0 1 10 0v4" />
            </svg>
          </div>
          <div class="feature-title">100% Local</div>
          <div class="feature-body">
            Your files never leave your machine. No uploads, no servers, no third-party APIs. Works
            on a plane.
          </div>
          <div class="feature-list">
            <span class="chip">offline-first</span><span class="chip">no telemetry</span
            ><span class="chip">air-gapped</span>
          </div>
        </div>
        <div class="feature">
          <div class="feature-icon">
            <svg viewBox="0 0 24 24">
              <path d="M16 18l6-6-6-6" />
              <path d="M8 6l-6 6 6 6" />
            </svg>
          </div>
          <div class="feature-title">Open source</div>
          <div class="feature-body">
            MIT licensed. No black boxes, no creepy phone-home. Audit the source, file an issue,
            send a PR.
          </div>
          <div class="feature-list">
            <span class="chip">MIT</span><span class="chip">Rust + Tauri</span
            ><span class="chip">2.4k ★</span>
          </div>
        </div>
      </div>
    </div>
  </section>

  <!-- PRIVACY -->
  <section class="privacy">
    <div class="container">
      <div class="section-eyebrow" style="text-align: center">Privacy</div>
      <h2 class="privacy-title">Your files stay on your machine.</h2>
      <p class="privacy-sub">
        No uploads. No logs. No accounts. No analytics. Verto runs entirely on your device — what
        happens on your laptop stays on your laptop.
      </p>
      <div class="privacy-visual">
        <div class="pv-node local">
          <div class="pv-icon">
            <svg viewBox="0 0 24 24">
              <rect x="2" y="4" width="20" height="14" rx="2" />
              <line x1="2" y1="20" x2="22" y2="20" />
            </svg>
          </div>
          <div>Your machine</div>
          <div class="lock">
            <svg viewBox="0 0 24 24">
              <rect x="3" y="11" width="18" height="10" rx="2" />
              <path d="M7 11V7a5 5 0 0 1 10 0v4" />
            </svg>
          </div>
        </div>
        <div class="pv-arrow">
          <span class="x">no upload</span>
          <span class="line"></span>
        </div>
        <div class="pv-node" style="opacity: 0.5">
          <div class="pv-icon">
            <svg viewBox="0 0 24 24">
              <path d="M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z" />
            </svg>
          </div>
          <div>Cloud / Server</div>
        </div>
      </div>
    </div>
  </section>

  <!-- DOWNLOAD -->
  <section id="download">
    <div class="container">
      <div class="section-head" style="text-align: center; margin-left: auto; margin-right: auto">
        <div class="section-eyebrow">Download</div>
        <h2 class="section-title">Ready to install?</h2>
        <p class="section-sub" style="margin-left: auto; margin-right: auto">
          ~18 MB. Single binary. No installer trickery, no bundled extras.
        </p>
      </div>
      <div class="download-grid">
        <!-- Linux -->
        <div class="dl-card">
          <div class="dl-os">
            <div class="dl-os-icon">
              <svg viewBox="0 0 24 24">
                <path
                  d="M12.504 0c-.155 0-.315.008-.48.021-4.226.333-3.105 4.807-3.17 6.298-.076 1.092-.3 1.953-1.05 3.02-.885 1.051-2.127 2.75-2.716 4.521-.278.832-.41 1.684-.287 2.489.005.04.011.08.018.118-.078.224-.144.452-.183.692-.317 1.96.46 4.005 2.13 5.605 1.81 1.732 4.357 2.633 6.96 2.483 2.45-.137 4.62-1.157 6.18-2.825.78-.836 1.39-1.836 1.74-2.95.36-1.14.42-2.35.21-3.61-.06-.42-.18-.84-.36-1.26.06-.45.06-.9-.06-1.35-.24-.96-.78-1.86-1.5-2.55-.6-.6-1.32-1.05-2.1-1.32-.42-.15-.84-.24-1.26-.27-.045-.524-.06-1.05-.15-1.575-.36-2.1-1.32-4.2-2.97-5.355-.795-.555-1.755-.81-2.715-.81z"
                />
              </svg>
            </div>
            <div>
              <div class="dl-os-name">Linux</div>
              <div class="dl-os-version">{{ version ? `${version} · x86_64` : 'x86_64' }}</div>
            </div>
          </div>
          <div class="dl-formats">
            <template v-if="linuxAssets.length">
              <div v-for="a in linuxAssets" :key="a.name" class="row">
                <span>{{ ext(a.name) }}</span
                ><span class="size">{{ sizeMB(a.size) }}</span>
              </div>
            </template>
            <template v-else>
              <div class="row"><span>.AppImage</span><span class="size">~18 MB</span></div>
              <div class="row"><span>.deb</span><span class="size">~17 MB</span></div>
              <div class="row"><span>.rpm</span><span class="size">~17 MB</span></div>
            </template>
          </div>
          <div class="dl-links">
            <template v-if="linuxAssets.length">
              <a
                v-for="(a, i) in linuxAssets"
                :key="a.name"
                :class="['dl-btn', i > 0 ? 'outline' : '']"
                :href="a.browser_download_url"
                target="_blank"
                rel="noopener"
              >
                <svg viewBox="0 0 24 24">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                  <polyline points="7 10 12 15 17 10" />
                  <line x1="12" y1="15" x2="12" y2="3" />
                </svg>
                Download {{ ext(a.name) }}
              </a>
            </template>
            <a v-else class="dl-btn" :href="RELEASES_URL" target="_blank" rel="noopener">
              <svg viewBox="0 0 24 24">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="7 10 12 15 17 10" />
                <line x1="12" y1="15" x2="12" y2="3" />
              </svg>
              View releases
            </a>
          </div>
        </div>

        <!-- Windows -->
        <div class="dl-card">
          <div class="dl-os">
            <div class="dl-os-icon">
              <svg viewBox="0 0 24 24">
                <path
                  d="M0 3.449L9.75 2.1v9.451H0m10.949-9.602L24 0v11.4H10.949M0 12.6h9.75v9.451L0 20.699M10.949 12.6H24V24l-13.051-1.351"
                />
              </svg>
            </div>
            <div>
              <div class="dl-os-name">Windows</div>
              <div class="dl-os-version">{{ version ? `${version} · x86_64` : 'x86_64' }}</div>
            </div>
          </div>
          <div class="dl-formats">
            <template v-if="windowsAssets.length">
              <div v-for="a in windowsAssets" :key="a.name" class="row">
                <span>{{ ext(a.name) }}</span
                ><span class="size">{{ sizeMB(a.size) }}</span>
              </div>
            </template>
            <template v-else>
              <div class="row"><span>.exe (installer)</span><span class="size">~20 MB</span></div>
              <div class="row"><span>.msi</span><span class="size">~20 MB</span></div>
            </template>
          </div>
          <div class="dl-links">
            <template v-if="windowsAssets.length">
              <a
                v-for="(a, i) in windowsAssets"
                :key="a.name"
                :class="['dl-btn', i > 0 ? 'outline' : '']"
                :href="a.browser_download_url"
                target="_blank"
                rel="noopener"
              >
                <svg viewBox="0 0 24 24">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                  <polyline points="7 10 12 15 17 10" />
                  <line x1="12" y1="15" x2="12" y2="3" />
                </svg>
                Download {{ ext(a.name) }}
              </a>
            </template>
            <a v-else class="dl-btn outline" :href="RELEASES_URL" target="_blank" rel="noopener">
              <svg viewBox="0 0 24 24">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="7 10 12 15 17 10" />
                <line x1="12" y1="15" x2="12" y2="3" />
              </svg>
              View releases
            </a>
          </div>
        </div>

        <!-- macOS -->
        <div class="dl-card">
          <div class="dl-os">
            <div class="dl-os-icon">
              <svg viewBox="0 0 24 24">
                <path
                  d="M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.34 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z"
                />
              </svg>
            </div>
            <div>
              <div class="dl-os-name">macOS</div>
              <div class="dl-os-version">
                {{ version ? `${version} · Apple Silicon` : 'Apple Silicon' }}
              </div>
            </div>
          </div>
          <div class="dl-formats">
            <template v-if="macosAssets.length">
              <div v-for="a in macosAssets" :key="a.name" class="row">
                <span>{{ ext(a.name) }}</span
                ><span class="size">{{ sizeMB(a.size) }}</span>
              </div>
            </template>
            <template v-else>
              <div class="row"><span>.dmg</span><span class="size">~22 MB</span></div>
            </template>
          </div>
          <div class="dl-links">
            <template v-if="macosAssets.length">
              <a
                v-for="(a, i) in macosAssets"
                :key="a.name"
                :class="['dl-btn', i > 0 ? 'outline' : '']"
                :href="a.browser_download_url"
                target="_blank"
                rel="noopener"
              >
                <svg viewBox="0 0 24 24">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                  <polyline points="7 10 12 15 17 10" />
                  <line x1="12" y1="15" x2="12" y2="3" />
                </svg>
                Download {{ ext(a.name) }}
              </a>
            </template>
            <a v-else class="dl-btn outline" :href="RELEASES_URL" target="_blank" rel="noopener">
              <svg viewBox="0 0 24 24">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="7 10 12 15 17 10" />
                <line x1="12" y1="15" x2="12" y2="3" />
              </svg>
              View releases
            </a>
          </div>
        </div>
      </div>
    </div>
  </section>

  <footer>
    <div class="container footer-inner">
      <div class="footer-meta">Verto · MIT License · 2026</div>
      <div class="footer-links">
        <a href="https://github.com/Mvth1s/Verto" target="_blank" rel="noopener">GitHub</a>
        <a
          href="https://github.com/Mvth1s/Verto/blob/main/CHANGELOG.md"
          target="_blank"
          rel="noopener"
          >CHANGELOG</a
        >
        <a href="https://github.com/Mvth1s/Verto/issues" target="_blank" rel="noopener">Issues</a>
        <a href="https://github.com/Mvth1s/Verto/releases" target="_blank" rel="noopener"
          >Releases</a
        >
      </div>
    </div>
  </footer>
</template>

<style>
:root {
  --bg: #18181b;
  --bg-soft: #1f1f22;
  --surface: #27272a;
  --surface-2: #1c1c1f;
  --border: #3f3f46;
  --border-soft: #27272a;
  --text: #f4f4f5;
  --text-2: #a1a1aa;
  --text-3: #71717a;
  --text-4: #52525b;
  --accent: #10b981;
  --accent-bright: #34d399;
  --accent-soft: rgba(16, 185, 129, 0.12);
}
* {
  box-sizing: border-box;
}
html,
body {
  margin: 0;
  padding: 0;
  background: var(--bg);
  color: var(--text);
  font-family: 'Inter', system-ui, sans-serif;
  -webkit-font-smoothing: antialiased;
  line-height: 1.5;
}
html {
  scroll-behavior: smooth;
}
.mono {
  font-family: 'JetBrains Mono', monospace;
}

.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 32px;
}

/* HEADER */
header.nav {
  position: sticky;
  top: 0;
  z-index: 50;
  backdrop-filter: blur(14px);
  background: rgba(24, 24, 27, 0.7);
  border-bottom: 1px solid var(--border-soft);
}
.nav-inner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 0;
}
.brand {
  display: flex;
  align-items: center;
  gap: 10px;
}
.brand-logo {
  width: 28px;
  height: 28px;
  border-radius: 6px;
  display: block;
  object-fit: cover;
}
.brand-name {
  font-weight: 600;
  letter-spacing: -0.01em;
  font-size: 15px;
}

.nav-right {
  display: flex;
  align-items: center;
  gap: 12px;
}
.nav-link {
  color: var(--text-2);
  text-decoration: none;
  font-size: 13px;
  font-weight: 500;
  padding: 8px 12px;
  border-radius: 6px;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  transition: color 120ms ease;
}
.nav-link:hover {
  color: var(--text);
}
.nav-link svg {
  width: 14px;
  height: 14px;
  fill: currentColor;
}
.star-count {
  background: var(--surface);
  border: 1px solid var(--border-soft);
  border-radius: 4px;
  padding: 1px 6px;
  font-family: 'JetBrains Mono', monospace;
  font-size: 11px;
  color: var(--text-2);
}
.btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  border-radius: 99px;
  font-size: 13px;
  font-weight: 600;
  text-decoration: none;
  cursor: pointer;
  border: none;
  font-family: inherit;
  transition:
    transform 60ms ease,
    background 120ms ease,
    border-color 120ms ease;
  letter-spacing: -0.005em;
}
.btn-primary {
  background: var(--accent);
  color: #052e22;
  box-shadow: 0 4px 16px -4px rgba(16, 185, 129, 0.4);
}
.btn-primary:hover {
  background: var(--accent-bright);
}
.btn-primary:active {
  transform: translateY(1px);
}
.btn-outline {
  background: transparent;
  border: 1px solid var(--border);
  color: var(--text);
}
.btn-outline:hover {
  border-color: var(--text-2);
}
.btn-lg {
  padding: 12px 22px;
  font-size: 14px;
}
.btn svg {
  width: 14px;
  height: 14px;
  stroke: currentColor;
  stroke-width: 2;
  fill: none;
}

/* HERO */
.hero {
  min-height: calc(100vh - 57px);
  display: flex;
  align-items: center;
  padding: 60px 0;
  position: relative;
  overflow: hidden;
}
.hero::before {
  content: '';
  position: absolute;
  top: -200px;
  left: 50%;
  transform: translateX(-50%);
  width: 900px;
  height: 600px;
  background: radial-gradient(ellipse at center, rgba(16, 185, 129, 0.1), transparent 60%);
  pointer-events: none;
  filter: blur(40px);
}
.hero::after {
  content: '';
  position: absolute;
  inset: 0;
  background-image:
    linear-gradient(rgba(255, 255, 255, 0.025) 1px, transparent 1px),
    linear-gradient(90deg, rgba(255, 255, 255, 0.025) 1px, transparent 1px);
  background-size: 48px 48px;
  mask-image: radial-gradient(ellipse at 50% 40%, black 30%, transparent 75%);
  pointer-events: none;
}
.hero-grid {
  display: grid;
  grid-template-columns: 1fr 1.05fr;
  gap: 56px;
  align-items: center;
  position: relative;
  z-index: 1;
}
.eyebrow {
  display: inline-flex;
  align-items: center;
  gap: 0;
  font-family: 'JetBrains Mono', monospace;
  font-size: 11px;
  color: var(--text-2);
  padding: 6px 12px;
  border: 1px solid var(--border-soft);
  background: var(--surface-2);
  border-radius: 99px;
  letter-spacing: 0.02em;
}
.eyebrow .dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--accent);
  box-shadow: 0 0 8px var(--accent);
  margin-right: 8px;
  display: inline-block;
}
.eyebrow .sep {
  color: var(--text-4);
  margin: 0 8px;
}

.headline {
  font-size: clamp(56px, 7vw, 88px);
  font-weight: 700;
  line-height: 0.95;
  letter-spacing: -0.035em;
  margin: 22px 0 8px;
}
.headline .subline {
  display: block;
  color: var(--accent-bright);
  font-weight: 700;
}
.hero-body {
  font-size: 16px;
  color: var(--text-2);
  max-width: 440px;
  margin: 0 0 32px;
  line-height: 1.6;
}
.hero-ctas {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}
.os-row {
  display: flex;
  align-items: center;
  gap: 14px;
  margin-top: 28px;
  color: var(--text-3);
  font-size: 12px;
}
.os-row .label {
  font-family: 'JetBrains Mono', monospace;
  font-size: 11px;
}
.os-badges {
  display: flex;
  gap: 6px;
}
.os-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 5px 10px;
  border: 1px solid var(--border-soft);
  background: var(--surface-2);
  border-radius: 4px;
  font-family: 'JetBrains Mono', monospace;
  font-size: 11px;
  color: var(--text-2);
}
.os-badge svg {
  width: 11px;
  height: 11px;
  fill: currentColor;
}

/* Hero mockup */
.mockup {
  position: relative;
  transform: perspective(1600px) rotateY(-6deg) rotateX(2deg);
  transform-origin: center;
}
.mockup::before {
  content: '';
  position: absolute;
  inset: -20px;
  background: radial-gradient(ellipse at 60% 40%, rgba(16, 185, 129, 0.18), transparent 65%);
  filter: blur(30px);
  pointer-events: none;
  z-index: -1;
}
.win {
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 10px;
  overflow: hidden;
  box-shadow:
    0 50px 100px -30px rgba(0, 0, 0, 0.7),
    0 0 0 1px rgba(255, 255, 255, 0.02) inset;
}
.win-bar {
  display: flex;
  align-items: center;
  padding: 9px 12px;
  background: #1a1a1c;
  border-bottom: 1px solid var(--border-soft);
}
.win-bar .traffic {
  display: flex;
  gap: 6px;
}
.win-bar .traffic span {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  display: block;
}
.win-bar .traffic .r {
  background: #ff5f57;
}
.win-bar .traffic .y {
  background: #febc2e;
}
.win-bar .traffic .g {
  background: #28c840;
}
.win-bar .label {
  flex: 1;
  text-align: center;
  font-size: 11px;
  color: var(--text-4);
  font-family: 'JetBrains Mono', monospace;
}

.win-body {
  display: grid;
  grid-template-columns: 130px 1fr 180px;
  height: 380px;
}
.ws-sidebar {
  background: #161618;
  border-right: 1px solid var(--border-soft);
  padding: 14px 8px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.ws-brand {
  padding: 0 2px 8px;
}
.ws-brand-img {
  width: 44px;
  height: 44px;
  border-radius: 8px;
  display: block;
  object-fit: cover;
}
.ws-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 8px;
  border-radius: 5px;
  font-size: 11px;
  color: var(--text-3);
  font-weight: 500;
}
.ws-item.active {
  background: var(--accent-soft);
  color: var(--accent-bright);
}
.ws-item .ico {
  width: 11px;
  height: 11px;
  border-radius: 2px;
  background: currentColor;
  opacity: 0.6;
}
.ws-main {
  padding: 14px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.ws-drop {
  border: 1.5px dashed rgba(16, 185, 129, 0.45);
  background: rgba(16, 185, 129, 0.04);
  border-radius: 8px;
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 8px;
  color: var(--accent-bright);
  font-size: 11px;
  font-weight: 500;
}
.ws-drop .icon-box {
  width: 30px;
  height: 30px;
  border-radius: 6px;
  background: var(--accent-soft);
  display: grid;
  place-items: center;
}
.ws-drop .icon-box svg {
  width: 16px;
  height: 16px;
  stroke: var(--accent-bright);
  stroke-width: 1.6;
  fill: none;
}
.ws-row {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 7px 10px;
  background: var(--surface);
  border-radius: 5px;
  font-family: 'JetBrains Mono', monospace;
  font-size: 10px;
  color: var(--text-2);
}
.ws-row .name {
  flex: 1;
  color: var(--text);
}
.ws-row .arrow {
  color: var(--text-4);
}
.ws-row .to {
  color: var(--accent-bright);
}
.ws-bar {
  width: 70px;
  height: 3px;
  background: var(--border);
  border-radius: 99px;
  overflow: hidden;
}
.ws-bar .fill {
  height: 100%;
  background: var(--accent);
  width: 65%;
  border-radius: 99px;
}
.ws-panel {
  background: #161618;
  border-left: 1px solid var(--border-soft);
  padding: 14px 12px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.ws-label {
  font-size: 9px;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  color: var(--text-3);
  font-weight: 500;
}
.ws-field {
  border: 1px solid var(--border-soft);
  background: var(--surface);
  border-radius: 4px;
  padding: 6px 9px;
  font-size: 10px;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text);
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.ws-field .arrow-d {
  color: var(--text-3);
  font-size: 8px;
}
.ws-slider {
  height: 4px;
  background: var(--border);
  border-radius: 99px;
  position: relative;
}
.ws-slider::after {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 80%;
  background: var(--accent);
  border-radius: 99px;
}
.ws-slider::before {
  content: '';
  position: absolute;
  left: calc(80% - 6px);
  top: -3px;
  width: 10px;
  height: 10px;
  background: var(--accent);
  border-radius: 50%;
  border: 2px solid var(--bg);
  box-shadow: 0 0 0 1px var(--accent);
}
.ws-btn {
  margin-top: auto;
  background: var(--accent);
  color: #052e22;
  padding: 8px 12px;
  border-radius: 5px;
  font-size: 11px;
  font-weight: 600;
  text-align: center;
}

/* SECTION shared */
section {
  padding: 120px 0;
  position: relative;
}
.section-title {
  font-size: clamp(36px, 4.5vw, 52px);
  font-weight: 700;
  letter-spacing: -0.03em;
  line-height: 1.1;
  margin: 0;
}
.section-eyebrow {
  font-family: 'JetBrains Mono', monospace;
  font-size: 11px;
  color: var(--accent-bright);
  letter-spacing: 0.08em;
  text-transform: uppercase;
  margin-bottom: 14px;
}
.section-head {
  margin-bottom: 56px;
  max-width: 720px;
}
.section-sub {
  color: var(--text-2);
  font-size: 16px;
  margin-top: 14px;
  max-width: 560px;
}

/* FEATURES */
.features-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1px;
  background: var(--border-soft);
  border: 1px solid var(--border-soft);
  border-radius: 12px;
  overflow: hidden;
}
.feature {
  background: var(--bg);
  padding: 36px;
  position: relative;
  transition: background 200ms ease;
}
.feature:hover {
  background: var(--bg-soft);
}
.feature-icon {
  width: 38px;
  height: 38px;
  border-radius: 8px;
  background: var(--accent-soft);
  border: 1px solid rgba(16, 185, 129, 0.2);
  display: grid;
  place-items: center;
  color: var(--accent-bright);
  margin-bottom: 22px;
}
.feature-icon svg {
  width: 18px;
  height: 18px;
  stroke: currentColor;
  stroke-width: 1.6;
  fill: none;
}
.feature-title {
  font-size: 18px;
  font-weight: 600;
  letter-spacing: -0.01em;
  margin-bottom: 8px;
}
.feature-body {
  color: var(--text-2);
  font-size: 14px;
  line-height: 1.6;
  max-width: 380px;
}
.feature-list {
  margin-top: 14px;
  display: flex;
  flex-wrap: wrap;
  gap: 5px;
}
.chip {
  font-family: 'JetBrains Mono', monospace;
  font-size: 10px;
  color: var(--text-2);
  padding: 3px 8px;
  background: var(--surface);
  border: 1px solid var(--border-soft);
  border-radius: 3px;
}

/* PRIVACY */
.privacy {
  background: linear-gradient(180deg, var(--bg) 0%, #0f0f11 50%, var(--bg) 100%);
  text-align: center;
  border-top: 1px solid var(--border-soft);
  border-bottom: 1px solid var(--border-soft);
}
.privacy-title {
  font-size: clamp(38px, 5vw, 64px);
  font-weight: 700;
  letter-spacing: -0.035em;
  line-height: 1.05;
  max-width: 880px;
  margin: 0 auto;
}
.privacy-sub {
  color: var(--text-2);
  font-size: 16px;
  max-width: 540px;
  margin: 22px auto 0;
  line-height: 1.6;
}
.privacy-visual {
  margin-top: 56px;
  display: grid;
  grid-template-columns: 1fr auto 1fr;
  align-items: center;
  gap: 32px;
  max-width: 760px;
  margin-left: auto;
  margin-right: auto;
}
.pv-node {
  border: 1px solid var(--border-soft);
  background: var(--surface-2);
  border-radius: 10px;
  padding: 24px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  font-family: 'JetBrains Mono', monospace;
  font-size: 11px;
  color: var(--text-2);
  position: relative;
}
.pv-node.local {
  border-color: rgba(16, 185, 129, 0.25);
  background: rgba(16, 185, 129, 0.03);
}
.pv-node .pv-icon {
  width: 44px;
  height: 44px;
  border-radius: 8px;
  background: var(--surface);
  border: 1px solid var(--border-soft);
  display: grid;
  place-items: center;
}
.pv-node.local .pv-icon {
  background: var(--accent-soft);
  border-color: rgba(16, 185, 129, 0.3);
  color: var(--accent-bright);
}
.pv-node .pv-icon svg {
  width: 20px;
  height: 20px;
  stroke: currentColor;
  stroke-width: 1.6;
  fill: none;
}
.pv-node .lock {
  position: absolute;
  top: -8px;
  right: -8px;
  width: 20px;
  height: 20px;
  background: var(--accent);
  border-radius: 50%;
  display: grid;
  place-items: center;
  color: #052e22;
}
.pv-node .lock svg {
  width: 10px;
  height: 10px;
  stroke: currentColor;
  stroke-width: 2.5;
  fill: none;
}
.pv-arrow {
  font-family: 'JetBrains Mono', monospace;
  font-size: 20px;
  color: var(--text-4);
  position: relative;
}
.pv-arrow .x {
  position: absolute;
  top: -22px;
  left: 50%;
  transform: translateX(-50%);
  color: #ef4444;
  font-size: 11px;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  white-space: nowrap;
  font-weight: 600;
}
.pv-arrow .line {
  width: 80px;
  height: 1px;
  background: var(--border);
  background-image: linear-gradient(90deg, var(--text-4) 50%, transparent 50%);
  background-size: 8px 1px;
  display: block;
}

/* DOWNLOAD */
.download-grid {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 16px;
}
.dl-card {
  background: var(--surface-2);
  border: 1px solid var(--border-soft);
  border-radius: 10px;
  padding: 28px 26px;
  display: flex;
  flex-direction: column;
  gap: 14px;
  transition:
    border-color 160ms ease,
    transform 160ms ease;
}
.dl-card:hover {
  border-color: var(--border);
  transform: translateY(-2px);
}
.dl-os {
  display: flex;
  align-items: center;
  gap: 12px;
}
.dl-os-icon {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  background: var(--surface);
  border: 1px solid var(--border-soft);
  display: grid;
  place-items: center;
  color: var(--text);
}
.dl-os-icon svg {
  width: 18px;
  height: 18px;
  fill: currentColor;
}
.dl-os-name {
  font-size: 16px;
  font-weight: 600;
  letter-spacing: -0.01em;
}
.dl-os-version {
  font-family: 'JetBrains Mono', monospace;
  font-size: 11px;
  color: var(--text-3);
}
.dl-formats {
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-family: 'JetBrains Mono', monospace;
  font-size: 11px;
  color: var(--text-2);
}
.dl-formats .row {
  display: flex;
  justify-content: space-between;
  padding: 6px 0;
  border-bottom: 1px dashed var(--border-soft);
}
.dl-formats .row:last-child {
  border-bottom: none;
}
.dl-formats .size {
  color: var(--text-3);
}
.dl-links {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-top: 6px;
}
.dl-btn {
  margin-top: 6px;
  width: 100%;
  background: var(--accent);
  color: #052e22;
  border: none;
  border-radius: 6px;
  padding: 10px;
  font: inherit;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  transition: background 120ms ease;
}
.dl-btn:hover {
  background: var(--accent-bright);
}
.dl-btn.outline {
  background: transparent;
  color: var(--text);
  border: 1px solid var(--border);
}
.dl-btn.outline:hover {
  border-color: var(--text-2);
  background: transparent;
}
.dl-btn svg {
  width: 14px;
  height: 14px;
  stroke: currentColor;
  stroke-width: 2;
  fill: none;
}

/* FOOTER */
footer {
  border-top: 1px solid var(--border-soft);
  padding: 32px 0;
  color: var(--text-3);
  font-size: 12px;
}
.footer-inner {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 14px;
}
.footer-links {
  display: flex;
  gap: 18px;
}
.footer-links a {
  color: var(--text-3);
  text-decoration: none;
  transition: color 120ms ease;
}
.footer-links a:hover {
  color: var(--text);
}
.footer-meta {
  font-family: 'JetBrains Mono', monospace;
}

/* Terminal snippet */
.terminal-snippet {
  margin-top: 24px;
  background: var(--surface-2);
  border: 1px solid var(--border-soft);
  border-radius: 8px;
  padding: 14px 16px;
  font-family: 'JetBrains Mono', monospace;
  font-size: 12px;
  color: var(--text-2);
  max-width: 440px;
  display: flex;
  align-items: center;
  gap: 10px;
}
.terminal-snippet .prompt {
  color: var(--accent-bright);
}
.terminal-snippet .caret {
  display: inline-block;
  width: 8px;
  height: 14px;
  background: var(--accent);
  margin-left: 6px;
  animation: blink 1s steps(1) infinite;
}

@keyframes blink {
  50% {
    opacity: 0;
  }
}

@media (max-width: 900px) {
  .hero-grid {
    grid-template-columns: 1fr;
  }
  .features-grid {
    grid-template-columns: 1fr;
  }
  .download-grid {
    grid-template-columns: 1fr;
  }
  .privacy-visual {
    grid-template-columns: 1fr;
  }
  .pv-arrow .line {
    width: 1px;
    height: 40px;
    background: none;
    background-image: linear-gradient(180deg, var(--text-4) 50%, transparent 50%);
    background-size: 1px 8px;
  }
}
</style>
