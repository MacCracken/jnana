# Roadmap

## Completed

### v0.1.0 — Foundation

- [x] Core knowledge types: Entry, Fact, Constant, Procedure, Table
- [x] 20 knowledge domains with AGNOS crate mappings
- [x] In-memory Registry with HashMap backing
- [x] Cross-domain search with relevance scoring
- [x] Storage profiles: Survival, Developer, Homesteader, Educator, Full
- [x] Budget calculator (what fits on this disk)
- [x] External source management (ZIM, PDF, OSM, File)
- [x] Portal configuration placeholder
- [x] Serde roundtrip for all types
- [x] P(-1) scaffold hardening

### v0.3.0 — AGNOS Integration

- [x] `KnowledgeProvider` trait for AGNOS crates to export knowledge entries
- [x] Feature-gated providers for kimiya, tanmatra, hisab, khanij (from crates.io)
- [x] Auto-registration via `Registry::with_agnos_providers()`
- [x] Cross-crate link resolution via shared tags (`linker::resolve_links`)
- [x] `Domain::agnos_crates()` refactor for programmatic crate lookup
- [x] `related` field on `Entry` for cross-references

### v0.2.0 — Content Pipeline

- [x] TOML-based content definitions in `content/sources/` (8 curated sources)
- [x] Profile definitions in `content/profiles/` (5 profiles)
- [x] Content loader (`content::load_sources`, `content::load_profiles`)
- [x] SHA-256 checksum computation and verification (`download` feature)
- [x] Source download with progress reporting (local/file:// — HTTP deferred)
- [x] Content indexing from source metadata into Registry

### v0.4.0 — Portal

- [x] Self-contained HTML portal generation (`portal::generate`)
- [x] Kiwix ZIM integration — auto-links to localhost kiwix server for ZIM sources
- [x] Client-side search with domain filtering (embedded JS, no deps)
- [x] Dark theme, responsive, entries + sources rendered as cards

### v1.0.0 — Stable

- [x] All content pipelines verified (TOML loading, indexing, portal generation)
- [x] 138 tests across all modules, integration tests cover full pipeline
- [x] Benchmark baseline established (11 benchmarks via criterion)
- [x] Full documentation — zero doc warnings, all public items documented

## Planned

### v0.5.0 — AI Integration

- [ ] MCP tools via bote: `jnana_search`, `jnana_lookup`, `jnana_budget`
- [ ] Daimon integration for grounded knowledge queries
- [ ] Structured responses for AI consumption
- [ ] **Upstream**: bote and daimon will likely need updates to support jnana's tool/plugin interfaces
