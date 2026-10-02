<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { type Locale } from '../i18n'

const { t, locale: i18nLocale } = useI18n()
const locale = ref<Locale>('en')

function setLocale(l: Locale) {
  locale.value = l
  i18nLocale.value = l
}
</script>

<template>
  <header class="nav">
    <div class="container nav-inner">
      <a class="brand" href="#" style="text-decoration: none; color: inherit">
        <img src="/verto.png" alt="" width="28" height="28" class="brand-logo" />
        <div class="brand-name">Verto</div>
      </a>
      <div class="nav-right">
        <a class="nav-link" href="#features">{{ t('nav.features') }}</a>
        <a class="nav-link" href="#privacy">{{ t('nav.privacy') }}</a>
        <a class="nav-link" href="https://github.com/Mvth1s/Verto" target="_blank" rel="noopener">
          <svg viewBox="0 0 24 24">
            <path
              d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.4 3-.405 1.02.005 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"
            />
          </svg>
          GitHub
        </a>
        <div class="lang-toggle">
          <button :class="{ active: locale === 'en' }" @click="setLocale('en')">EN</button>
          <button :class="{ active: locale === 'fr' }" @click="setLocale('fr')">FR</button>
        </div>
        <a class="btn btn-primary" href="#download">
          <svg viewBox="0 0 24 24">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
            <polyline points="7 10 12 15 17 10" />
            <line x1="12" y1="15" x2="12" y2="3" />
          </svg>
          {{ t('nav.download') }}
        </a>
      </div>
    </div>
  </header>
</template>
