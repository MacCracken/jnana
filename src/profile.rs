//! Storage profiles — presets for different use cases.
//!
//! Each profile specifies which domains and sources to include,
//! with a target storage budget.

use crate::domain::Domain;
use serde::{Deserialize, Serialize};

/// A storage profile — a curated selection of knowledge for a use case.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Profile {
    /// Profile identifier (e.g. "survival", "developer").
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Description of the use case.
    pub description: String,
    /// Target storage budget in megabytes.
    pub budget_mb: u64,
    /// Domains to include.
    pub domains: Vec<Domain>,
    /// Whether to include external sources (Wikipedia, etc.).
    pub include_external: bool,
}

impl Profile {
    /// Survival kit — essential survival, medical, and practical knowledge.
    #[must_use]
    pub fn survival() -> Self {
        Self {
            id: "survival".into(),
            name: "Survival Kit".into(),
            description: "Essential survival, medical, and practical knowledge. Fits on a phone."
                .into(),
            budget_mb: 2_000,
            domains: vec![
                Domain::Medicine,
                Domain::Survival,
                Domain::Agriculture,
                Domain::Construction,
                Domain::Repair,
                Domain::Communication,
            ],
            include_external: true,
        }
    }

    /// Homesteader — self-sufficiency knowledge.
    #[must_use]
    pub fn homesteader() -> Self {
        Self {
            id: "homesteader".into(),
            name: "Homesteader".into(),
            description:
                "Self-sufficiency: farming, building, repair, medicine, plus science fundamentals."
                    .into(),
            budget_mb: 5_000,
            domains: vec![
                Domain::Medicine,
                Domain::Survival,
                Domain::Agriculture,
                Domain::Construction,
                Domain::Repair,
                Domain::Communication,
                Domain::Chemistry,
                Domain::Biology,
                Domain::EarthScience,
                Domain::Mathematics,
            ],
            include_external: true,
        }
    }

    /// Developer — programming, systems, math.
    #[must_use]
    pub fn developer() -> Self {
        Self {
            id: "developer".into(),
            name: "Developer".into(),
            description: "Programming references, algorithms, systems knowledge, math.".into(),
            budget_mb: 3_000,
            domains: vec![
                Domain::Computing,
                Domain::Mathematics,
                Domain::Statistics,
                Domain::Physics,
            ],
            include_external: false,
        }
    }

    /// Educator — broad coverage for teaching.
    #[must_use]
    pub fn educator() -> Self {
        Self {
            id: "educator".into(),
            name: "Educator".into(),
            description: "Broad knowledge base for teaching and learning across all domains."
                .into(),
            budget_mb: 8_000,
            domains: Domain::all().to_vec(),
            include_external: true,
        }
    }

    /// Full — everything.
    #[must_use]
    pub fn full() -> Self {
        Self {
            id: "full".into(),
            name: "Complete Knowledge Base".into(),
            description: "All domains, all sources. The full foundation of knowing.".into(),
            budget_mb: 10_000,
            domains: Domain::all().to_vec(),
            include_external: true,
        }
    }

    /// All built-in profiles.
    #[must_use]
    pub fn all_profiles() -> Vec<Self> {
        vec![
            Self::survival(),
            Self::homesteader(),
            Self::developer(),
            Self::educator(),
            Self::full(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_survival() {
        let p = Profile::survival();
        assert_eq!(p.id, "survival");
        assert!(p.budget_mb <= 2_000);
        assert!(p.domains.contains(&Domain::Medicine));
    }

    #[test]
    fn profile_developer() {
        let p = Profile::developer();
        assert!(p.domains.contains(&Domain::Computing));
        assert!(!p.include_external);
    }

    #[test]
    fn profile_full_has_all_domains() {
        let p = Profile::full();
        assert_eq!(p.domains.len(), Domain::all().len());
    }

    #[test]
    fn all_profiles() {
        let profiles = Profile::all_profiles();
        assert_eq!(profiles.len(), 5);
    }

    #[test]
    fn profile_serde_roundtrip() {
        let p = Profile::survival();
        let json = serde_json::to_string(&p).unwrap();
        let decoded: Profile = serde_json::from_str(&json).unwrap();
        assert_eq!(p.id, decoded.id);
        assert_eq!(p.budget_mb, decoded.budget_mb);
    }
}
