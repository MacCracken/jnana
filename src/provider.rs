//! Knowledge provider trait for AGNOS crate integration.
//!
//! A [`KnowledgeProvider`] extracts structured knowledge entries from a
//! source — typically an AGNOS science crate. Jnana ships built-in
//! implementations behind feature gates for each published crate.

use crate::domain::Domain;
use crate::entry::Entry;

/// A source of knowledge entries.
///
/// Implementors produce [`Entry`] values from their domain data.
/// Built-in providers live in [`crate::providers`] behind feature gates.
pub trait KnowledgeProvider {
    /// The crate or source name (e.g. `"kimiya"`, `"tanmatra"`).
    fn source_name(&self) -> &str;

    /// The primary domain this provider covers.
    fn domain(&self) -> Domain;

    /// Extract all knowledge entries from this provider.
    fn entries(&self) -> Vec<Entry>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::{Constant, EntryKind};

    struct TestProvider;

    impl KnowledgeProvider for TestProvider {
        fn source_name(&self) -> &str {
            "test"
        }

        fn domain(&self) -> Domain {
            Domain::Mathematics
        }

        fn entries(&self) -> Vec<Entry> {
            vec![Entry::new(
                "test_pi",
                "Pi",
                Domain::Mathematics,
                "The ratio of circumference to diameter.",
                EntryKind::Constant(Constant {
                    symbol: "\u{03c0}".into(),
                    value: "3.14159265358979".into(),
                    unit: "".into(),
                    numeric: std::f64::consts::PI,
                    uncertainty: None,
                    authority: "exact".into(),
                }),
                "test",
                vec!["circle".into(), "geometry".into()],
            )]
        }
    }

    #[test]
    fn provider_trait_basics() {
        let p = TestProvider;
        assert_eq!(p.source_name(), "test");
        assert_eq!(p.domain(), Domain::Mathematics);
        let entries = p.entries();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id, "test_pi");
    }

    #[test]
    fn provider_is_object_safe() {
        let p: Box<dyn KnowledgeProvider> = Box::new(TestProvider);
        assert_eq!(p.source_name(), "test");
        assert_eq!(p.entries().len(), 1);
    }
}
