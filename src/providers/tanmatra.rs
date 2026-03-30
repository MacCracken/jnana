//! Physics constants provider — extracts from the `tanmatra` crate.

use crate::domain::Domain;
use crate::entry::{Constant, Entry, EntryKind};
use crate::provider::KnowledgeProvider;

/// Provides fundamental physics constants from the tanmatra crate.
pub struct TanmatraProvider;

impl KnowledgeProvider for TanmatraProvider {
    fn source_name(&self) -> &str {
        "tanmatra"
    }

    fn domain(&self) -> Domain {
        Domain::Physics
    }

    fn entries(&self) -> Vec<Entry> {
        vec![
            constant_entry(
                "speed_of_light",
                "Speed of Light in Vacuum",
                "c",
                &tanmatra::constants::C.to_string(),
                "m/s",
                tanmatra::constants::C,
                None,
                "CODATA 2022 (exact)",
                &["light", "relativity", "fundamental", "exact"],
            ),
            constant_entry(
                "fine_structure_constant",
                "Fine-Structure Constant",
                "\u{03b1}",
                &format!("{:.12}", tanmatra::constants::FINE_STRUCTURE),
                "",
                tanmatra::constants::FINE_STRUCTURE,
                Some("\u{00b1}0.000 000 011"),
                "CODATA 2022",
                &["QED", "electromagnetic", "fundamental", "dimensionless"],
            ),
            constant_entry(
                "electron_mass",
                "Electron Rest Mass",
                "m_e",
                &tanmatra::constants::ELECTRON_MASS_MEV.to_string(),
                "MeV/c\u{00b2}",
                tanmatra::constants::ELECTRON_MASS_MEV,
                Some("\u{00b1}0.000 000 015"),
                "CODATA 2022",
                &["electron", "lepton", "particle", "mass"],
            ),
            constant_entry(
                "proton_mass",
                "Proton Rest Mass",
                "m_p",
                &tanmatra::constants::PROTON_MASS_MEV.to_string(),
                "MeV/c\u{00b2}",
                tanmatra::constants::PROTON_MASS_MEV,
                Some("\u{00b1}0.000 000 28"),
                "CODATA 2022",
                &["proton", "baryon", "particle", "mass"],
            ),
            constant_entry(
                "neutron_mass",
                "Neutron Rest Mass",
                "m_n",
                &tanmatra::constants::NEUTRON_MASS_MEV.to_string(),
                "MeV/c\u{00b2}",
                tanmatra::constants::NEUTRON_MASS_MEV,
                Some("\u{00b1}0.000 000 58"),
                "CODATA 2022",
                &["neutron", "baryon", "particle", "mass"],
            ),
            constant_entry(
                "planck_constant",
                "Planck Constant",
                "h",
                &tanmatra::constants::H_EV_S.to_string(),
                "eV\u{00b7}s",
                tanmatra::constants::H_EV_S,
                None,
                "CODATA 2022 (exact)",
                &["planck", "quantum", "fundamental", "exact"],
            ),
            constant_entry(
                "reduced_planck_constant",
                "Reduced Planck Constant",
                "\u{0127}",
                &tanmatra::constants::HBAR_EV_S.to_string(),
                "eV\u{00b7}s",
                tanmatra::constants::HBAR_EV_S,
                None,
                "CODATA 2022 (exact)",
                &["planck", "quantum", "fundamental"],
            ),
            constant_entry(
                "elementary_charge",
                "Elementary Charge",
                "e",
                &tanmatra::constants::ELEMENTARY_CHARGE.to_string(),
                "C",
                tanmatra::constants::ELEMENTARY_CHARGE,
                None,
                "CODATA 2022 (exact)",
                &["charge", "electron", "fundamental", "exact"],
            ),
            constant_entry(
                "boltzmann_constant",
                "Boltzmann Constant",
                "k_B",
                &tanmatra::constants::BOLTZMANN_EV.to_string(),
                "eV/K",
                tanmatra::constants::BOLTZMANN_EV,
                None,
                "CODATA 2022 (exact)",
                &["boltzmann", "thermodynamics", "fundamental", "exact"],
            ),
            constant_entry(
                "rydberg_constant",
                "Rydberg Constant",
                "R_\u{221e}",
                &tanmatra::constants::RYDBERG.to_string(),
                "m\u{207b}\u{00b9}",
                tanmatra::constants::RYDBERG,
                Some("\u{00b1}0.000 000 0021"),
                "CODATA 2022",
                &["rydberg", "spectroscopy", "hydrogen", "atomic"],
            ),
            constant_entry(
                "bohr_radius",
                "Bohr Radius",
                "a_0",
                &tanmatra::constants::BOHR_RADIUS.to_string(),
                "m",
                tanmatra::constants::BOHR_RADIUS,
                Some("\u{00b1}0.000 000 000 80e-11"),
                "CODATA 2022",
                &["bohr", "atomic", "hydrogen", "radius"],
            ),
            constant_entry(
                "atomic_mass_unit",
                "Atomic Mass Unit",
                "u",
                &tanmatra::constants::AMU_MEV.to_string(),
                "MeV/c\u{00b2}",
                tanmatra::constants::AMU_MEV,
                Some("\u{00b1}0.000 000 28"),
                "CODATA 2022",
                &["amu", "dalton", "mass", "nuclear"],
            ),
            constant_entry(
                "bohr_magneton",
                "Bohr Magneton",
                "\u{03bc}_B",
                &tanmatra::constants::BOHR_MAGNETON_EV_T.to_string(),
                "eV/T",
                tanmatra::constants::BOHR_MAGNETON_EV_T,
                Some("\u{00b1}0.000 000 0017e-5"),
                "CODATA 2022",
                &["magneton", "magnetic", "electron", "atomic"],
            ),
            constant_entry(
                "coulomb_constant_nuclear",
                "Coulomb Constant (nuclear scale)",
                "k_e",
                &tanmatra::constants::COULOMB_MEV_FM.to_string(),
                "MeV\u{00b7}fm/e\u{00b2}",
                tanmatra::constants::COULOMB_MEV_FM,
                None,
                "CODATA 2022",
                &["coulomb", "electromagnetic", "nuclear"],
            ),
        ]
    }
}

