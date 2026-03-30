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

## Planned

### v0.2.0 — Content Pipeline

- [ ] TOML-based content definitions in `content/sources/`
- [ ] Profile definitions in `content/profiles/`
- [ ] Source download with progress reporting
- [ ] SHA-256 checksum verification on download
- [ ] Content indexing from TOML files into Registry

### v0.4.0 — Portal

- [ ] HTML portal generation from Registry + Source list
- [ ] Kiwix ZIM integration for offline Wikipedia/WikiMed
- [ ] Search UI with domain filtering

### v0.5.0 — AI Integration

- [ ] MCP tools via bote: `jnana_search`, `jnana_lookup`, `jnana_budget`
- [ ] Daimon integration for grounded knowledge queries
- [ ] Structured responses for AI consumption
- [ ] **Upstream**: bote and daimon will likely need updates to support jnana's tool/plugin interfaces

### v1.0.0 — Stable

- [ ] All content pipelines verified
- [ ] 80%+ test coverage
- [ ] Benchmark baseline established and tracked
- [ ] Full documentation with examples
- [ ] Published to crates.io
