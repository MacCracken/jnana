//! In-memory knowledge registry.

use crate::domain::Domain;
use crate::entry::Entry;
use crate::error::{JnanaError, Result};
use std::collections::HashMap;

/// The knowledge registry — holds all entries in memory.
pub struct Registry {
    entries: HashMap<String, Entry>,
}

impl Registry {
    /// Create an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Register an entry. Overwrites if the ID already exists.
    pub fn register(&mut self, entry: Entry) {
        tracing::debug!(id = %entry.id, domain = %entry.domain, "registered entry");
        self.entries.insert(entry.id.clone(), entry);
    }

    /// Get an entry by ID.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&Entry> {
        self.entries.get(id)
    }

    /// Get an entry by ID, returning an error if not found.
    pub fn get_or_err(&self, id: &str) -> Result<&Entry> {
        self.entries
            .get(id)
            .ok_or_else(|| JnanaError::EntryNotFound(id.into()))
    }

    /// List all entry IDs, sorted.
    #[must_use]
    pub fn list_ids(&self) -> Vec<&str> {
        let mut ids: Vec<&str> = self.entries.keys().map(|s| s.as_str()).collect();
        ids.sort_unstable();
        ids
    }

    /// List all entries, sorted by ID.
    #[must_use]
    pub fn list(&self) -> Vec<&Entry> {
        let mut entries: Vec<&Entry> = self.entries.values().collect();
        entries.sort_by(|a, b| a.id.cmp(&b.id));
        entries
    }

    /// Filter entries by domain.
    #[must_use]
    pub fn by_domain(&self, domain: Domain) -> Vec<&Entry> {
        self.entries
            .values()
            .filter(|e| e.domain == domain)
            .collect()
    }

    /// Number of entries.
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Total estimated size of all entries in bytes.
    #[must_use]
    pub fn total_size(&self) -> usize {
        self.entries.values().map(|e| e.estimated_size()).sum()
    }

    /// Count entries per domain.
    #[must_use]
    pub fn domain_counts(&self) -> HashMap<Domain, usize> {
        let mut counts = HashMap::new();
        for entry in self.entries.values() {
            *counts.entry(entry.domain).or_insert(0) += 1;
        }
        counts
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::{Constant, EntryKind};

    fn make_entry(id: &str, domain: Domain) -> Entry {
        Entry {
            id: id.into(),
            title: id.into(),
            domain,
            summary: format!("Test entry: {id}"),
            kind: EntryKind::Constant(Constant {
                symbol: "x".into(),
                value: "42".into(),
                unit: "".into(),
                numeric: 42.0,
                uncertainty: None,
                authority: "test".into(),
            }),
            source: "test".into(),
            tags: vec![],
        }
    }

    #[test]
    fn registry_empty() {
        let reg = Registry::new();
        assert!(reg.is_empty());
        assert_eq!(reg.len(), 0);
    }

    #[test]
    fn registry_register_and_get() {
        let mut reg = Registry::new();
        reg.register(make_entry("pi", Domain::Mathematics));
        assert_eq!(reg.len(), 1);
        assert!(reg.get("pi").is_some());
        assert!(reg.get("missing").is_none());
    }

    #[test]
    fn registry_by_domain() {
        let mut reg = Registry::new();
        reg.register(make_entry("pi", Domain::Mathematics));
        reg.register(make_entry("c", Domain::Physics));
        reg.register(make_entry("e", Domain::Mathematics));
        let math = reg.by_domain(Domain::Mathematics);
        assert_eq!(math.len(), 2);
    }

    #[test]
    fn registry_domain_counts() {
        let mut reg = Registry::new();
        reg.register(make_entry("a", Domain::Mathematics));
        reg.register(make_entry("b", Domain::Mathematics));
        reg.register(make_entry("c", Domain::Physics));
        let counts = reg.domain_counts();
        assert_eq!(counts[&Domain::Mathematics], 2);
        assert_eq!(counts[&Domain::Physics], 1);
    }

    #[test]
    fn registry_total_size() {
        let mut reg = Registry::new();
        reg.register(make_entry("a", Domain::Mathematics));
        assert!(reg.total_size() > 0);
    }
}