#[allow(clippy::too_many_arguments)]
fn constant_entry(
    id: &str,
    title: &str,
    symbol: &str,
    value: &str,
    unit: &str,
    numeric: f64,
    uncertainty: Option<&str>,
    authority: &str,
    tags: &[&str],
) -> Entry {
    Entry::new(
        id,
        title,
        Domain::Physics,
        format!("{title} ({symbol} = {value} {unit})."),
        EntryKind::Constant(Constant {
            symbol: symbol.into(),
            value: value.into(),
            unit: unit.into(),
            numeric,
            uncertainty: uncertainty.map(Into::into),
            authority: authority.into(),
        }),
        "tanmatra",
        tags.iter().map(|t| (*t).into()).collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_metadata() {
        let p = TanmatraProvider;
        assert_eq!(p.source_name(), "tanmatra");
        assert_eq!(p.domain(), Domain::Physics);
    }

    #[test]
    fn entries_cover_key_constants() {
        let entries = TanmatraProvider.entries();
        assert!(entries.len() >= 10);

        let ids: Vec<&str> = entries.iter().map(|e| e.id.as_str()).collect();
        assert!(ids.contains(&"speed_of_light"));
        assert!(ids.contains(&"fine_structure_constant"));
        assert!(ids.contains(&"planck_constant"));
        assert!(ids.contains(&"elementary_charge"));
        assert!(ids.contains(&"boltzmann_constant"));
    }

    #[test]
    fn all_entries_are_constants() {
        for entry in TanmatraProvider.entries() {
            assert!(
                matches!(entry.kind, EntryKind::Constant(_)),
                "{} is not a Constant",
                entry.id
            );
        }
    }
}
