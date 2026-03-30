//! Content pipeline — load sources and profiles from TOML files.
//!
//! Source definitions live in `content/sources/*.toml` and profile
//! definitions in `content/profiles/*.toml`. This module reads them
//! into the existing [`Source`] and [`Profile`] types, and can index
//! source metadata into the [`Registry`] for unified search.

use crate::entry::{Entry, EntryKind, Fact};
use crate::error::Result;
use crate::profile::Profile;
use crate::registry::Registry;
use crate::source::Source;
use std::path::Path;

/// Load a single source definition from a TOML file.
pub fn load_source(path: &Path) -> Result<Source> {
    let text = std::fs::read_to_string(path)?;
    let source: Source = toml::from_str(&text)?;
    tracing::debug!(id = %source.id, path = %path.display(), "loaded source");
    Ok(source)
}

/// Load all source definitions from a directory of TOML files.
pub fn load_sources(dir: &Path) -> Result<Vec<Source>> {
    let mut sources = Vec::new();
    let entries = std::fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "toml") {
            sources.push(load_source(&path)?);
        }
    }
    sources.sort_by(|a, b| a.id.cmp(&b.id));
    tracing::info!(count = sources.len(), dir = %dir.display(), "loaded sources");
    Ok(sources)
}

/// Load a single profile definition from a TOML file.
pub fn load_profile(path: &Path) -> Result<Profile> {
    let text = std::fs::read_to_string(path)?;
    let profile: Profile = toml::from_str(&text)?;
    tracing::debug!(id = %profile.id, path = %path.display(), "loaded profile");
    Ok(profile)
}

/// Load all profile definitions from a directory of TOML files.
pub fn load_profiles(dir: &Path) -> Result<Vec<Profile>> {
    let mut profiles = Vec::new();
    let entries = std::fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "toml") {
            profiles.push(load_profile(&path)?);
        }
    }
    profiles.sort_by(|a, b| a.id.cmp(&b.id));
    tracing::info!(count = profiles.len(), dir = %dir.display(), "loaded profiles");
    Ok(profiles)
}

/// Index source metadata into the registry as searchable entries.
///
/// Each source becomes a `Fact` entry so users can search for available
/// content alongside AGNOS knowledge.
pub fn index_sources(registry: &mut Registry, sources: &[Source]) {
    for source in sources {
        let entry = Entry::new(
            format!("source_{}", source.id),
            &source.name,
            source.domain,
            format!("{} — {} ({} MB)", source.name, source.kind, source.size_mb),
            EntryKind::Fact(Fact {
                statement: format!(
                    "{} is a {} source covering {} ({} MB).",
                    source.name,
                    source.kind,
                    source.domain.display_name(),
                    source.size_mb,
                ),
                explanation: source.notes.clone(),
                verification: source.checksum.as_ref().map(|c| format!("checksum: {c}")),
            }),
            format!("external:{}", source.id),
            vec![
                "source".into(),
                "external".into(),
                source.kind.to_string().to_lowercase(),
                source.domain.display_name().to_lowercase(),
            ],
        );
        registry.register(entry);
    }
    tracing::info!(count = sources.len(), "indexed source metadata");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Domain;
    use crate::source::SourceKind;
    use std::io::Write;

    fn write_toml(dir: &Path, name: &str, content: &str) {
        let path = dir.join(name);
        let mut f = std::fs::File::create(path).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn load_source_from_toml() {
        let dir = tempfile::tempdir().unwrap();
        write_toml(
            dir.path(),
            "test.toml",
            r#"
id = "test_source"
name = "Test Source"
domain = "Medicine"
kind = "Zim"
url = "https://example.com/test.zim"
size_mb = 500
enabled = true
notes = "A test source."
"#,
        );
        let source = load_source(&dir.path().join("test.toml")).unwrap();
        assert_eq!(source.id, "test_source");
        assert_eq!(source.domain, Domain::Medicine);
        assert_eq!(source.kind, SourceKind::Zim);
        assert_eq!(source.size_mb, 500);
    }

    #[test]
    fn load_sources_from_directory() {
        let dir = tempfile::tempdir().unwrap();
        write_toml(
            dir.path(),
            "a.toml",
            r#"
id = "alpha"
name = "Alpha"
domain = "Medicine"
kind = "Pdf"
url = ""
size_mb = 100
enabled = true
notes = ""
"#,
        );
        write_toml(
            dir.path(),
            "b.toml",
            r#"
id = "beta"
name = "Beta"
domain = "Survival"
kind = "Zim"
url = ""
size_mb = 200
enabled = true
notes = ""
"#,
        );
        // Non-TOML files should be ignored
        write_toml(dir.path(), "readme.txt", "not toml");

        let sources = load_sources(dir.path()).unwrap();
        assert_eq!(sources.len(), 2);
        assert_eq!(sources[0].id, "alpha"); // sorted
        assert_eq!(sources[1].id, "beta");
    }

    #[test]
    fn load_profile_from_toml() {
        let dir = tempfile::tempdir().unwrap();
        write_toml(
            dir.path(),
            "test.toml",
            r#"
id = "test_profile"
name = "Test"
description = "A test profile."
budget_mb = 3000
domains = ["Medicine", "Survival"]
include_external = true
"#,
        );
        let profile = load_profile(&dir.path().join("test.toml")).unwrap();
        assert_eq!(profile.id, "test_profile");
        assert_eq!(profile.budget_mb, 3000);
        assert_eq!(profile.domains.len(), 2);
        assert!(profile.domains.contains(&Domain::Medicine));
    }

    #[test]
    fn load_profiles_from_directory() {
        let dir = tempfile::tempdir().unwrap();
        write_toml(
            dir.path(),
            "dev.toml",
            r#"
id = "dev"
name = "Dev"
description = "Dev profile."
budget_mb = 1000
domains = ["Computing"]
include_external = false
"#,
        );
        let profiles = load_profiles(dir.path()).unwrap();
        assert_eq!(profiles.len(), 1);
    }

    #[test]
    fn index_sources_creates_entries() {
        let sources = vec![Source::new(
            "wikimed",
            "WikiMed",
            Domain::Medicine,
            SourceKind::Zim,
            "https://example.com",
            1200,
        )];
        let mut reg = Registry::new();
        index_sources(&mut reg, &sources);
        assert_eq!(reg.len(), 1);
        let entry = reg.get("source_wikimed").unwrap();
        assert_eq!(entry.domain, Domain::Medicine);
        assert!(entry.has_tag("source"));
        assert!(entry.has_tag("external"));
    }

    #[test]
    fn load_real_sources() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("content/sources");
        if dir.exists() {
            let sources = load_sources(&dir).unwrap();
            assert!(!sources.is_empty(), "should have source definitions");
            for s in &sources {
                assert!(!s.id.is_empty());
                assert!(!s.name.is_empty());
                assert!(s.size_mb > 0);
            }
        }
    }

    #[test]
    fn load_real_profiles() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("content/profiles");
        if dir.exists() {
            let profiles = load_profiles(&dir).unwrap();
            assert!(!profiles.is_empty(), "should have profile definitions");
            for p in &profiles {
                assert!(!p.id.is_empty());
                assert!(p.budget_mb > 0);
                assert!(!p.domains.is_empty());
            }
        }
    }
}
