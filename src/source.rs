//! External source management — download, verify, update.

use crate::domain::Domain;
use serde::{Deserialize, Serialize};

/// An external knowledge source (ZIM file, PDF collection, dataset).
#[derive(Debug, Clone, Serialize, Deserialize)]
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
        let s = Source {
            id: "wikipedia_en".into(),
            name: "Wikipedia (English)".into(),
            domain: Domain::Encyclopedia,
            kind: SourceKind::Zim,
            url: "https://download.kiwix.org/zim/wikipedia/".into(),
            size_mb: 50_000,
            checksum: None,
            enabled: true,
            notes: "Full English Wikipedia without images.".into(),
        };
        let json = serde_json::to_string(&s).unwrap();
        let decoded: Source = serde_json::from_str(&json).unwrap();
        assert_eq!(s.id, decoded.id);
        assert_eq!(s.size_mb, decoded.size_mb);
    }
}
