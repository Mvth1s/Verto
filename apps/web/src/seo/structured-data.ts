// Schema.org JSON-LD injected into index.html at build time (see vite.config.ts).
// Only factual data: no ratings, no placeholder handles.
// The FAQPage is built from the same i18n strings as the visible FAQ (English, the default locale).

import en from '../i18n/en.json'

const REPO_URL = 'https://github.com/Mvth1s/Verto'

export const SITE_DESCRIPTION =
  'Verto is a free, open-source desktop app for converting images, documents, audio and video files entirely offline. Your files never leave your machine.'

interface StructuredDataOptions {
  siteUrl: string
  version?: string
}

export function buildStructuredData({ siteUrl, version }: StructuredDataOptions) {
  const authorId = `${siteUrl}/#author`
  const sourceId = `${siteUrl}/#source`

  return {
    '@context': 'https://schema.org',
    '@graph': [
      {
        '@type': 'SoftwareApplication',
        '@id': `${siteUrl}/#app`,
        name: 'Verto',
        description: SITE_DESCRIPTION,
        url: `${siteUrl}/`,
        image: `${siteUrl}/og-image.png`,
        applicationCategory: 'UtilitiesApplication',
        operatingSystem: 'Windows, macOS, Linux',
        ...(version ? { softwareVersion: version } : {}),
        downloadUrl: `${REPO_URL}/releases/latest`,
        license: 'https://opensource.org/license/mit',
        isAccessibleForFree: true,
        offers: { '@type': 'Offer', price: '0', priceCurrency: 'EUR' },
        author: { '@id': authorId },
        isBasedOn: { '@id': sourceId },
      },
      {
        '@type': 'SoftwareSourceCode',
        '@id': sourceId,
        name: 'Verto',
        codeRepository: REPO_URL,
        programmingLanguage: ['Rust', 'TypeScript', 'Vue'],
        license: 'https://opensource.org/license/mit',
        author: { '@id': authorId },
      },
      {
        '@type': 'Person',
        '@id': authorId,
        name: 'Mathis Aguado',
        url: 'https://mathisaguado.vercel.app',
        sameAs: ['https://github.com/Mvth1s', 'https://www.linkedin.com/in/mathis-aguado'],
      },
      {
        '@type': 'FAQPage',
        '@id': `${siteUrl}/#faq`,
        mainEntity: en.faq.items.map(({ q, a }) => ({
          '@type': 'Question',
          name: q,
          acceptedAnswer: { '@type': 'Answer', text: a },
        })),
      },
    ],
  }
}
