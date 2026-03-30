//! Knowledge domains — the top-level organization of human understanding.

use serde::{Deserialize, Serialize};

/// A domain of human knowledge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Domain {
    // ── Sciences (from AGNOS crates) ───────────────────────────────
    /// Mathematics — algebra, calculus, geometry, number theory (hisab)
    Mathematics,
    /// Physics — mechanics, optics, EM, thermodynamics (impetus, prakash, bijli, ushma)
    Physics,
    /// Chemistry — elements, reactions, kinetics (kimiya)
    Chemistry,
    /// Biology — microbiology, ethology, botany, physiology (jivanu, jantu, vanaspati, sharira)
    Biology,
    /// Earth science — geology, weather, atmospheric (khanij, badal)
    EarthScience,
    /// Astronomy — orbital mechanics, stellar physics (falak, tara, jyotish)
    Astronomy,
    /// Statistics — probability, distributions, inference (pramana)
    Statistics,
    /// Computing — programming, algorithms, systems (vidya)
    Computing,
    /// Psychology — cognition, perception, learning (bodh)
    Psychology,
    /// Sociology — networks, groups, population dynamics (sangha)
    Sociology,

    // ── Applied knowledge ──────────────────────────────────────────
    /// Medicine — diagnosis, treatment, first aid, field medicine
    Medicine,
    /// Survival — shelter, water, food, navigation, emergency
    Survival,
    /// Agriculture — farming, gardening, permaculture, aquaponics
    Agriculture,
    /// Construction — building, plumbing, electrical, solar, earthworks
    Construction,
    /// Repair — electronics, appliances, vehicles, maintenance
    Repair,
    /// Communication — radio, signals, emergency comms
    Communication,

    // ── Reference ──────────────────────────────────────────────────
    /// Encyclopedia — general knowledge (Wikipedia, etc.)
    Encyclopedia,
    /// Language — dictionaries, grammar, translation
    Language,
    /// Literature — books, philosophy, history
    Literature,
    /// Geography — maps, travel, cultures
    Geography,
}

impl Domain {
    /// Human-readable display name.
    #[must_use]
    pub const fn display_name(&self) -> &'static str {
        match self {
            Self::Mathematics => "Mathematics",
            Self::Physics => "Physics",
            Self::Chemistry => "Chemistry",
            Self::Biology => "Biology",
            Self::EarthScience => "Earth Science",
            Self::Astronomy => "Astronomy",
            Self::Statistics => "Statistics",
            Self::Computing => "Computing",
            Self::Psychology => "Psychology",
            Self::Sociology => "Sociology",
            Self::Medicine => "Medicine",
            Self::Survival => "Survival",
            Self::Agriculture => "Agriculture",
            Self::Construction => "Construction",
            Self::Repair => "Repair",
            Self::Communication => "Communication",
            Self::Encyclopedia => "Encyclopedia",
            Self::Language => "Language",
            Self::Literature => "Literature",
            Self::Geography => "Geography",
        }
    }

    /// Short description of the domain.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Mathematics => "Algebra, calculus, geometry, number theory",
            Self::Physics => "Mechanics, optics, electromagnetism, thermodynamics",
            Self::Chemistry => "Elements, reactions, kinetics, materials",
            Self::Biology => "Microbiology, ethology, botany, physiology",
            Self::EarthScience => "Geology, mineralogy, weather, atmospheric science",
            Self::Astronomy => "Orbital mechanics, stellar physics, celestial computation",
            Self::Statistics => "Probability, distributions, inference, data analysis",
            Self::Computing => "Programming, algorithms, systems, best practices",
            Self::Psychology => "Cognition, perception, learning, decision-making",
            Self::Sociology => "Social networks, game theory, group dynamics",
            Self::Medicine => "Diagnosis, treatment, first aid, field medicine",
            Self::Survival => "Shelter, water, food, navigation, emergency response",
            Self::Agriculture => "Farming, gardening, permaculture, aquaponics",
            Self::Construction => "Building, plumbing, electrical, solar, earthworks",
            Self::Repair => "Electronics, appliances, vehicles, maintenance",
            Self::Communication => "Radio, signals, emergency communications",
            Self::Encyclopedia => "General knowledge and reference",
            Self::Language => "Dictionaries, grammar, translation",
            Self::Literature => "Books, philosophy, history",
            Self::Geography => "Maps, travel, cultures, world facts",
        }
    }

    /// AGNOS crate that provides internal knowledge for this domain (if any).
    #[must_use]
    pub const fn agnos_crate(&self) -> Option<&'static str> {
        match self {
            Self::Mathematics => Some("hisab"),
            Self::Physics => Some("impetus, prakash, bijli, ushma"),
            Self::Chemistry => Some("kimiya"),
            Self::Biology => Some("jivanu, jantu, vanaspati, sharira"),
            Self::EarthScience => Some("khanij, badal"),
            Self::Astronomy => Some("falak, tara, jyotish"),
            Self::Statistics => Some("pramana"),
            Self::Computing => Some("vidya"),
            Self::Psychology => Some("bodh"),
            Self::Sociology => Some("sangha"),
            _ => None,
        }
    }

    /// All domains.
    #[must_use]
    pub const fn all() -> &'static [Domain] {
        &[
            Self::Mathematics,
            Self::Physics,
            Self::Chemistry,
            Self::Biology,
            Self::EarthScience,
            Self::Astronomy,
            Self::Statistics,
            Self::Computing,
            Self::Psychology,
            Self::Sociology,
            Self::Medicine,
            Self::Survival,
            Self::Agriculture,
            Self::Construction,
            Self::Repair,
            Self::Communication,
            Self::Encyclopedia,
            Self::Language,
            Self::Literature,
            Self::Geography,
        ]
    }
}

impl std::fmt::Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_domains() {
        assert_eq!(Domain::all().len(), 20);
    }

    #[test]
    fn display() {
        assert_eq!(Domain::Mathematics.to_string(), "Mathematics");
        assert_eq!(Domain::EarthScience.to_string(), "Earth Science");
    }

    #[test]
    fn agnos_crates() {
        assert_eq!(Domain::Mathematics.agnos_crate(), Some("hisab"));
        assert_eq!(Domain::Chemistry.agnos_crate(), Some("kimiya"));
        assert!(Domain::Medicine.agnos_crate().is_none());
    }

    #[test]
    fn serde_roundtrip() {
        let d = Domain::Physics;
        let json = serde_json::to_string(&d).unwrap();
        let decoded: Domain = serde_json::from_str(&json).unwrap();
        assert_eq!(d, decoded);
    }
}
