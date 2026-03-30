//! Mathematics knowledge provider — extracts from the `hisab` crate.

use crate::domain::Domain;
use crate::entry::{Constant, Entry, EntryKind, Fact};
use crate::provider::KnowledgeProvider;

/// Provides mathematical constants and reference data from the hisab crate.
pub struct HisabProvider;

impl KnowledgeProvider for HisabProvider {
    fn source_name(&self) -> &str {
        "hisab"
    }

    fn domain(&self) -> Domain {
        Domain::Mathematics
    }

    fn entries(&self) -> Vec<Entry> {
        vec![
            // ── Fundamental mathematical constants (from std, verified by hisab) ──
            Entry::new(
                "pi",
                "Pi",
                Domain::Mathematics,
                "Ratio of a circle's circumference to its diameter.",
                EntryKind::Constant(Constant {
                    symbol: "\u{03c0}".into(),
                    value: "3.14159265358979323846".into(),
                    unit: "".into(),
                    numeric: std::f64::consts::PI,
                    uncertainty: None,
                    authority: "exact (transcendental)".into(),
                }),
                "hisab",
                vec![
                    "circle".into(),
                    "geometry".into(),
                    "fundamental".into(),
                    "transcendental".into(),
                ],
            ),
            Entry::new(
                "tau",
                "Tau",
                Domain::Mathematics,
                "Full circle constant, equal to 2\u{03c0}.",
                EntryKind::Constant(Constant {
                    symbol: "\u{03c4}".into(),
                    value: "6.28318530717958647692".into(),
                    unit: "".into(),
                    numeric: std::f64::consts::TAU,
                    uncertainty: None,
                    authority: "exact (2\u{03c0})".into(),
                }),
                "hisab",
                vec!["circle".into(), "geometry".into(), "radians".into()],
            ),
            Entry::new(
                "eulers_number",
                "Euler's Number",
                Domain::Mathematics,
                "Base of the natural logarithm.",
                EntryKind::Constant(Constant {
                    symbol: "e".into(),
                    value: "2.71828182845904523536".into(),
                    unit: "".into(),
                    numeric: std::f64::consts::E,
                    uncertainty: None,
                    authority: "exact (transcendental)".into(),
                }),
                "hisab",
                vec![
                    "euler".into(),
                    "logarithm".into(),
                    "exponential".into(),
                    "fundamental".into(),
                    "transcendental".into(),
                ],
            ),
            Entry::new(
                "sqrt_2",
                "Square Root of 2",
                Domain::Mathematics,
                "The first known irrational number, diagonal of a unit square.",
                EntryKind::Constant(Constant {
                    symbol: "\u{221a}2".into(),
                    value: "1.41421356237309504880".into(),
                    unit: "".into(),
                    numeric: std::f64::consts::SQRT_2,
                    uncertainty: None,
                    authority: "exact (irrational)".into(),
                }),
                "hisab",
                vec!["irrational".into(), "geometry".into(), "algebraic".into()],
            ),
            Entry::new(
                "golden_ratio",
                "Golden Ratio",
                Domain::Mathematics,
                "The golden ratio, (1 + \u{221a}5) / 2.",
                EntryKind::Constant(Constant {
                    symbol: "\u{03c6}".into(),
                    value: "1.61803398874989484820".into(),
                    unit: "".into(),
                    numeric: (1.0 + 5.0_f64.sqrt()) / 2.0,
                    uncertainty: None,
                    authority: "exact (irrational)".into(),
                }),
                "hisab",
                vec![
                    "golden".into(),
                    "fibonacci".into(),
                    "geometry".into(),
                    "irrational".into(),
                ],
            ),
            // ── hisab-specific precision constants ────────────────────
            Entry::new(
                "epsilon_f64",
                "Machine Epsilon (f64, hisab)",
                Domain::Mathematics,
                "Default floating-point comparison tolerance used by hisab.",
                EntryKind::Constant(Constant {
                    symbol: "\u{03b5}".into(),
                    value: "1e-12".into(),
                    unit: "".into(),
                    numeric: hisab::EPSILON_F64,
                    uncertainty: None,
                    authority: "hisab convention".into(),
                }),
                "hisab",
                vec![
                    "epsilon".into(),
                    "precision".into(),
                    "floating-point".into(),
                    "tolerance".into(),
                ],
            ),
            Entry::new(
                "epsilon_f32",
                "Machine Epsilon (f32, hisab)",
                Domain::Mathematics,
                "Default floating-point comparison tolerance for f32 used by hisab.",
                EntryKind::Fact(Fact {
                    statement: "hisab uses 1e-7 as the default f32 comparison tolerance."
                        .into(),
                    explanation: "This is the EPSILON_F32 constant, used for approximate equality checks in single-precision geometry and transforms.".into(),
                    verification: Some("hisab::EPSILON_F32 == 1e-7".into()),
                }),
                "hisab",
                vec![
                    "epsilon".into(),
                    "precision".into(),
                    "floating-point".into(),
                    "f32".into(),
                ],
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_metadata() {
        let p = HisabProvider;
        assert_eq!(p.source_name(), "hisab");
        assert_eq!(p.domain(), Domain::Mathematics);
    }

    #[test]
    fn entries_cover_fundamental_constants() {
        let entries = HisabProvider.entries();
        let ids: Vec<&str> = entries.iter().map(|e| e.id.as_str()).collect();
        assert!(ids.contains(&"pi"));
        assert!(ids.contains(&"eulers_number"));
        assert!(ids.contains(&"golden_ratio"));
        assert!(ids.contains(&"epsilon_f64"));
    }

    #[test]
    fn pi_value_correct() {
        let entries = HisabProvider.entries();
        let pi = entries.iter().find(|e| e.id == "pi").unwrap();
        if let EntryKind::Constant(c) = &pi.kind {
            assert!((c.numeric - std::f64::consts::PI).abs() < 1e-15);
        } else {
            panic!("expected Constant");
        }
    }
}
