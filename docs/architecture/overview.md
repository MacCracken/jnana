# Architecture Overview

## Module Map

```
jnana
├── domain      — 20 knowledge domains with AGNOS crate mappings
├── entry       — Entry, EntryKind (Fact, Constant, Procedure, Table)
├── registry    — In-memory HashMap registry with lookup, search, domain filtering
├── search      — Cross-domain search with relevance scoring
├── profile     — Storage profiles (survival, homesteader, developer, educator, full)
├── storage     — Budget calculator: what fits on this disk for this profile
├── source      — External source management (ZIM, PDF, OSM, File)
├── portal      — Web portal generation from registry           [feature: serve]
├── logging     — Structured logging via tracing                [feature: logging]
└── error       — JnanaError, Result alias
```

## Feature Flags

| Flag | Dependencies | Description |
|------|-------------|-------------|
| `std` | serde/std, thiserror/std | Standard library support (default) |
| `logging` | tracing-subscriber | Structured logging with JNANA_LOG env |
| `serve` | — | Web portal generation |
| `download` | — | External source downloading |
| `full` | std + logging + serve + download | All features |

None except `std` are enabled by default. The core library (domain, entry, registry, search, profile, storage, source, error) has minimal dependencies.

## Design Principles

- **Proven knowledge** — every constant cites its authority, every procedure is verified
- **Offline-first** — 1-10GB on a phone, no network required
- **Feature-gated** — heavy dependencies are opt-in
- **Zero unsafe** — no `unsafe` blocks anywhere
- **Thread-safe** — all public types are `Send + Sync`

## Data Flow

```
AGNOS Crates (hisab, kimiya, etc.)
        │
        ▼
  Entry::new() → Registry::register()
        │                    │
        │          ┌─────────┼──────────┐
        │          ▼         ▼          ▼
        │     search()   by_domain()  list()
        │          │         │          │
        │          └─────────┼──────────┘
        │                    ▼
        │            SearchResult / &Entry
        │
External Sources (ZIM, PDF)
        │
        ▼
  Source → StorageBudget::calculate()
                    │
                    ▼
            fits[] / excluded[]
```

## Consumers

| Project | Usage |
|---------|-------|
| daimon | AI agent queries jnana for grounded knowledge |
| agnoshi | CLI knowledge browser |
| portal | Offline web interface to jnana registry |
