//! Core knowledge entry types.
//!
//! An [`Entry`] is a single piece of knowledge — a fact, a constant,
//! a procedure, or a data table. Entries are the atoms of the knowledge system.

use crate::domain::Domain;
use serde::{Deserialize, Serialize};

/// A single knowledge entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Entry {
    /// Unique identifier (e.g. "speed_of_light", "cpr_procedure").
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// Knowledge domain.
    pub domain: Domain,
    /// One-line summary.
    pub summary: String,
    /// The knowledge content.
    pub kind: EntryKind,
    /// Source attribution (crate name, book, URL).
    pub source: String,
    /// Tags for search.
    pub tags: Vec<String>,
    /// IDs of related entries (cross-references).
    #[serde(default)]
    pub related: Vec<String>,
}

/// The type of knowledge in an entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EntryKind {
    /// A verified fact or statement.
    Fact(Fact),
    /// A physical or mathematical constant.
    Constant(Constant),
    /// A step-by-step procedure.
    Procedure(Procedure),
    /// A data table (periodic table, unit conversions, etc.).
    Table(Table),
}

/// A verified fact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fact {
    /// The statement.
    pub statement: String,
    /// Supporting explanation.
    pub explanation: String,
    /// How this was verified (test name, citation).
    pub verification: Option<String>,
}

/// A physical or mathematical constant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constant {
    /// Symbol (e.g. "c", "G", "π").
    pub symbol: String,
    /// Value as a string (preserves precision).
    pub value: String,
    /// Unit (e.g. "m/s", "N⋅m²/kg²").
    pub unit: String,
    /// Numeric value for computation.
    pub numeric: f64,
    /// Uncertainty if known (e.g. "±0.000 000 015").
    pub uncertainty: Option<String>,
    /// Source (e.g. "CODATA 2022", "NIST").
    pub authority: String,
}

/// A step-by-step procedure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Procedure {
    /// When to use this procedure.
    pub when: String,
    /// Ordered steps.
    pub steps: Vec<String>,
    /// Warnings or critical notes.
    pub warnings: Vec<String>,
    /// What you need (tools, materials).
    pub requires: Vec<String>,
}

/// A data table (rows of structured data).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    /// Column headers.
    pub columns: Vec<String>,
    /// Row data (each row is a vec of cell values).
    pub rows: Vec<Vec<String>>,
    /// Description of what this table contains.
    pub description: String,
}

