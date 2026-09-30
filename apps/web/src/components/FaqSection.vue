<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

// Same source as the FAQPage JSON-LD (src/seo/structured-data.ts)
interface FaqItem {
  q: string
  a: string
}

const { t, tm, rt } = useI18n()

const items = computed(() =>
  (tm('faq.items') as unknown as FaqItem[]).map((item) => ({ q: rt(item.q), a: rt(item.a) })),
)
</script>

<template>
  <section id="faq" class="faq">
    <div class="container">
      <div class="section-head faq-head">
        <div class="section-eyebrow">{{ t('faq.eyebrow') }}</div>
        <h2 class="section-title">{{ t('faq.title') }}</h2>
      </div>
      <div class="faq-list">
        <details v-for="item in items" :key="item.q" class="faq-item">
          <summary>
            <h3 class="faq-q">{{ item.q }}</h3>
            <svg class="faq-icon" viewBox="0 0 24 24" aria-hidden="true">
              <path d="M12 5v14M5 12h14" />
            </svg>
          </summary>
          <p class="faq-a">{{ item.a }}</p>
        </details>
      </div>
    </div>
  </section>
</template>

<style scoped>
.faq {
  border-top: 1px solid var(--border-soft);
}
.faq-head {
  text-align: center;
  margin-left: auto;
  margin-right: auto;
}
.faq-list {
  max-width: 760px;
  margin: 0 auto;
  border: 1px solid var(--border-soft);
  border-radius: 12px;
  overflow: hidden;
}
.faq-item {
  background: var(--bg);
  transition: background 200ms ease;
}
.faq-item + .faq-item {
  border-top: 1px solid var(--border-soft);
}
.faq-item[open],
.faq-item:hover {
  background: var(--bg-soft);
}
summary {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 20px 24px;
  cursor: pointer;
  list-style: none;
}
summary::-webkit-details-marker {
  display: none;
}
summary:focus-visible {
  outline: 2px solid var(--accent-bright);
  outline-offset: -2px;
  border-radius: 12px;
}
.faq-q {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  letter-spacing: -0.01em;
}
.faq-icon {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
  stroke: var(--accent-bright);
  stroke-width: 2;
  fill: none;
  transition: transform 160ms ease;
}
.faq-item[open] .faq-icon {
  transform: rotate(45deg);
}
.faq-a {
  margin: 0;
  padding: 0 24px 22px;
  color: var(--text-2);
  font-size: 14px;
  line-height: 1.6;
}

@media (max-width: 640px) {
  summary {
    padding: 16px 18px;
  }
  .faq-a {
    padding: 0 18px 18px;
  }
}
</style>
