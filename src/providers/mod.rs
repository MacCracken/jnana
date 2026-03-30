//! Built-in knowledge providers for AGNOS science crates.
//!
//! Each provider is gated behind a feature flag matching the crate name.
//! Enable `agnos-all` to pull in every available provider.

use crate::provider::KnowledgeProvider;

// ── Feature-gated provider modules ────────────────────────────────────────

#[cfg(feature = "kimiya")]
pub mod kimiya;

#[cfg(feature = "tanmatra")]
pub mod tanmatra;

#[cfg(feature = "hisab")]
pub mod hisab;

#[cfg(feature = "khanij")]
pub mod khanij;

/// Returns all providers enabled by the current feature flags.
#[must_use]
pub fn all_providers() -> Vec<Box<dyn KnowledgeProvider>> {
    #[allow(unused_mut)]
    let mut providers: Vec<Box<dyn KnowledgeProvider>> = Vec::new();

    #[cfg(feature = "kimiya")]
    providers.push(Box::new(kimiya::KimiyaProvider));

    #[cfg(feature = "tanmatra")]
    providers.push(Box::new(tanmatra::TanmatraProvider));

    #[cfg(feature = "hisab")]
    providers.push(Box::new(hisab::HisabProvider));

    #[cfg(feature = "khanij")]
    providers.push(Box::new(khanij::KhanijProvider));

    providers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_providers_returns_vec() {
        // With no features enabled, returns empty
        let providers = all_providers();
        // Just verify it compiles and returns without panic
        let _ = providers.len();
    }
}
