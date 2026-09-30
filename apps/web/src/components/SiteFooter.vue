<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
</script>

<template>
  <footer>
    <div class="container footer-inner">
      <div class="footer-meta">Verto · {{ t('footer.license') }} · 2026</div>
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
