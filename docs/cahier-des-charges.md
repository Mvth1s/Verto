# Cahier des charges — Verto

## 1. Contexte et objectif

### Problème

Convertir un fichier d'un format à un autre sous Linux (ou Windows/macOS) implique souvent de se souvenir de commandes `ffmpeg`, `convert`, `pandoc` complexes, ou de passer par des services en ligne qui nécessitent d'envoyer ses fichiers sur un serveur tiers.

### Solution

**Verto** est une application de bureau open-source permettant la conversion locale de fichiers (images, documents, audio, vidéo) via une interface graphique simple, sans dépendance à Internet.

### Cible

- Développeurs, designers, rédacteurs techniques
- Utilisateurs souhaitant une alternative locale aux convertisseurs en ligne
- Profils débutants et avancés (interface simple, mais options disponibles)

---

## 2. Périmètre fonctionnel

### 2.1 Fonctionnalités principales

#### Conversion d'images
- Formats supportés en entrée/sortie : JPEG, PNG, WebP, BMP, TIFF, GIF, AVIF
- Options : qualité (0-100), préréglages (Web / Print / Lossless), redimensionnement (largeur × hauteur, ratio conservé)
- Prévisualisation miniature dans la file de conversion
- Conversion unitaire et par lot (batch)

#### Conversion de documents
- Formats : PDF, DOCX, Markdown (.md), HTML, RST, ODT, EPUB
- Paires supportées :
  - DOCX → PDF, HTML, Markdown
  - Markdown → PDF, HTML, DOCX
  - HTML → PDF, Markdown
  - PDF → HTML (texte extrait)

#### Conversion audio
- Formats : MP3, FLAC, OGG, WAV, AAC
- Options : débit (bitrate en kbps)
- Moteur : FFmpeg sidecar

#### Conversion vidéo
- Formats : MP4, MKV, WebM, MOV
- Options : codec (H.264, H.265, VP9), résolution (largeur × hauteur, ratio conservé)
- Prévisualisation miniature dans la file de conversion
- Moteur : FFmpeg sidecar

#### Interface
- Glisser-déposer (drag & drop) de fichiers et de dossiers
- Sélecteur de fichiers alternatif (bouton)
- Sélection du format de sortie par menu déroulant
- Sélection du dossier de destination
- File de conversion avec état par fichier (en attente / en cours / terminé / erreur)
- Notification visuelle et système (OS) à la fin d'une conversion par lot
- Navigation par catégorie : Images, Documents, Audio, Vidéo
- Page de préférences avec persistance (format, répertoire, qualité, débit, codec)
- Support multilingue : français et anglais (bascule dans la nav)

### 2.2 Fonctionnalités secondaires

- Préférences : format de sortie par défaut par catégorie, dossier de destination par défaut
- Mise à jour automatique (vérification au lancement via Tauri updater)
- Navigation clavier complète et labels ARIA

### 2.3 Hors périmètre

- Interface TUI (prévue post-v1.x)
- Interface CLI
- Plugins / formats tiers
- Synchronisation cloud

---

## 3. Exigences non fonctionnelles

### Performance
- Conversion d'une image < 10 Mo en moins de 3 secondes
- L'interface ne se fige pas pendant une conversion (traitement asynchrone)
- Batch de 20 images < 30 secondes

### Légèreté
- Taille de l'installeur : < 100 Mo (incluant les sidecars FFmpeg et Pandoc)
- Consommation mémoire au repos : < 100 Mo RAM

### Confidentialité
- Aucune télémétrie
- Aucune connexion réseau sortante (sauf vérification de mise à jour opt-in)
- Aucune donnée utilisateur transmise

### Compatibilité
- Linux : Ubuntu 22.04+, Fedora 38+, Arch (via AppImage ou .deb)
- Windows : 10 et 11 (x64)
- macOS : 12+ (Apple Silicon)

### Accessibilité
- Navigation clavier complète
- Labels ARIA sur tous les éléments interactifs
- Contraste suffisant (WCAG AA)

---

## 4. Contraintes techniques

- Backend : **Rust** via **Tauri v2**
- Frontend : **Vue 3** (Composition API) + **Vite** + **Tailwind CSS**
- Gestion d'état : **Pinia**
- i18n : **vue-i18n** (EN + FR)
- Conversions images natives : crate **image**
- Conversions avancées : **FFmpeg** et **Pandoc** (bundlés comme sidecars Tauri)
- Monorepo : **pnpm workspaces**
- Tests : **Vitest** (Vue) + tests unitaires Rust
- CI/CD : **GitHub Actions**
- Releases : **Semantic Release** (Conventional Commits)
- Licence : **MIT**

---

## 5. Critères d'acceptation par feature

### Drag & drop
- [ ] Déposer un ou plusieurs fichiers déclenche leur ajout à la file
- [ ] Déposer un dossier ajoute tous les fichiers compatibles qu'il contient
- [ ] Les fichiers de format non supporté sont signalés clairement (pas silencieusement ignorés)

### Conversion image
- [ ] JPEG → PNG, WebP, AVIF avec option qualité
- [ ] PNG → JPEG, WebP avec option qualité
- [ ] Redimensionnement optionnel avec conservation du ratio
- [ ] Batch de 5 fichiers converti sans erreur
- [ ] Le fichier source n'est jamais modifié ni supprimé

### Conversion document
- [ ] Markdown → PDF fonctionne avec du contenu standard (titres, listes, liens)
- [ ] DOCX → PDF produit un fichier lisible
- [ ] Les erreurs Pandoc sont affichées à l'utilisateur

### Conversion audio
- [ ] MP3 → FLAC, OGG, WAV avec option bitrate
- [ ] Les erreurs FFmpeg sont affichées à l'utilisateur

### Conversion vidéo
- [ ] MP4 → MKV, WebM, MOV avec sélection du codec
- [ ] Résolution optionnelle avec conservation du ratio
- [ ] Les erreurs FFmpeg sont affichées à l'utilisateur

### File de conversion
- [ ] Chaque fichier a un état visible (en attente / en cours / terminé / erreur)
- [ ] L'utilisateur peut annuler une conversion en attente
- [ ] L'utilisateur peut réessayer une conversion en erreur

---

## 6. Landing page

### Objectif
Présenter le projet, convaincre de l'installer, faciliter le téléchargement.

### Contenu
1. **Nav** : logo, lien GitHub, bascule de langue EN/FR
2. **Hero** : titre, accroche, mockup CSS de l'interface, boutons de téléchargement (Linux / Windows / macOS)
3. **Fonctionnalités** : cards par catégorie (Images, Documents, Audio, Vidéo, Local, Open source)
4. **Promesse privacy** : section "Vos fichiers restent sur votre machine"
5. **Téléchargement** : liens dynamiques vers les artifacts de la dernière release GitHub (cache localStorage 1h)
6. **Footer** : lien GitHub, licence

### Contraintes
- Hébergement Vercel (déploiement automatique depuis `main`)
- Responsive (desktop + mobile)
- Pas de tracker, pas de cookie banner
