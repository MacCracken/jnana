//! Integration tests — cross-module workflows.

use jnana::entry::{Constant, EntryKind, Procedure};
use jnana::search::search;
use jnana::source::{Source, SourceKind};
use jnana::storage::calculate;
use jnana::{Domain, Entry, Profile, Registry, SearchQuery};

fn build_registry() -> Registry {
    let mut reg = Registry::new();

    reg.register(Entry::new(
        "speed_of_light",
        "Speed of Light in Vacuum",
        Domain::Physics,
        "The speed of light in vacuum.",
        EntryKind::Constant(Constant {
            symbol: "c".into(),
            value: "299792458".into(),
            unit: "m/s".into(),
            numeric: 299_792_458.0,
            uncertainty: None,
            authority: "CODATA 2022 (exact)".into(),
        }),
        "prakash",
        vec!["light".into(), "fundamental".into()],
    ));

    reg.register(Entry::new(
        "water_purification",
        "Emergency Water Purification",
        Domain::Survival,
        "Methods to make water safe for drinking.",
        EntryKind::Procedure(Procedure {
            when: "Water source is untreated.".into(),
            steps: vec![
                "Collect water.".into(),
                "Filter through cloth.".into(),
                "Boil for 1 minute.".into(),
            ],
            warnings: vec!["Does not remove chemical contaminants.".into()],
            requires: vec!["Heat source".into()],
        }),
        "FM 21-76",
        vec!["water".into(), "survival".into()],
    ));

    reg
}

#[test]
fn end_to_end_search_and_budget() {
    let reg = build_registry();

    // Search finds entries
    let results = search(&reg, &SearchQuery::text("light"));
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "speed_of_light");

    // Domain filter works
    let mut q = SearchQuery::text("water");
    q.domain = Some(Domain::Survival);
    let results = search(&reg, &q);
    assert_eq!(results.len(), 1);

    // Budget calculation with sources
    let profile = Profile::survival();
    let sources = vec![
        Source::new(
            "wikimed",
            "WikiMed",
            Domain::Medicine,
            SourceKind::Zim,
            "",
            1200,
        ),
        Source::new(
            "maps",
            "Offline Maps",
            Domain::Geography,
            SourceKind::Osm,
            "",
            3000,
        ),
    ];
    let budget = calculate(5000, &profile, &sources);
    // WikiMed fits (Medicine is in survival profile), maps don't (Geography isn't)
    assert_eq!(budget.fits.len(), 1);
    assert_eq!(budget.fits[0], "WikiMed");
}

#[test]
fn all_profiles_have_valid_budgets() {
    for profile in Profile::all_profiles() {
        assert!(profile.budget_mb > 0);
        assert!(!profile.domains.is_empty());
        assert!(!profile.id.is_empty());
        assert!(!profile.name.is_empty());
    }
}

#[test]
fn registry_roundtrip_via_json() {
    let reg = build_registry();
    let entries: Vec<&Entry> = reg.list();

    // Serialize all entries
    let json = serde_json::to_string(&entries).unwrap();

    // Deserialize back
    let decoded: Vec<Entry> = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.len(), entries.len());

    // Re-register and verify
    let mut reg2 = Registry::new();
    for entry in decoded {
        reg2.register(entry);
    }
    assert_eq!(reg2.len(), reg.len());
    assert!(reg2.get("speed_of_light").is_some());
}
