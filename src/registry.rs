//! In-memory knowledge registry.

use crate::domain::Domain;
use crate::entry::Entry;
use crate::error::{JnanaError, Result};
use crate::provider::KnowledgeProvider;
use std::collections::HashMap;

/// The knowledge registry — holds all entries in memory.
#[non_exhaustive]
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

    /// Get a mutable reference to an entry by ID.
    #[must_use]
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Entry> {
        self.entries.get_mut(id)
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

    /// Register all entries from a knowledge provider.
    pub fn register_provider(&mut self, provider: &dyn KnowledgeProvider) {
        let entries = provider.entries();
        tracing::info!(
            source = provider.source_name(),
            count = entries.len(),
            "registering knowledge provider"
        );
        for entry in entries {
            self.register(entry);
        }
    }

    /// Create a registry pre-loaded with all enabled AGNOS providers.
    ///
    /// Loads every provider enabled by feature flags and resolves
    /// cross-references between entries.
    #[must_use]
    pub fn with_agnos_providers() -> Self {
        let mut reg = Self::new();
        for provider in crate::providers::all_providers() {
            reg.register_provider(&*provider);
        }
        crate::linker::resolve_links(&mut reg);
        reg
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
            related: vec![],
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

    #[test]
    fn registry_overwrite() {
        let mut reg = Registry::new();
        reg.register(make_entry("a", Domain::Mathematics));
        reg.register(make_entry("a", Domain::Physics));
        assert_eq!(reg.len(), 1);
        assert_eq!(reg.get("a").unwrap().domain, Domain::Physics);
    }

    #[test]
    fn registry_get_or_err_found() {
        let mut reg = Registry::new();
        reg.register(make_entry("pi", Domain::Mathematics));
        assert!(reg.get_or_err("pi").is_ok());
    }

    #[test]
    fn registry_get_or_err_missing() {
        let reg = Registry::new();
        assert!(reg.get_or_err("missing").is_err());
    }

    #[test]
    fn registry_list_ids_sorted() {
        let mut reg = Registry::new();
        reg.register(make_entry("c", Domain::Physics));
        reg.register(make_entry("a", Domain::Mathematics));
        reg.register(make_entry("b", Domain::Chemistry));
        let ids = reg.list_ids();
        assert_eq!(ids, vec!["a", "b", "c"]);
    }

    #[test]
    fn registry_default() {
        let reg = Registry::default();
        assert!(reg.is_empty());
    }
}
