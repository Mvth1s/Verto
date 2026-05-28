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

### 2.1 Fonctionnalités principales (v1.0)

#### Conversion d'images
- Formats supportés en entrée/sortie : JPEG, PNG, WebP, BMP, TIFF, GIF, AVIF
- Options : qualité (0-100), redimensionnement (largeur × hauteur, ratio conservé)
- Conversion unitaire et par lot (batch)

#### Conversion de documents
- Formats : PDF, DOCX, Markdown (.md), HTML, ODT
- Paires supportées :
  - DOCX → PDF, HTML, Markdown
  - Markdown → PDF, HTML, DOCX
  - HTML → PDF, Markdown
  - PDF → HTML (texte extrait)

#### Interface
- Glisser-déposer (drag & drop) de fichiers et de dossiers
- Sélecteur de fichiers alternatif (bouton)
- Sélection du format de sortie par menu déroulant
- Sélection du dossier de destination
- File de conversion avec état par fichier (en attente / en cours / terminé / erreur)
- Notification visuelle de fin de conversion
- Accès direct au dossier de sortie depuis l'app

### 2.2 Fonctionnalités secondaires (v1.0)

- Historique des dernières conversions (session courante)
- Préférences : format de sortie par défaut par catégorie, dossier de destination par défaut
- Support multilingue : français, anglais (base i18n extensible)

### 2.3 Hors périmètre v1.0

- Conversion audio et vidéo (prévue v0.3 et v1.1)
- Interface TUI (prévue post-v1.0)
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
- Aucune connexion réseau sortante
- Aucune donnée utilisateur transmise

### Compatibilité
- Linux : Ubuntu 22.04+, Fedora 38+, Arch (via AppImage ou .deb)
- Windows : 10 et 11 (x64)
- macOS : 12+ (x64 et Apple Silicon)

### Accessibilité
- Navigation clavier complète
- Labels ARIA sur tous les éléments interactifs
- Contraste suffisant (WCAG AA)

---

## 4. Contraintes techniques

- Backend : **Rust** via **Tauri v2**
- Frontend : **Vue 3** (Composition API) + **Vite** + **Tailwind CSS**
- Gestion d'état : **Pinia**
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
- [ ] Batch de 5 fichiers converti sans erreur
- [ ] Le fichier source n'est jamais modifié ni supprimé

### Conversion document
- [ ] Markdown → PDF fonctionne avec du contenu standard (titres, listes, liens)
- [ ] DOCX → PDF produit un fichier lisible
- [ ] Les erreurs Pandoc sont affichées à l'utilisateur

### File de conversion
- [ ] Chaque fichier a un état visible (en attente / en cours / terminé / erreur)
- [ ] L'utilisateur peut annuler une conversion en attente
- [ ] L'utilisateur peut réessayer une conversion en erreur

---

## 6. Landing page

### Objectif
Présenter le projet, convaincre de l'installer, faciliter le téléchargement.

### Contenu
1. **Hero** : titre, accroche, screenshot de l'app, boutons de téléchargement (Linux / Windows / macOS)
2. **Fonctionnalités** : 4 cards (Images, Documents, Local, Open source)
3. **Promesse privacy** : section "Vos fichiers restent sur votre machine"
4. **Téléchargement** : liens vers les artifacts de la dernière release GitHub
5. **Footer** : lien GitHub, licence

### Contraintes
- Hébergement Vercel (déploiement automatique depuis `main`)
- Responsive (desktop + mobile)
- Pas de tracker, pas de cookie banner
