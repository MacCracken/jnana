//! Web portal generation from registry and sources.
//!
//! Generates a self-contained HTML portal from the knowledge registry
//! and external source list. The portal includes a search interface
//! with domain filtering and links to Kiwix-served ZIM content.

use crate::domain::Domain;
use crate::entry::EntryKind;
use crate::registry::Registry;
use crate::source::Source;
use std::fmt::Write;

/// Configuration for portal generation.
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
            title: "Jnana \u{2014} Offline Knowledge".into(),
            kiwix_port: 8888,
        }
    }
}

/// Generate a self-contained HTML portal.
///
/// The output is a single HTML file with embedded CSS and JavaScript
/// for search and domain filtering. No external dependencies.
#[must_use]
pub fn generate(config: &PortalConfig, registry: &Registry, sources: &[Source]) -> String {
    let mut html = String::with_capacity(16384);

    write_head(&mut html, &config.title);
    write_header(&mut html, &config.title);
    write_search(&mut html);
    write_domain_nav(&mut html, registry);
    write_sources_section(&mut html, sources, config.kiwix_port);
    write_entries_section(&mut html, registry);
    write_footer(&mut html);
    write_script(&mut html);
    let _ = write!(html, "</body></html>");

    html
}

fn write_head(html: &mut String, title: &str) {
    let _ = write!(
        html,
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<style>
:root {{ --bg: #0a0a0a; --fg: #e0e0e0; --accent: #4fc3f7; --card: #1a1a1a; --border: #333; }}
* {{ margin: 0; padding: 0; box-sizing: border-box; }}
body {{ font-family: system-ui, sans-serif; background: var(--bg); color: var(--fg); line-height: 1.6; }}
header {{ padding: 2rem; text-align: center; border-bottom: 1px solid var(--border); }}
header h1 {{ color: var(--accent); font-size: 1.8rem; }}
.search-box {{ padding: 1rem 2rem; position: sticky; top: 0; background: var(--bg); z-index: 10; border-bottom: 1px solid var(--border); }}
.search-box input {{ width: 100%; padding: 0.8rem; font-size: 1rem; background: var(--card); color: var(--fg); border: 1px solid var(--border); border-radius: 4px; }}
.domain-nav {{ display: flex; flex-wrap: wrap; gap: 0.5rem; padding: 1rem 2rem; }}
.domain-btn {{ padding: 0.3rem 0.8rem; font-size: 0.85rem; background: var(--card); color: var(--fg); border: 1px solid var(--border); border-radius: 3px; cursor: pointer; }}
.domain-btn.active {{ background: var(--accent); color: #000; border-color: var(--accent); }}
section {{ padding: 1rem 2rem; }}
section h2 {{ color: var(--accent); margin-bottom: 1rem; font-size: 1.3rem; }}
.card {{ background: var(--card); border: 1px solid var(--border); border-radius: 4px; padding: 1rem; margin-bottom: 0.8rem; }}
.card h3 {{ font-size: 1rem; margin-bottom: 0.3rem; }}
.card .meta {{ font-size: 0.8rem; color: #888; }}
.card .summary {{ font-size: 0.9rem; margin-top: 0.3rem; }}
.card .tags {{ margin-top: 0.5rem; }}
.tag {{ display: inline-block; font-size: 0.75rem; padding: 0.1rem 0.5rem; background: #222; border-radius: 2px; margin-right: 0.3rem; color: #aaa; }}
.source-card {{ display: flex; justify-content: space-between; align-items: center; }}
.source-card .size {{ color: var(--accent); font-weight: bold; white-space: nowrap; margin-left: 1rem; }}
.hidden {{ display: none; }}
footer {{ padding: 2rem; text-align: center; color: #555; font-size: 0.8rem; border-top: 1px solid var(--border); margin-top: 2rem; }}
</style>
</head>
<body>
"#
    );
}

fn write_header(html: &mut String, title: &str) {
    let _ = writeln!(
        html,
        "<header><h1>{title}</h1><p>Structured, verified, offline knowledge</p></header>"
    );
}

fn write_search(html: &mut String) {
    let _ = writeln!(
        html,
        r#"<div class="search-box"><input type="text" id="search" placeholder="Search knowledge..." autocomplete="off"></div>"#
    );
}

fn write_domain_nav(html: &mut String, registry: &Registry) {
    let counts = registry.domain_counts();
    let _ = write!(html, r#"<nav class="domain-nav">"#);
    let _ = write!(
        html,
        r#"<button class="domain-btn active" data-domain="all">All</button>"#
    );
    for domain in Domain::all() {
        let count = counts.get(domain).copied().unwrap_or(0);
        if count > 0 {
            let _ = write!(
                html,
                r#"<button class="domain-btn" data-domain="{}">{} ({})</button>"#,
                domain.display_name(),
                domain.display_name(),
                count
            );
        }
    }
    let _ = writeln!(html, "</nav>");
}

fn write_sources_section(html: &mut String, sources: &[Source], kiwix_port: u16) {
    let enabled: Vec<&Source> = sources.iter().filter(|s| s.enabled).collect();
    if enabled.is_empty() {
        return;
    }

    let _ = writeln!(html, r#"<section id="sources"><h2>External Sources</h2>"#);
    for source in &enabled {
        let link = match source.kind {
            crate::source::SourceKind::Zim => {
                format!(
                    r#" <a href="http://localhost:{kiwix_port}/{}" style="color:var(--accent)">Open</a>"#,
                    source.id
                )
            }
            _ => String::new(),
        };
        let _ = writeln!(
            html,
            r#"<div class="card source-card" data-domain="{}"><div><h3>{}{}</h3><div class="meta">{} &middot; {}</div><div class="summary">{}</div></div><div class="size">{} MB</div></div>"#,
            source.domain.display_name(),
            source.name,
            link,
            source.kind,
            source.domain.display_name(),
            source.notes,
            source.size_mb
        );
    }
    let _ = writeln!(html, "</section>");
}

fn write_entries_section(html: &mut String, registry: &Registry) {
    let entries = registry.list();
    if entries.is_empty() {
        return;
    }

    let _ = writeln!(html, r#"<section id="entries"><h2>Knowledge Entries</h2>"#);
    for entry in &entries {
        let kind_label = match &entry.kind {
            EntryKind::Fact(_) => "Fact",
            EntryKind::Constant(_) => "Constant",
            EntryKind::Procedure(_) => "Procedure",
            EntryKind::Table(_) => "Table",
        };
        let _ = write!(
            html,
            r#"<div class="card entry-card" data-domain="{}" data-tags="{}" data-id="{}"><h3>{}</h3><div class="meta">{} &middot; {} &middot; {}</div><div class="summary">{}</div>"#,
            entry.domain.display_name(),
            entry.tags.join(","),
            entry.id,
            entry.title,
            kind_label,
            entry.domain.display_name(),
            entry.source,
            entry.summary,
        );
        if !entry.tags.is_empty() {
            let _ = write!(html, r#"<div class="tags">"#);
            for tag in &entry.tags {
                let _ = write!(html, r#"<span class="tag">{tag}</span>"#);
            }
            let _ = write!(html, "</div>");
        }
        let _ = writeln!(html, "</div>");
    }
    let _ = writeln!(html, "</section>");
}

fn write_footer(html: &mut String) {
    let _ = writeln!(
        html,
        r#"<footer>Generated by Jnana &mdash; the foundation of knowing</footer>"#
    );
}

fn write_script(html: &mut String) {
    let _ = write!(
        html,
        r#"<script>
(function() {{
  const search = document.getElementById('search');
  const cards = document.querySelectorAll('.card');
  const domainBtns = document.querySelectorAll('.domain-btn');
  let activeDomain = 'all';

  function filter() {{
    const q = search.value.toLowerCase();
    cards.forEach(card => {{
      const domain = card.dataset.domain || '';
      const text = card.textContent.toLowerCase();
      const domainMatch = activeDomain === 'all' || domain === activeDomain;
      const textMatch = !q || text.includes(q);
      card.classList.toggle('hidden', !(domainMatch && textMatch));
    }});
  }}

  search.addEventListener('input', filter);

  domainBtns.forEach(btn => {{
    btn.addEventListener('click', () => {{
      domainBtns.forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      activeDomain = btn.dataset.domain;
      filter();
    }});
  }});
}})();
</script>
"#
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::{Constant, Entry, EntryKind};
    use crate::source::SourceKind;

    fn sample_registry() -> Registry {
        let mut reg = Registry::new();
        reg.register(Entry::new(
            "speed_of_light",
            "Speed of Light",
            Domain::Physics,
            "The speed of light in vacuum.",
            EntryKind::Constant(Constant {
                symbol: "c".into(),
                value: "299792458".into(),
                unit: "m/s".into(),
                numeric: 299_792_458.0,
                uncertainty: None,
                authority: "CODATA".into(),
            }),
            "tanmatra",
            vec!["light".into(), "fundamental".into()],
        ));
        reg
    }

    #[test]
    fn portal_config_default() {
        let cfg = PortalConfig::default();
        assert_eq!(cfg.kiwix_port, 8888);
        assert!(cfg.title.contains("Jnana"));
    }

    #[test]
    fn generate_produces_html() {
        let config = PortalConfig::default();
        let reg = sample_registry();
        let sources = vec![Source::new(
            "wikimed",
            "WikiMed",
            Domain::Medicine,
            SourceKind::Zim,
            "https://example.com",
            1200,
        )];
        let html = generate(&config, &reg, &sources);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Jnana"));
        assert!(html.contains("Speed of Light"));
        assert!(html.contains("WikiMed"));
        assert!(html.contains("</html>"));
    }

    #[test]
    fn generate_includes_domain_filter() {
        let config = PortalConfig::default();
        let reg = sample_registry();
        let html = generate(&config, &reg, &[]);
        assert!(html.contains("Physics"));
        assert!(html.contains("data-domain="));
    }

    #[test]
    fn generate_includes_search() {
        let config = PortalConfig::default();
        let reg = Registry::new();
        let html = generate(&config, &reg, &[]);
        assert!(html.contains("id=\"search\""));
        assert!(html.contains("<script>"));
    }

    #[test]
    fn generate_empty_registry() {
        let config = PortalConfig::default();
        let reg = Registry::new();
        let html = generate(&config, &reg, &[]);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("</html>"));
    }

    #[test]
    fn generate_kiwix_link() {
        let config = PortalConfig::default();
        let reg = Registry::new();
        let sources = vec![Source::new(
            "wikimed",
            "WikiMed",
            Domain::Medicine,
            SourceKind::Zim,
            "",
            1200,
        )];
        let html = generate(&config, &reg, &sources);
        assert!(html.contains("localhost:8888/wikimed"));
    }

    #[test]
    fn generate_no_kiwix_link_for_pdf() {
        let config = PortalConfig::default();
        let reg = Registry::new();
        let sources = vec![Source::new(
            "manual",
            "Manual",
            Domain::Medicine,
            SourceKind::Pdf,
            "",
            50,
        )];
        let html = generate(&config, &reg, &sources);
        assert!(!html.contains("localhost:8888/manual"));
    }
}
