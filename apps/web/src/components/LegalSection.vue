<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useAnalytics } from '../composables/useAnalytics'

const { t } = useI18n()
const { openConsentBanner } = useAnalytics()

const PRIVACY_ITEMS = ['website', 'analytics', 'third_parties', 'storage', 'app', 'rights'] as const
</script>

<template>
  <section id="legal" class="legal">
    <div class="container">
      <div class="section-eyebrow">{{ t('legal.eyebrow') }}</div>
      <h2 class="legal-title">{{ t('legal.title') }}</h2>
      <p>{{ t('legal.intro') }}</p>
      <dl>
        <dt>{{ t('legal.publisher_t') }}</dt>
        <dd>{{ t('legal.publisher') }}</dd>
        <dt>{{ t('legal.contact_t') }}</dt>
        <dd>{{ t('legal.contact') }}</dd>
        <dt>{{ t('legal.host_t') }}</dt>
        <dd>
          {{ t('legal.host') }}
          <a href="https://vercel.com/legal" target="_blank" rel="noopener">vercel.com/legal</a>
        </dd>
      </dl>

      <h2 id="privacy-policy" class="legal-title">{{ t('legal.privacy_title') }}</h2>
      <dl>
        <template v-for="key in PRIVACY_ITEMS" :key="key">
          <dt>{{ t(`legal.privacy.${key}_t`) }}</dt>
          <dd>{{ t(`legal.privacy.${key}`) }}</dd>
        </template>
      </dl>
      <p class="legal-actions">
        <button type="button" class="btn btn-outline" @click="openConsentBanner">
          {{ t('footer.cookie_settings') }}
        </button>
        <a href="https://www.cnil.fr/fr/plaintes" target="_blank" rel="noopener">CNIL</a>
      </p>
    </div>
  </section>
</template>

<style scoped>
.legal {
  padding: 64px 0;
  border-top: 1px solid var(--border-soft);
  color: var(--text-3);
  font-size: 14px;
  line-height: 1.65;
}
.legal-title {
  color: var(--text);
  font-size: 22px;
  margin: 0 0 12px;
}
#privacy-policy {
  margin-top: 40px;
}
.legal dl {
  margin: 0;
  max-width: 820px;
}
.legal dt {
  color: var(--text);
  font-weight: 600;
  margin-top: 14px;
}
.legal dd {
  margin: 4px 0 0;
}
.legal a {
  color: var(--text);
  text-decoration: underline;
}
.legal-actions {
  display: flex;
  align-items: center;
  gap: 18px;
  margin-top: 24px;
}
</style>
