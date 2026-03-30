//! Web portal generation from registry and sources.
//!
//! Generates a self-contained HTML portal from the knowledge registry,
//! replacing the hand-coded index.html. The portal is always in sync
//! with the actual content because it's generated from the same data.

/// Placeholder for portal generation.
/// Will generate HTML from Registry + Source list.
#[non_exhaustive]
pub struct PortalConfig {
    /// Title shown in the portal header.
    pub title: String,
    /// Kiwix server port for ZIM links.
    pub kiwix_port: u16,
}

impl Default for PortalConfig {
    fn default() -> Self {
        Self {
            title: "Jnana — Offline Knowledge".into(),
            kiwix_port: 8888,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portal_config_default() {
        let cfg = PortalConfig::default();
        assert_eq!(cfg.kiwix_port, 8888);
        assert!(cfg.title.contains("Jnana"));
    }
}
