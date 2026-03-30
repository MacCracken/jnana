//! Earth science knowledge provider — extracts from the `khanij` crate.

use crate::domain::Domain;
use crate::entry::{Entry, EntryKind, Table};
use crate::provider::KnowledgeProvider;

/// Provides geology and mineralogy knowledge from the khanij crate.
pub struct KhanijProvider;

impl KnowledgeProvider for KhanijProvider {
    fn source_name(&self) -> &str {
        "khanij"
    }

    fn domain(&self) -> Domain {
        Domain::EarthScience
    }

    fn entries(&self) -> Vec<Entry> {
        let mut entries = Vec::new();

        // ── Common minerals table ─────────────────────────────────────
        let minerals = [
            khanij::Mineral::quartz(),
            khanij::Mineral::feldspar(),
            khanij::Mineral::calcite(),
            khanij::Mineral::diamond(),
            khanij::Mineral::talc(),
            khanij::Mineral::olivine(),
            khanij::Mineral::pyrite(),
            khanij::Mineral::magnetite(),
            khanij::Mineral::halite(),
            khanij::Mineral::gypsum(),
            khanij::Mineral::muscovite(),
            khanij::Mineral::fluorite(),
            khanij::Mineral::apatite(),
            khanij::Mineral::corundum(),
            khanij::Mineral::topaz(),
        ];

        entries.push(Entry::new(
            "common_minerals",
            "Common Minerals",
            Domain::EarthScience,
            "15 common minerals with hardness, density, crystal system, and luster.",
            EntryKind::Table(Table {
                columns: vec![
                    "Name".into(),
                    "Formula".into(),
                    "Hardness".into(),
                    "Density (g/cm\u{00b3})".into(),
                    "Crystal System".into(),
                    "Luster".into(),
                ],
                rows: minerals
                    .iter()
                    .map(|m| {
                        vec![
                            m.name.clone(),
                            m.formula.clone(),
                            format!("{:.1}", m.hardness.value()),
                            format!("{:.2}", m.density),
                            format!("{:?}", m.crystal_system),
                            format!("{:?}", m.luster),
                        ]
                    })
                    .collect(),
                description: "Common rock-forming and notable minerals from khanij.".into(),
            }),
            "khanij",
            vec![
                "minerals".into(),
                "geology".into(),
                "hardness".into(),
                "crystal".into(),
            ],
        ));

        // ── Mohs hardness scale ───────────────────────────────────────
        let mohs_minerals = [
            ("Talc", 1.0),
            ("Gypsum", 2.0),
            ("Calcite", 3.0),
            ("Fluorite", 4.0),
            ("Apatite", 5.0),
            ("Orthoclase", 6.0),
            ("Quartz", 7.0),
            ("Topaz", 8.0),
            ("Corundum", 9.0),
            ("Diamond", 10.0),
        ];

        entries.push(Entry::new(
            "mohs_hardness_scale",
            "Mohs Hardness Scale",
            Domain::EarthScience,
            "The 10-point mineral hardness reference scale.",
            EntryKind::Table(Table {
                columns: vec!["Hardness".into(), "Reference Mineral".into()],
                rows: mohs_minerals
                    .iter()
                    .map(|(name, h)| vec![format!("{h:.0}"), (*name).into()])
                    .collect(),
                description: "Mohs scale of mineral hardness (1 = softest, 10 = hardest).".into(),
            }),
            "khanij",
            vec![
                "mohs".into(),
                "hardness".into(),
                "minerals".into(),
                "identification".into(),
            ],
        ));

        // ── Crystal systems ───────────────────────────────────────────
        entries.push(Entry::new(
            "crystal_systems",
            "Crystal Systems",
            Domain::EarthScience,
            "The seven crystal systems that classify mineral structures.",
            EntryKind::Table(Table {
                columns: vec![
                    "System".into(),
                    "Axes".into(),
                    "Angles".into(),
                    "Example".into(),
                ],
                rows: vec![
                    vec![
                        "Cubic".into(),
                        "a = b = c".into(),
                        "\u{03b1} = \u{03b2} = \u{03b3} = 90\u{00b0}".into(),
                        "Diamond, Halite".into(),
                    ],
                    vec![
                        "Tetragonal".into(),
                        "a = b \u{2260} c".into(),
                        "\u{03b1} = \u{03b2} = \u{03b3} = 90\u{00b0}".into(),
                        "Zircon".into(),
                    ],
                    vec![
                        "Orthorhombic".into(),
                        "a \u{2260} b \u{2260} c".into(),
                        "\u{03b1} = \u{03b2} = \u{03b3} = 90\u{00b0}".into(),
                        "Olivine, Topaz".into(),
                    ],
                    vec![
                        "Hexagonal".into(),
                        "a = b \u{2260} c".into(),
                        "\u{03b1} = \u{03b2} = 90\u{00b0}, \u{03b3} = 120\u{00b0}".into(),
                        "Quartz, Calcite".into(),
                    ],
                    vec![
                        "Monoclinic".into(),
                        "a \u{2260} b \u{2260} c".into(),
                        "\u{03b1} = \u{03b3} = 90\u{00b0}, \u{03b2} \u{2260} 90\u{00b0}".into(),
                        "Feldspar, Gypsum".into(),
                    ],
                    vec![
                        "Triclinic".into(),
                        "a \u{2260} b \u{2260} c".into(),
                        "\u{03b1} \u{2260} \u{03b2} \u{2260} \u{03b3} \u{2260} 90\u{00b0}".into(),
                        "Plagioclase".into(),
                    ],
                    vec![
                        "Trigonal".into(),
                        "a = b = c".into(),
                        "\u{03b1} = \u{03b2} = \u{03b3} \u{2260} 90\u{00b0}".into(),
                        "Tourmaline".into(),
                    ],
                ],
                description: "The seven crystal systems in crystallography.".into(),
            }),
            "khanij",
            vec![
                "crystal".into(),
                "crystallography".into(),
                "minerals".into(),
                "structure".into(),
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
        let p = KhanijProvider;
        assert_eq!(p.source_name(), "khanij");
        assert_eq!(p.domain(), Domain::EarthScience);
    }

    #[test]
    fn entries_not_empty() {
        let entries = KhanijProvider.entries();
        assert!(entries.len() >= 3);
    }

    #[test]
    fn minerals_table_has_rows() {
        let entries = KhanijProvider.entries();
        let table = entries.iter().find(|e| e.id == "common_minerals").unwrap();
        if let EntryKind::Table(t) = &table.kind {
            assert_eq!(t.rows.len(), 15);
            assert_eq!(t.columns.len(), 6);
        } else {
            panic!("expected Table");
        }
    }

    #[test]
    fn mohs_scale_complete() {
        let entries = KhanijProvider.entries();
        let scale = entries
            .iter()
            .find(|e| e.id == "mohs_hardness_scale")
            .unwrap();
        if let EntryKind::Table(t) = &scale.kind {
            assert_eq!(t.rows.len(), 10);
        } else {
            panic!("expected Table");
        }
    }
}
