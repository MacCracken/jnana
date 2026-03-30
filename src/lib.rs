//! Jnana — the foundation of knowing
//!
//! **Jnana** (Sanskrit: ज्ञान — knowledge, wisdom) is the unified knowledge
//! system for AGNOS. It distills human understanding into structured, tested,
//! queryable, offline-accessible data — drawing from AGNOS science crates
//! and curated external sources.
//!
//! # Philosophy
//!
//! Wikipedia is 50GB of prose. Jnana is 1-10GB of *proven, testable,
//! executable* knowledge. Every formula is verified by a test. Every
//! constant comes from an authoritative source. Every procedure works.
//!
//! # Knowledge Sources
//!
//! **Internal** (from AGNOS crates — distilled, tested):
//! - hisab: math formulas, linear algebra, calculus, geometry
//! - kimiya: periodic table, reactions, thermochemistry
//! - tanmatra: particle physics, nuclear data, spectral lines
//! - prakash: optics tables, spectral data, Fresnel equations
//! - pramana: probability distributions, statistical methods
//! - bodh: cognitive models, psychophysics laws
//! - vidya: programming best practices across languages
//! - ...every science crate's reference data
//!
//! **External** (curated, downloaded, verified):
//! - Medical references (WikiMed, field medicine guides)
//! - Survival procedures (shelter, water, food, navigation)
//! - Repair guides (iFixit, practical how-to)
//! - Agricultural science (FAO, permaculture, aquaponics)
//! - Construction (earthbag, timber, solar, plumbing)
//!
//! # Modules
//!
//! - [`domain`] — Knowledge domains and their entries
//! - [`entry`] — Core types: `Entry`, `Fact`, `Procedure`, `Constant`, `Table`
//! - [`registry`] — In-memory knowledge registry with lookup and search
//! - [`profile`] — Storage profiles (survival, homesteader, developer, educator, full)
//! - [`search`] — Cross-domain search
//! - [`source`] — External source management (download, verify, update)
//! - [`portal`] — Web portal generation from registry
//! - [`storage`] — Budget calculator, what-fits analysis
//! - [`error`] — Error types
//!
//! # Example
//!
//! ```rust
//! use jnana::{Registry, SearchQuery};
//!
//! let registry = Registry::new();
//! if let Some(entry) = registry.get("speed_of_light") {
//!     println!("{}: {}", entry.title, entry.summary);
//! }
//! let results = jnana::search::search(&registry, &SearchQuery::text("binding energy"));
//! ```

pub mod content;
pub mod domain;
pub mod entry;
pub mod error;
pub mod linker;
pub mod portal;
pub mod profile;
pub mod provider;
pub mod providers;
pub mod registry;
pub mod search;
pub mod source;
pub mod storage;

#[cfg(feature = "download")]
pub mod download;

#[cfg(feature = "logging")]
pub mod logging;

// ── Core types ─────────────────────────────────────────────────────────────
pub use domain::Domain;
pub use entry::{Constant, Entry, EntryKind, Fact, Procedure, Table};
pub use error::{JnanaError, Result};
pub use profile::Profile;
pub use provider::KnowledgeProvider;
pub use registry::Registry;
pub use search::{SearchQuery, SearchResult};
pub use source::Source;
pub use storage::StorageBudget;
