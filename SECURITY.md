# Security Policy

## Scope

Jnana is a knowledge management library providing structured, offline-accessible data for the AGNOS ecosystem. The core library performs no network I/O (download features are opt-in) and contains no `unsafe` code.

## Attack Surface

| Area | Risk | Mitigation |
|------|------|------------|
| Content deserialization | Crafted JSON/TOML entries | Serde derive validation; no arbitrary code execution |
| Search queries | ReDoS via crafted search text | Simple `contains()` matching, no regex |
| Storage budget | Integer overflow on size calculations | `saturating_sub()` arithmetic; u64 overflow infeasible |
| External sources | Tampered downloads | SHA-256 checksum verification (when checksum provided) |
| Source URLs | URL injection if used in system calls | URLs stored as data only; no shell execution in core |
| Portal generation | XSS via entry content | Feature-gated; HTML escaping required at render time |
| Dependencies | Supply chain compromise | cargo-deny, cargo-audit in CI; minimal core deps |

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x | Yes |

## Reporting

- Contact: **security@agnos.dev**
- Do not open public issues for security vulnerabilities
- 48-hour acknowledgement SLA
- 90-day coordinated disclosure

## Design Principles

- Zero `unsafe` code
- No `unwrap()` or `panic!()` in library code — all errors via `Result`
- All public types are `Send + Sync` (compile-time verified)
- No network I/O in core library (download is opt-in via feature flag)
- Minimal dependency surface (core depends only on serde, thiserror, tracing)
- Checksum verification for all external content