impl Entry {
    /// Create a new knowledge entry.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        domain: Domain,
        summary: impl Into<String>,
        kind: EntryKind,
        source: impl Into<String>,
        tags: Vec<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            domain,
            summary: summary.into(),
            kind,
            source: source.into(),
            tags,
            related: Vec::new(),
        }
    }

    /// Check if a tag matches (case-insensitive).
    #[must_use]
    #[inline]
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
    }

    /// Estimated size in bytes for storage budgeting.
    #[must_use]
    #[inline]
    pub fn estimated_size(&self) -> usize {
        self.summary.len()
            + self.source.len()
            + self.related.iter().map(|r| r.len()).sum::<usize>()
            + match &self.kind {
                EntryKind::Fact(f) => f.statement.len() + f.explanation.len(),
                EntryKind::Constant(c) => c.value.len() + c.unit.len() + c.authority.len(),
                EntryKind::Procedure(p) => {
                    p.steps.iter().map(|s| s.len()).sum::<usize>()
                        + p.warnings.iter().map(|w| w.len()).sum::<usize>()
                }
                EntryKind::Table(t) => t.rows.iter().flat_map(|r| r.iter()).map(|c| c.len()).sum(),
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_constant() -> Entry {
        Entry {
            id: "speed_of_light".into(),
            title: "Speed of Light in Vacuum".into(),
            domain: Domain::Physics,
            summary: "The speed of light in vacuum, a fundamental physical constant.".into(),
            kind: EntryKind::Constant(Constant {
                symbol: "c".into(),
                value: "299792458".into(),
                unit: "m/s".into(),
                numeric: 299_792_458.0,
                uncertainty: None,
                authority: "CODATA 2022 (exact)".into(),
            }),
            source: "prakash, tanmatra".into(),
            tags: vec![
                "light".into(),
                "speed".into(),
                "fundamental".into(),
                "exact".into(),
            ],
            related: vec![],
        }
    }

    fn sample_procedure() -> Entry {
        Entry {
            id: "cpr_adult".into(),
            title: "CPR for Adults".into(),
            domain: Domain::Medicine,
            summary: "Cardiopulmonary resuscitation for unresponsive adults.".into(),
            kind: EntryKind::Procedure(Procedure {
                when: "Person is unresponsive and not breathing normally.".into(),
                steps: vec![
                    "Call emergency services.".into(),
                    "Place heel of hand on center of chest.".into(),
                    "Push hard and fast — 100-120 compressions per minute, 2 inches deep.".into(),
                    "After 30 compressions, give 2 rescue breaths.".into(),
                    "Continue until help arrives or person recovers.".into(),
                ],
                warnings: vec![
                    "Do not stop compressions to check for pulse.".into(),
                    "Push hard enough — ribs may crack, that's expected.".into(),
                ],
                requires: vec!["Flat surface".into()],
            }),
            source: "Red Cross First Aid Manual".into(),
            tags: vec!["first-aid".into(), "emergency".into(), "cardiac".into()],
            related: vec![],
        }
    }

    #[test]
    fn entry_has_tag() {
        let e = sample_constant();
        assert!(e.has_tag("light"));
        assert!(e.has_tag("LIGHT"));
        assert!(!e.has_tag("gravity"));
    }

    #[test]
    fn entry_estimated_size() {
        let e = sample_constant();
        assert!(e.estimated_size() > 0);
    }

    #[test]
    fn constant_numeric() {
        if let EntryKind::Constant(c) = &sample_constant().kind {
            assert!((c.numeric - 299_792_458.0).abs() < 1.0);
        } else {
            panic!("expected constant");
        }
    }

    #[test]
    fn procedure_steps() {
        if let EntryKind::Procedure(p) = &sample_procedure().kind {
            assert_eq!(p.steps.len(), 5);
            assert!(!p.warnings.is_empty());
        } else {
            panic!("expected procedure");
        }
    }

    #[test]
    fn entry_serde_roundtrip() {
        let e = sample_constant();
        let json = serde_json::to_string(&e).unwrap();
        let decoded: Entry = serde_json::from_str(&json).unwrap();
        assert_eq!(e.id, decoded.id);
        assert_eq!(e.domain, decoded.domain);
    }

    #[test]
    fn procedure_serde_roundtrip() {
        let e = sample_procedure();
        let json = serde_json::to_string(&e).unwrap();
        let decoded: Entry = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.id, "cpr_adult");
    }

    #[test]
    fn has_tag_case_insensitive() {
        let e = sample_constant();
        assert!(e.has_tag("Light"));
        assert!(e.has_tag("EXACT"));
        assert!(e.has_tag("fundamental"));
    }

    #[test]
    fn has_tag_no_match() {
        let e = sample_constant();
        assert!(!e.has_tag("quantum"));
        assert!(!e.has_tag(""));
    }

    #[test]
    fn estimated_size_table() {
        let e = Entry::new(
            "elements",
            "Periodic Table",
            Domain::Chemistry,
            "The periodic table.",
            EntryKind::Table(Table {
                columns: vec!["Symbol".into(), "Name".into()],
                rows: vec![
                    vec!["H".into(), "Hydrogen".into()],
                    vec!["He".into(), "Helium".into()],
                ],
                description: "First two elements.".into(),
            }),
            "kimiya",
            vec![],
        );
        assert!(e.estimated_size() > 0);
    }

    #[test]
    fn entry_new_constructor() {
        let e = Entry::new(
            "test",
            "Test Entry",
            Domain::Mathematics,
            "A test.",
            EntryKind::Fact(Fact {
                statement: "1+1=2".into(),
                explanation: "Arithmetic.".into(),
                verification: Some("test_addition".into()),
            }),
            "hisab",
            vec!["math".into()],
        );
        assert_eq!(e.id, "test");
        assert_eq!(e.domain, Domain::Mathematics);
        assert!(e.has_tag("math"));
        assert!(e.related.is_empty(), "new entries should have empty related");
    }

    #[test]
    fn estimated_size_procedure() {
        let e = sample_procedure();
        let size = e.estimated_size();
        assert!(size > 0);
        // Procedure size includes steps + warnings
        if let EntryKind::Procedure(p) = &e.kind {
            let step_size: usize = p.steps.iter().map(|s| s.len()).sum();
            assert!(size >= step_size);
        }
    }

    #[test]
    fn estimated_size_fact() {
        let e = Entry::new(
            "fact",
            "Fact",
            Domain::Mathematics,
            "A fact.",
            EntryKind::Fact(Fact {
                statement: "Water boils at 100C at 1 atm.".into(),
                explanation: "Standard boiling point.".into(),
                verification: None,
            }),
            "test",
            vec![],
        );
        assert!(e.estimated_size() > 0);
    }

    #[test]
    fn estimated_size_includes_related() {
        let mut e = sample_constant();
        let size_without = e.estimated_size();
        e.related = vec!["some_related_entry".into(), "another_entry".into()];
        let size_with = e.estimated_size();
        assert!(size_with > size_without);
    }

    #[test]
    fn serde_roundtrip_with_related() {
        let mut e = sample_constant();
        e.related = vec!["pi".into(), "planck".into()];
        let json = serde_json::to_string(&e).unwrap();
        let decoded: Entry = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.related, vec!["pi", "planck"]);
    }

    #[test]
    fn serde_roundtrip_without_related() {
        // Verify #[serde(default)] works — JSON without "related" field
        let json = r#"{
            "id": "test",
            "title": "Test",
            "domain": "Physics",
            "summary": "Test entry.",
            "kind": {"Fact": {"statement": "x", "explanation": "y", "verification": null}},
            "source": "test",
            "tags": []
        }"#;
        let decoded: Entry = serde_json::from_str(json).unwrap();
        assert!(decoded.related.is_empty());
    }
}
