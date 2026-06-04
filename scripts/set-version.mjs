#!/usr/bin/env node
// Updates version in tauri.conf.json, Cargo.toml, and apps/desktop/package.json.
// Called by Semantic Release: node scripts/set-version.js <version>

import { readFileSync, writeFileSync } from 'fs'
import { resolve, dirname } from 'path'
import { fileURLToPath } from 'url'

const version = process.argv[2]
if (!version) {
  console.error('Usage: set-version.js <version>')
  process.exit(1)
}

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..')

// apps/desktop/package.json
const desktopPkg = resolve(root, 'apps/desktop/package.json')
const pkg = JSON.parse(readFileSync(desktopPkg, 'utf8'))
pkg.version = version
writeFileSync(desktopPkg, JSON.stringify(pkg, null, 2) + '\n')
console.log(`  apps/desktop/package.json -> ${version}`)

// apps/desktop/src-tauri/tauri.conf.json
const tauriConf = resolve(root, 'apps/desktop/src-tauri/tauri.conf.json')
const conf = JSON.parse(readFileSync(tauriConf, 'utf8'))
conf.version = version
writeFileSync(tauriConf, JSON.stringify(conf, null, 2) + '\n')
console.log(`  tauri.conf.json -> ${version}`)

// apps/desktop/src-tauri/Cargo.toml  (first version = "..." line only)
const cargoPath = resolve(root, 'apps/desktop/src-tauri/Cargo.toml')
const cargo = readFileSync(cargoPath, 'utf8')
const updated = cargo.replace(/^version = "[^"]*"/m, `version = "${version}"`)
writeFileSync(cargoPath, updated)
console.log(`  Cargo.toml -> ${version}`)
