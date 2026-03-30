//! External source management — download, verify, update.

use crate::domain::Domain;
use serde::{Deserialize, Serialize};

/// An external knowledge source (ZIM file, PDF collection, dataset).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Source {
    /// Unique identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Domain this source covers.
    pub domain: Domain,
    /// Source type.
    pub kind: SourceKind,
    /// Download URL.
    pub url: String,
    /// Expected size in megabytes.
    pub size_mb: u64,
    /// SHA-256 checksum (if known).
    pub checksum: Option<String>,
    /// Whether this source is enabled by default.
    pub enabled: bool,
    /// Human-readable notes.
    pub notes: String,
}

impl Source {
    /// Create a new external source.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        domain: Domain,
        kind: SourceKind,
        url: impl Into<String>,
        size_mb: u64,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            domain,
            kind,
            url: url.into(),
            size_mb,
            checksum: None,
            enabled: true,
            notes: String::new(),
        }
    }

    /// Set the SHA-256 checksum.
    #[must_use]
    pub fn with_checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum = Some(checksum.into());
        self
    }

    /// Set enabled/disabled state.
    #[must_use]
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set human-readable notes.
    #[must_use]
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = notes.into();
        self
    }
}

/// Type of external source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SourceKind {
    /// Kiwix ZIM file (full-text searchable).
    Zim,
    /// PDF document or collection.
    Pdf,
    /// OpenStreetMap data.
    Osm,
    /// Generic file or dataset.
    File,
}

impl std::fmt::Display for SourceKind {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zim => f.write_str("ZIM"),
            Self::Pdf => f.write_str("PDF"),
            Self::Osm => f.write_str("OSM"),
            Self::File => f.write_str("File"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_kind_display() {
        assert_eq!(SourceKind::Zim.to_string(), "ZIM");
        assert_eq!(SourceKind::Pdf.to_string(), "PDF");
    }

    #[test]
    fn source_serde_roundtrip() {
        let s = Source::new(
            "wikipedia_en",
            "Wikipedia (English)",
            Domain::Encyclopedia,
            SourceKind::Zim,
            "https://download.kiwix.org/zim/wikipedia/",
            50_000,
        )
        .with_notes("Full English Wikipedia without images.");
        let json = serde_json::to_string(&s).unwrap();
        let decoded: Source = serde_json::from_str(&json).unwrap();
        assert_eq!(s.id, decoded.id);
        assert_eq!(s.size_mb, decoded.size_mb);
    }

    #[test]
    fn source_kind_display_all() {
        assert_eq!(SourceKind::Zim.to_string(), "ZIM");
        assert_eq!(SourceKind::Pdf.to_string(), "PDF");
        assert_eq!(SourceKind::Osm.to_string(), "OSM");
        assert_eq!(SourceKind::File.to_string(), "File");
    }

    #[test]
    fn source_builder() {
        let s = Source::new(
            "test",
            "Test",
            Domain::Medicine,
            SourceKind::Pdf,
            "url",
            100,
        )
        .with_checksum("abc123")
        .with_enabled(false)
        .with_notes("test notes");
        assert_eq!(s.checksum.as_deref(), Some("abc123"));
        assert!(!s.enabled);
        assert_eq!(s.notes, "test notes");
    }

    #[test]
    fn source_defaults() {
        let s = Source::new("test", "Test", Domain::Medicine, SourceKind::Zim, "", 0);
        assert!(s.checksum.is_none());
        assert!(s.enabled);
        assert!(s.notes.is_empty());
    }

    #[test]
    fn source_kind_serde_roundtrip() {
        for kind in [
            SourceKind::Zim,
            SourceKind::Pdf,
            SourceKind::Osm,
            SourceKind::File,
        ] {
            let json = serde_json::to_string(&kind).unwrap();
            let decoded: SourceKind = serde_json::from_str(&json).unwrap();
            assert_eq!(kind, decoded);
        }
    }

    #[test]
    fn source_with_all_fields() {
        let s = Source::new(
            "full",
            "Full Source",
            Domain::Encyclopedia,
            SourceKind::Zim,
            "https://example.com",
            5000,
        )
        .with_checksum("sha256:abc")
        .with_enabled(true)
        .with_notes("Complete source.");
        let json = serde_json::to_string(&s).unwrap();
        let decoded: Source = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.id, "full");
        assert_eq!(decoded.checksum.as_deref(), Some("sha256:abc"));
        assert_eq!(decoded.notes, "Complete source.");
        assert!(decoded.enabled);
        assert_eq!(decoded.domain, Domain::Encyclopedia);
        assert_eq!(decoded.kind, SourceKind::Zim);
        assert_eq!(decoded.url, "https://example.com");
    }

    #[test]
    fn source_disabled() {
        let s = Source::new("d", "D", Domain::Medicine, SourceKind::Pdf, "", 0).with_enabled(false);
        assert!(!s.enabled);
        let json = serde_json::to_string(&s).unwrap();
        let decoded: Source = serde_json::from_str(&json).unwrap();
        assert!(!decoded.enabled);
    }
}
