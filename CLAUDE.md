# Jnana — Claude Code Instructions

## Project Identity

**Jnana** (Sanskrit: ज्ञान — knowledge, wisdom) — The foundation of knowing. Unified knowledge system for AGNOS.

- **Type**: Flat library crate + content directory
- **License**: GPL-3.0
- **MSRV**: 1.89
- **Version**: SemVer 0.1.0

## What This Is

Jnana is the knowledge layer of AGNOS. It distills human understanding into structured, tested, queryable, offline-accessible data — 1-10GB of proven knowledge that fits on a phone.

**Internal knowledge** comes from AGNOS science crates (hisab, prakash, kimiya, tanmatra, bodh, vidya, etc.) — every formula verified by a test, every constant from an authoritative source.

**External knowledge** is curated from the best open sources (medical guides, survival manuals, agricultural science, repair guides) — downloaded, verified, and served alongside internal knowledge.

## Key Types

- `Entry` — A single piece of knowledge (fact, constant, procedure, table)
- `Domain` — 20 knowledge domains from Mathematics to Geography
- `Profile` — Storage presets (survival 2GB, homesteader 5GB, developer 3GB, educator 8GB, full 10GB)
- `Source` — External content source (ZIM, PDF, dataset) with download/verify
- `Registry` — In-memory knowledge store with search
- `StorageBudget` — What fits on this disk for this profile

## Content Standards

- Every constant MUST cite its authority (CODATA, NIST, etc.)
- Every procedure MUST include warnings and requirements
- Every fact MUST have a verification path (test name, citation)
- Internal knowledge references the AGNOS crate that provides it
- External sources track checksum for integrity verification

## Development Process

Same P(-1) + Work Loop as all AGNOS crates. See first-party-standards.md.

## DO NOT

- **Do not commit or push** — the user handles all git operations
- **NEVER use `gh` CLI** — use `curl` to GitHub API only
- Do not add unnecessary dependencies
- Do not include unverified facts — every entry needs a source
- Do not include copyrighted content — public domain and CC only
