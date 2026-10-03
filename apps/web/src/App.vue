<script setup lang="ts">
import { defineAsyncComponent, onMounted } from 'vue'
import SiteHeader from './components/SiteHeader.vue'
import HeroSection from './components/HeroSection.vue'
import CookieBanner from './components/CookieBanner.vue'
import { useLatestRelease } from './composables/useLatestRelease'

// Sections below the fold are split into their own chunks
const sectionLoaders = [
  () => import('./components/FeaturesSection.vue'),
  () => import('./components/PrivacySection.vue'),
  () => import('./components/DownloadSection.vue'),
  () => import('./components/FaqSection.vue'),
  () => import('./components/SiteFooter.vue'),
] as const

const FeaturesSection = defineAsyncComponent(sectionLoaders[0])
const PrivacySection = defineAsyncComponent(sectionLoaders[1])
const DownloadSection = defineAsyncComponent(sectionLoaders[2])
const FaqSection = defineAsyncComponent(sectionLoaders[3])
const SiteFooter = defineAsyncComponent(sectionLoaders[4])

const { loadLatestRelease } = useLatestRelease()

onMounted(async () => {
  loadLatestRelease()

  // A deep link such as /#download targets a lazy section: scroll once it is rendered
  if (window.location.hash) {
    await Promise.all(sectionLoaders.map((load) => load()))
    requestAnimationFrame(() => {
      document.querySelector(window.location.hash)?.scrollIntoView()
    })
  }
})
</script>

<template>
  <SiteHeader />
  <HeroSection />
  <FeaturesSection />
  <PrivacySection />
  <DownloadSection />
  <FaqSection />
  <SiteFooter />
  <CookieBanner />
</template>
