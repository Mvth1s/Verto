<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
</script>

<template>
  <section id="features">
    <div class="container">
      <div class="section-head">
        <div class="section-eyebrow">{{ t('features.title') }}</div>
        <h2 class="section-title">{{ t('features.section_title') }}</h2>
        <p class="section-sub">{{ t('features.section_sub') }}</p>
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
          <div class="feature-title">{{ t('features.images_title') }}</div>
          <div class="feature-body">{{ t('features.images_desc') }}</div>
          <div class="feature-list">
            <span class="chip">.jpeg</span><span class="chip">.png</span
            ><span class="chip">.webp</span><span class="chip">.avif</span
            ><span class="chip">.heic</span><span class="chip">.gif</span
            ><span class="chip">.bmp</span><span class="chip">.tiff</span
            ><span class="chip">.ico</span><span class="chip">.psd</span
            ><span class="chip">.exr</span>
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
          <div class="feature-title">{{ t('features.documents_title') }}</div>
          <div class="feature-body">{{ t('features.documents_desc') }}</div>
          <div class="feature-list">
            <span class="chip">.md</span><span class="chip">.docx</span
            ><span class="chip">.html</span><span class="chip">.rst</span
            ><span class="chip">.odt</span><span class="chip">.epub</span
            ><span class="chip">.tex</span><span class="chip">.org</span
            ><span class="chip">.txt</span><span class="chip">.rtf</span
            ><span class="chip">.pptx</span><span class="chip">.ipynb</span>
          </div>
        </div>
        <div class="feature">
          <div class="feature-icon">
            <svg viewBox="0 0 24 24">
              <path d="M9 18V5l12-2v13" />
              <circle cx="6" cy="18" r="3" />
              <circle cx="18" cy="16" r="3" />
            </svg>
          </div>
          <div class="feature-title">{{ t('features.audio_title') }}</div>
          <div class="feature-body">{{ t('features.audio_desc') }}</div>
          <div class="feature-list">
            <span class="chip">.mp3</span><span class="chip">.flac</span
            ><span class="chip">.ogg</span><span class="chip">.wav</span
            ><span class="chip">.aac</span><span class="chip">.aiff</span
            ><span class="chip">.wma</span><span class="chip">.ac3</span
            ><span class="chip">.opus</span>
          </div>
        </div>
        <div class="feature">
          <div class="feature-icon">
            <svg viewBox="0 0 24 24">
              <rect x="2" y="2" width="20" height="20" rx="2.18" ry="2.18" />
              <line x1="7" y1="2" x2="7" y2="22" />
              <line x1="17" y1="2" x2="17" y2="22" />
              <line x1="2" y1="12" x2="22" y2="12" />
              <line x1="2" y1="7" x2="7" y2="7" />
              <line x1="2" y1="17" x2="7" y2="17" />
              <line x1="17" y1="17" x2="22" y2="17" />
              <line x1="17" y1="7" x2="22" y2="7" />
            </svg>
          </div>
          <div class="feature-title">{{ t('features.video_title') }}</div>
          <div class="feature-body">{{ t('features.video_desc') }}</div>
          <div class="feature-list">
            <span class="chip">.mp4</span><span class="chip">.mkv</span
            ><span class="chip">.webm</span><span class="chip">.mov</span
            ><span class="chip">.avi</span><span class="chip">.ogv</span
            ><span class="chip">.gif</span><span class="chip">.3gp</span
            ><span class="chip">.ts</span><span class="chip">.mpg</span>
          </div>
        </div>
        <div class="feature">
          <div class="feature-icon">
            <svg viewBox="0 0 24 24">
              <path d="M3 6h18M3 12h18M3 18h18" />
              <rect x="3" y="3" width="4" height="4" rx="1" />
              <rect x="3" y="9" width="4" height="4" rx="1" />
              <rect x="3" y="15" width="4" height="4" rx="1" />
            </svg>
          </div>
          <div class="feature-title">{{ t('features.batch_title') }}</div>
          <div class="feature-body">{{ t('features.batch_desc') }}</div>
          <div class="feature-list">
            <span class="chip">{{ t('features.batch_chip_1') }}</span
            ><span class="chip">{{ t('features.batch_chip_2') }}</span
            ><span class="chip">{{ t('features.batch_chip_3') }}</span>
          </div>
        </div>
        <div class="feature">
          <div class="feature-icon">
            <svg viewBox="0 0 24 24">
              <polyline points="23 4 23 10 17 10" />
              <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10" />
            </svg>
          </div>
          <div class="feature-title">{{ t('features.queue_title') }}</div>
          <div class="feature-body">{{ t('features.queue_desc') }}</div>
          <div class="feature-list">
            <span class="chip">{{ t('features.queue_chip_1') }}</span
            ><span class="chip">{{ t('features.queue_chip_2') }}</span
            ><span class="chip">{{ t('features.queue_chip_3') }}</span>
          </div>
        </div>
        <div class="feature">
          <div class="feature-icon">
            <svg viewBox="0 0 24 24">
              <circle cx="12" cy="12" r="10" />
              <polyline points="12 6 12 12 16 14" />
            </svg>
          </div>
          <div class="feature-title">{{ t('features.history_title') }}</div>
          <div class="feature-body">{{ t('features.history_desc') }}</div>
          <div class="feature-list">
            <span class="chip">{{ t('features.history_chip_1') }}</span
            ><span class="chip">{{ t('features.history_chip_2') }}</span
            ><span class="chip">{{ t('features.history_chip_3') }}</span>
          </div>
        </div>
        <div class="feature">
          <div class="feature-icon">
            <svg viewBox="0 0 24 24">
              <rect x="3" y="11" width="18" height="10" rx="2" />
              <path d="M7 11V7a5 5 0 0 1 10 0v4" />
            </svg>
          </div>
          <div class="feature-title">{{ t('features.local_title') }}</div>
          <div class="feature-body">{{ t('features.local_desc') }}</div>
          <div class="feature-list">
            <span class="chip">{{ t('features.local_chip_1') }}</span
            ><span class="chip">{{ t('features.local_chip_2') }}</span
            ><span class="chip">{{ t('features.local_chip_3') }}</span>
          </div>
        </div>
        <div class="feature">
          <div class="feature-icon">
            <svg viewBox="0 0 24 24">
              <path d="M16 18l6-6-6-6" />
              <path d="M8 6l-6 6 6 6" />
            </svg>
          </div>
          <div class="feature-title">{{ t('features.opensource_title') }}</div>
          <div class="feature-body">{{ t('features.opensource_desc') }}</div>
          <div class="feature-list">
            <span class="chip">MIT</span><span class="chip">Rust + Tauri v2</span
            ><span class="chip">Vue 3</span>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
