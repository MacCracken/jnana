# Jnana

> **ज्ञान** (Sanskrit: knowledge, wisdom) — The foundation of knowing

Jnana is the unified knowledge system for AGNOS. It distills human understanding into structured, tested, queryable, offline-accessible data that fits in 1-10GB.

## Philosophy

Wikipedia is 50GB of prose. Jnana is 1-10GB of *proven, testable, executable* knowledge. Every formula is verified by a test. Every constant comes from an authoritative source. Every procedure works.

## Two Layers

**Internal** — Knowledge from AGNOS science crates, distilled and structured:
- hisab (math), prakash (optics), kimiya (chemistry), tanmatra (nuclear physics)
- bodh (psychology), sangha (sociology), pramana (statistics), vidya (programming)
- Every formula verified, every constant cited, every law tested

**External** — Curated open-source references, downloaded and verified:
- Medical encyclopedias, field medicine guides
- Survival procedures, emergency preparedness
- Agricultural science, construction, repair
- All public domain or Creative Commons

## Profiles

| Profile | Budget | Use Case |
|---------|--------|----------|
| Survival | 2 GB | Essential medical, survival, practical — fits on a phone |
| Developer | 3 GB | Programming, math, systems knowledge |
| Homesteader | 5 GB | Self-sufficiency: farming, building, repair, medicine |
| Educator | 8 GB | Broad coverage for teaching across all domains |
| Full | 10 GB | Everything — the complete foundation of knowing |

## Quick Start

```rust
use jnana::{Domain, Registry, SearchQuery};

let registry = Registry::new();
// Load internal knowledge from AGNOS crates
// Load external sources from content directory

// Search across all domains
let results = jnana::search::search(&registry, &SearchQuery::text("binding energy"));

// Budget: what fits on my 8GB USB stick?
let budget = jnana::storage::calculate(8000, &jnana::Profile::educator(), &sources);
```

## License

GPL-3.0
