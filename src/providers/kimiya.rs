//! Chemistry knowledge provider — extracts from the `kimiya` crate.

use crate::domain::Domain;
use crate::entry::{Constant, Entry, EntryKind, Table};
use crate::provider::KnowledgeProvider;

/// Provides chemistry knowledge from the kimiya crate.
pub struct KimiyaProvider;

impl KnowledgeProvider for KimiyaProvider {
    fn source_name(&self) -> &str {
        "kimiya"
    }

    fn domain(&self) -> Domain {
        Domain::Chemistry
    }

    fn entries(&self) -> Vec<Entry> {
        let mut entries = Vec::new();

        // ── Periodic table ────────────────────────────────────────────
        entries.push(Entry::new(
            "periodic_table",
            "Periodic Table of Elements",
            Domain::Chemistry,
            "All 118 elements with atomic number, symbol, name, mass, electronegativity, and category.",
            EntryKind::Table(Table {
                columns: vec![
                    "Z".into(),
                    "Symbol".into(),
                    "Name".into(),
                    "Mass (u)".into(),
                    "Electronegativity".into(),
                    "Category".into(),
                ],
                rows: kimiya::ELEMENTS
                    .iter()
                    .map(|e| {
                        vec![
                            e.atomic_number.to_string(),
                            e.symbol.to_string(),
                            e.name.to_string(),
                            format!("{:.4}", e.atomic_mass),
                            e.electronegativity
                                .map_or("-".into(), |v| format!("{:.2}", v)),
                            format!("{:?}", e.category),
                        ]
                    })
                    .collect(),
                description: "Full periodic table (Z=1 to Z=118) from kimiya.".into(),
            }),
            "kimiya",
            vec![
                "elements".into(),
                "periodic-table".into(),
                "chemistry".into(),
                "atoms".into(),
            ],
        ));

        // ── Avogadro's number ─────────────────────────────────────────
        entries.push(Entry::new(
            "avogadro_number",
            "Avogadro's Number",
            Domain::Chemistry,
            "Number of particles in one mole of substance.",
            EntryKind::Constant(Constant {
                symbol: "N_A".into(),
                value: "6.02214076e23".into(),
                unit: "mol\u{207b}\u{00b9}".into(),
                numeric: kimiya::element::AVOGADRO,
                uncertainty: None,
                authority: "CODATA 2022 (exact)".into(),
            }),
            "kimiya",
            vec![
                "avogadro".into(),
                "mole".into(),
                "fundamental".into(),
                "chemistry".into(),
            ],
        ));

        // ── Gas constant ──────────────────────────────────────────────
        entries.push(Entry::new(
            "gas_constant",
            "Universal Gas Constant",
            Domain::Chemistry,
            "The molar gas constant, relating energy scale to temperature scale.",
            EntryKind::Constant(Constant {
                symbol: "R".into(),
                value: "8.314462618".into(),
                unit: "J/(mol\u{00b7}K)".into(),
                numeric: kimiya::element::GAS_CONSTANT,
                uncertainty: None,
                authority: "CODATA 2022 (exact)".into(),
            }),
            "kimiya",
            vec![
                "gas".into(),
                "thermodynamics".into(),
                "fundamental".into(),
                "chemistry".into(),
            ],
        ));

        // ── Faraday constant ──────────────────────────────────────────
        entries.push(Entry::new(
            "faraday_constant",
            "Faraday Constant",
            Domain::Chemistry,
            "Electric charge per mole of electrons.",
            EntryKind::Constant(Constant {
                symbol: "F".into(),
                value: "96485.33212".into(),
                unit: "C/mol".into(),
                numeric: kimiya::FARADAY,
                uncertainty: None,
                authority: "CODATA 2022 (exact)".into(),
            }),
            "kimiya",
            vec![
                "faraday".into(),
                "electrochemistry".into(),
                "fundamental".into(),
                "chemistry".into(),
            ],
        ));

        entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_metadata() {
        let p = KimiyaProvider;
        assert_eq!(p.source_name(), "kimiya");
        assert_eq!(p.domain(), Domain::Chemistry);
    }

    #[test]
    fn entries_not_empty() {
        let entries = KimiyaProvider.entries();
        assert!(entries.len() >= 4);
    }

    #[test]
    fn periodic_table_has_118_rows() {
        let entries = KimiyaProvider.entries();
        let table = entries.iter().find(|e| e.id == "periodic_table").unwrap();
        if let EntryKind::Table(t) = &table.kind {
            assert_eq!(t.rows.len(), 118);
            assert_eq!(t.columns.len(), 6);
        } else {
            panic!("expected Table");
        }
    }

    #[test]
    fn avogadro_value() {
        let entries = KimiyaProvider.entries();
        let entry = entries.iter().find(|e| e.id == "avogadro_number").unwrap();
        if let EntryKind::Constant(c) = &entry.kind {
            assert!((c.numeric - 6.022_140_76e23).abs() < 1e17);
        } else {
            panic!("expected Constant");
        }
    }
}
