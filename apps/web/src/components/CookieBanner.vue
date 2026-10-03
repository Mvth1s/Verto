<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useAnalytics } from '../composables/useAnalytics'

const { t } = useI18n()
const { bannerVisible, setConsent } = useAnalytics()
</script>

<template>
  <Transition name="cookie-banner">
    <div
      v-if="bannerVisible"
      class="cookie-banner"
      role="dialog"
      aria-live="polite"
      :aria-label="t('cookies.title')"
    >
      <div class="cookie-banner-text">
        <strong>{{ t('cookies.title') }}</strong>
        <p>{{ t('cookies.body') }}</p>
      </div>
      <div class="cookie-banner-actions">
        <!-- Both choices get the same weight: refusing must be as easy as accepting -->
        <button class="btn btn-outline" @click="setConsent('denied')">
          {{ t('cookies.decline') }}
        </button>
        <button class="btn btn-outline" @click="setConsent('granted')">
          {{ t('cookies.accept') }}
        </button>
      </div>
    </div>
  </Transition>
</template>
