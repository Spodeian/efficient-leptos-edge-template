# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- `cargo generate` failed with "Substitution skipped, found invalid syntax" and created no files: workflows, `crates/shared/src/export.rs` and binary assets are now copied without Liquid substitution.
- `CHANGELOG.md`, `CONTRIBUTING.md`, `Cargo.lock` and build output are no longer copied into generated projects (moved from `exclude` to `ignore`).
- Generated projects take their own name in `wrangler.toml` and the `repository` URL instead of deploying over this template's Worker; this repo's own deploy passes `--name` from the repository name.
- `scripts/build-web.sh` requires `_headers`, `_redirects`, `sw.js` and `manifest.json` only when the project ships them, so PWA- or Cloudflare-off projects build.

### Changed
- CI job `template-scaffold-check` runs `cargo generate` with defaults and with every option off, then builds and tests the result.

## [0.1.0] - 2026-11-18

### Added
- Initial 4-crate pure serverless edge Leptos 0.8 starter (`crates/shared`, `crates/app`, `crates/web`, `crates/ssg`) engineered exclusively for Static SSR + WASM Hydration deployments on Cloudflare Pages, GitHub Pages, Netlify, and S3/CloudFront (zero desktop, Tauri, or CSR overhead).
- Pre-rendered route pipeline in `crates/ssg` generating `index.html`, `analytics.html`, `about.html`, `404.html`, `sitemap.xml`, `robots.txt`, hash-stamped `sw.js`, and SHA-256 CSP inline-script hashes in `_headers` (paired with `_redirects` 404 edge rules).
- Static SSR `<head>` shell (`crates/app/src/shell.rs`) with FOUT-free pre-paint theme/dyslexia synchronization, Schema.org `WebApplication` JSON-LD metadata (`EDGE_JSON_LD_SCRIPT`), OpenGraph/Twitter/canonical tags, and `<noscript>` guidance.
- Cross-target compute support in `crates/shared`: `spodeian-parallel` (`sample_points`, `batch_harmonic_sweep`) executing via Rayon during build-time SSR / host tests and inline WASM SIMD128 during client hydration, plus bundled `PARAMETRIC_HARMONIC_WGSL_SHADER` (`@compute @workgroup_size(64)`) and `EdgeComputeSummary`.
- Full productivity suite: `/analytics` SVG chart + harmonic explorer, searchable `CommandPaletteModal` (`Ctrl+K`), 64-step `UndoStack<ItemCollection>` (`Ctrl+Z`/`Ctrl+Y`), multi-tier `spodeian-cache` persistence (LocalStorage + IndexedDB), JSON/CSV/Compressed-BSON/SVG/HTML/`#state=` share-link interchange, modal Tab traps, and WCAG AA contrast validation tests.
