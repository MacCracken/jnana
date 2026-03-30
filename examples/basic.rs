use jnana::entry::{Constant, EntryKind, Procedure};
use jnana::search::search;
use jnana::{Domain, Entry, Profile, Registry, SearchQuery};

fn main() {
    let mut registry = Registry::new();

    // Register a physical constant
    registry.register(Entry::new(
        "speed_of_light",
        "Speed of Light in Vacuum",
        Domain::Physics,
        "The speed of light in vacuum — a fundamental physical constant.",
        EntryKind::Constant(Constant {
            symbol: "c".into(),
            value: "299792458".into(),
            unit: "m/s".into(),
            numeric: 299_792_458.0,
            uncertainty: None,
            authority: "CODATA 2022 (exact)".into(),
        }),
        "prakash, tanmatra",
        vec!["light".into(), "speed".into(), "fundamental".into()],
    ));

    // Register a survival procedure
    registry.register(Entry::new(
        "water_purification",
        "Emergency Water Purification",
        Domain::Survival,
        "Methods to make water safe for drinking in the field.",
        EntryKind::Procedure(Procedure {
            when: "Water source is untreated or of unknown safety.".into(),
            steps: vec![
                "Collect water from the cleanest available source.".into(),
                "Filter through cloth to remove sediment.".into(),
                "Boil at a rolling boil for 1 minute (3 minutes above 2000m/6500ft).".into(),
                "Let cool. Store in clean container.".into(),
            ],
            warnings: vec![
                "Boiling does not remove chemical contaminants.".into(),
                "If unable to boil, use water purification tablets or 2 drops of bleach per liter."
                    .into(),
            ],
            requires: vec!["Heat source".into(), "Container".into()],
        }),
        "FM 21-76, Where There Is No Doctor",
        vec!["water".into(), "survival".into(), "purification".into()],
    ));

    // Search
    println!("=== Search: 'light' ===");
    for r in search(&registry, &SearchQuery::text("light")) {
        println!("  {} ({}) — {}", r.title, r.domain, r.summary);
    }

    println!("\n=== Search: 'water' ===");
    for r in search(&registry, &SearchQuery::text("water")) {
        println!("  {} ({}) — {}", r.title, r.domain, r.summary);
    }

    // Profiles
    println!("\n=== Profiles ===");
    for p in Profile::all_profiles() {
        println!(
            "  {} — {}MB budget, {} domains",
            p.name,
            p.budget_mb,
            p.domains.len()
        );
    }

    // Stats
    println!("\n=== Registry ===");
    println!("  Entries: {}", registry.len());
    println!("  Size: ~{} bytes", registry.total_size());
    for (domain, count) in registry.domain_counts() {
        println!("  {domain}: {count} entries");
    }
}
