# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] — 2026-03-29

### Added
- Core knowledge types: Entry, Fact, Constant, Procedure, Table
- 20 knowledge domains with AGNOS crate mappings
- In-memory Registry with search and domain filtering
- Cross-domain search with relevance scoring
- Storage profiles: Survival (2GB), Developer (3GB), Homesteader (5GB), Educator (8GB), Full (10GB)
- Budget calculator for storage planning
- External source management (ZIM, PDF, OSM, File)
- `KnowledgeProvider` trait for AGNOS science crate integration
- Feature-gated providers pulling from crates.io:
  - `kimiya` (1.1) — periodic table (118 elements), Avogadro, gas constant, Faraday constant
  - `tanmatra` (1.1) — 14 CODATA 2022 physics constants (c, h, e, k_B, etc.)
  - `hisab` (1.3) — mathematical constants (pi, tau, e, sqrt2, golden ratio, epsilon)
  - `khanij` (1.1) — 15 common minerals, Mohs hardness scale, crystal systems
- `Registry::with_agnos_providers()` for auto-registration of all enabled providers
- `Registry::register_provider()` for custom provider registration
- `Registry::get_mut()` for mutable entry access
- `linker::resolve_links()` — cross-crate knowledge linking via shared tags
- `related` field on `Entry` for cross-references (`#[serde(default)]`)
- `agnos-all` feature flag to enable all science crate providers
- `Domain::agnos_crates()` returning `&[&str]` for programmatic crate lookup
- TOML content pipeline: `content::load_sources()`, `content::load_profiles()`
- 8 curated source definitions (WikiMed, Wikivoyage, Wiktionary, WHO field medicine, FAO agriculture, OSM, iFixit, Wikipedia)
- 5 profile definitions (survival, homesteader, developer, educator, full)
- `content::index_sources()` — index source metadata into Registry for unified search
- `download::download_source()` with progress callback (behind `download` feature)
- `download::verify_checksum()` and `download::compute_checksum()` — SHA-256 integrity verification
- HTML portal generation (`portal::generate`) — self-contained page with embedded CSS/JS
- Client-side search with domain filtering in portal
- Kiwix ZIM auto-linking in portal (localhost:port/source_id)
- 138 tests across all modules with full pipeline integration tests
- 11 criterion benchmarks (search, registry, linker, portal, content indexing)

[0.5.0]: https://github.com/MacCracken/jnana/releases/tag/v0.5.0
