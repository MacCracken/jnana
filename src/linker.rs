//! Cross-crate knowledge linking.
//!
//! After all providers have registered their entries, [`resolve_links`]
//! infers relationships between entries and populates the `related` field.

use crate::registry::Registry;
use std::collections::HashMap;

/// Minimum number of shared tags for two entries to be linked.
const MIN_SHARED_TAGS: usize = 2;

/// Resolve cross-references between entries in the registry.
///
/// Links are inferred from shared tags: entries sharing at least
/// [`MIN_SHARED_TAGS`] tags are connected bidirectionally.
pub fn resolve_links(registry: &mut Registry) {
    // Build tag index: lowercased tag -> list of entry IDs
    let mut tag_index: HashMap<String, Vec<String>> = HashMap::new();
    for entry in registry.list() {
        for tag in &entry.tags {
            tag_index
                .entry(tag.to_lowercase())
                .or_default()
                .push(entry.id.clone());
        }
    }

    // Count shared tags between each pair of entries
    let mut shared: HashMap<(String, String), usize> = HashMap::new();
    for ids in tag_index.values() {
        for (i, a) in ids.iter().enumerate() {
            for b in &ids[i + 1..] {
                if a == b {
                    continue;
                }
                let key = if a < b {
                    (a.clone(), b.clone())
                } else {
                    (b.clone(), a.clone())
                };
                *shared.entry(key).or_insert(0) += 1;
            }
        }
    }

    // Collect links that meet the threshold
    let mut links: HashMap<String, Vec<String>> = HashMap::new();
    for ((a, b), count) in &shared {
        if *count >= MIN_SHARED_TAGS {
            links.entry(a.clone()).or_default().push(b.clone());
            links.entry(b.clone()).or_default().push(a.clone());
        }
    }

    // Apply links to entries
    let mut link_count = 0;
    for (id, related) in &links {
        if let Some(entry) = registry.get_mut(id) {
            for r in related {
                if !entry.related.contains(r) {
                    entry.related.push(r.clone());
                    link_count += 1;
                }
            }
            entry.related.sort();
        }
    }

    tracing::info!(links = link_count, "resolved cross-references");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Domain;
    use crate::entry::{Constant, Entry, EntryKind};

    fn make_entry(id: &str, domain: Domain, tags: &[&str]) -> Entry {
        Entry::new(
            id,
            id,
            domain,
            "test",
            EntryKind::Constant(Constant {
                symbol: "x".into(),
                value: "1".into(),
                unit: "".into(),
                numeric: 1.0,
                uncertainty: None,
                authority: "test".into(),
            }),
            "test",
            tags.iter().map(|t| (*t).into()).collect(),
        )
    }

    #[test]
    fn links_entries_with_shared_tags() {
        let mut reg = Registry::new();
        reg.register(make_entry("a", Domain::Physics, &["light", "fundamental", "wave"]));
        reg.register(make_entry("b", Domain::Physics, &["light", "fundamental", "particle"]));
        reg.register(make_entry("c", Domain::Chemistry, &["acid", "reaction"]));

        resolve_links(&mut reg);

        // a and b share "light" + "fundamental" (2 tags) -> linked
        let a = reg.get("a").unwrap();
        assert!(a.related.contains(&"b".to_string()));
        let b = reg.get("b").unwrap();
        assert!(b.related.contains(&"a".to_string()));

        // c shares no tags with a or b -> not linked
        let c = reg.get("c").unwrap();
        assert!(c.related.is_empty());
    }

    #[test]
    fn no_links_below_threshold() {
        let mut reg = Registry::new();
        reg.register(make_entry("a", Domain::Physics, &["light"]));
        reg.register(make_entry("b", Domain::Physics, &["light"]));

        resolve_links(&mut reg);

        // Only 1 shared tag, below threshold of 2
        let a = reg.get("a").unwrap();
        assert!(a.related.is_empty());
    }

    #[test]
    fn empty_registry_is_fine() {
        let mut reg = Registry::new();
        resolve_links(&mut reg);
        assert!(reg.is_empty());
    }
}
