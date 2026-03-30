//! Storage budget calculator.
//!
//! Given available disk space and a profile, determines what fits.

use crate::profile::Profile;
use crate::source::Source;
use serde::{Deserialize, Serialize};

/// Storage budget analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageBudget {
    /// Available space in megabytes.
    pub available_mb: u64,
    /// Profile being evaluated.
    pub profile_id: String,
    /// Internal knowledge size (AGNOS crate data) in megabytes.
    pub internal_mb: u64,
    /// External sources that fit within budget.
    pub fits: Vec<String>,
    /// External sources that don't fit.
    pub excluded: Vec<String>,
    /// Total size of selected content in megabytes.
    pub total_mb: u64,
    /// Remaining space in megabytes.
    pub remaining_mb: u64,
}

/// Calculate what fits on the given storage for a profile.
#[must_use]
pub fn calculate(available_mb: u64, profile: &Profile, sources: &[Source]) -> StorageBudget {
    // Internal knowledge is tiny — estimate 100MB for all AGNOS crate data
    let internal_mb = 100;
    let mut remaining = available_mb.saturating_sub(internal_mb);
    let mut fits = Vec::new();
    let mut excluded = Vec::new();
    let mut total = internal_mb;

    // Sort sources by size (smallest first) to maximize count
    let mut sorted: Vec<&Source> = sources
        .iter()
        .filter(|s| s.enabled && profile.domains.contains(&s.domain))
        .collect();
    sorted.sort_by_key(|s| s.size_mb);

    for source in sorted {
        if source.size_mb <= remaining {
            remaining -= source.size_mb;
            total += source.size_mb;
            fits.push(source.name.clone());
        } else {
            excluded.push(source.name.clone());
        }
    }

    StorageBudget {
        available_mb,
        profile_id: profile.id.clone(),
        internal_mb,
        fits,
        excluded,
        total_mb: total,
        remaining_mb: remaining,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Domain;
    use crate::source::SourceKind;

    #[test]
    fn budget_empty() {
        let profile = Profile::survival();
        let budget = calculate(5000, &profile, &[]);
        assert_eq!(budget.internal_mb, 100);
        assert_eq!(budget.total_mb, 100);
        assert!(budget.fits.is_empty());
    }

    #[test]
    fn budget_fits_small_sources() {
        let profile = Profile::survival();
        let sources = vec![Source {
            id: "med".into(),
            name: "Medical Encyclopedia".into(),
            domain: Domain::Medicine,
            kind: SourceKind::Zim,
            url: "".into(),
            size_mb: 1500,
            checksum: None,
            enabled: true,
            notes: "".into(),
        }];
        let budget = calculate(5000, &profile, &sources);
        assert_eq!(budget.fits.len(), 1);
        assert!(budget.excluded.is_empty());
    }

    #[test]
    fn budget_excludes_too_large() {
        let profile = Profile::survival();
        let sources = vec![Source {
            id: "wiki".into(),
            name: "Full Wikipedia".into(),
            domain: Domain::Medicine,
            kind: SourceKind::Zim,
            url: "".into(),
            size_mb: 50_000,
            checksum: None,
            enabled: true,
            notes: "".into(),
        }];
        let budget = calculate(5000, &profile, &sources);
        assert!(budget.fits.is_empty());
        assert_eq!(budget.excluded.len(), 1);
    }
}
