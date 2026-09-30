<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
</script>

<template>
  <section id="privacy" class="privacy">
    <div class="container">
      <div class="section-eyebrow" style="text-align: center">{{ t('privacy.eyebrow') }}</div>
      <h2 class="privacy-title">{{ t('privacy.title') }}</h2>
      <p class="privacy-sub">{{ t('privacy.body') }}</p>
      <div class="privacy-visual">
        <div class="pv-node local">
          <div class="pv-icon">
            <svg viewBox="0 0 24 24">
              <rect x="2" y="4" width="20" height="14" rx="2" />
              <line x1="2" y1="20" x2="22" y2="20" />
            </svg>
          </div>
          <div>{{ t('privacy.your_machine') }}</div>
          <div class="lock">
            <svg viewBox="0 0 24 24">
              <rect x="3" y="11" width="18" height="10" rx="2" />
              <path d="M7 11V7a5 5 0 0 1 10 0v4" />
            </svg>
          </div>
        </div>
        <div class="pv-arrow">
          <span class="x">{{ t('privacy.no_upload') }}</span>
          <span class="line"></span>
        </div>
        <div class="pv-node" style="opacity: 0.5">
          <div class="pv-icon">
            <svg viewBox="0 0 24 24">
              <path d="M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z" />
            </svg>
          </div>
          <div>{{ t('privacy.cloud_server') }}</div>
        </div>
      </div>
      <div class="privacy-details">
        <div class="pv-detail">
          <span class="pv-check">✓</span>
          <div>
            <strong>{{ t('privacy.detail_1_title') }}</strong> {{ t('privacy.detail_1_body') }}
          </div>
        </div>
        <div class="pv-detail">
          <span class="pv-check">✓</span>
          <div>
            <strong>{{ t('privacy.detail_2_title') }}</strong> {{ t('privacy.detail_2_body') }}
          </div>
        </div>
        <div class="pv-detail">
          <span class="pv-check">✓</span>
          <div>
            <strong>{{ t('privacy.detail_3_title') }}</strong> {{ t('privacy.detail_3_body') }}
          </div>
        </div>
        <div class="pv-detail">
          <span class="pv-check">✓</span>
          <div>
            <strong>{{ t('privacy.detail_4_title') }}</strong> {{ t('privacy.detail_4_body') }}
          </div>
        </div>
        <div class="pv-detail">
          <span class="pv-check">✓</span>
          <div>
            <strong>{{ t('privacy.detail_5_title') }}</strong> {{ t('privacy.detail_5_body') }}
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
