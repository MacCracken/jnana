# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
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

### Changed
- `Domain::agnos_crate()` replaced by `Domain::agnos_crates()` returning `&[&str]`

## [0.1.0] — 2025-03-28

### Added
- Core knowledge types: Entry, Fact, Constant, Procedure, Table
- 20 knowledge domains with AGNOS crate mappings
- In-memory Registry with search and domain filtering
- Cross-domain search with relevance scoring
- Storage profiles: Survival (2GB), Developer (3GB), Homesteader (5GB), Educator (8GB), Full (10GB)
- Budget calculator for storage planning
- External source management (ZIM, PDF, OSM, File)
- Portal configuration placeholder
- Criterion benchmarks

[Unreleased]: https://github.com/MacCracken/jnana/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/MacCracken/jnana/releases/tag/v0.1.0
