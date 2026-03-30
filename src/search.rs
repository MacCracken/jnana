//! Cross-domain knowledge search.

use crate::domain::Domain;
use crate::entry::Entry;
use crate::registry::Registry;
use serde::{Deserialize, Serialize};

/// A search query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Free-text search.
    pub text: Option<String>,
    /// Filter by domain.
    pub domain: Option<Domain>,
    /// Filter by tags (all must match).
    pub tags: Vec<String>,
    /// Maximum results.
    pub limit: Option<usize>,
}

impl SearchQuery {
    /// Simple text search.
    #[must_use]
    pub fn text(query: impl Into<String>) -> Self {
        Self {
            text: Some(query.into()),
            domain: None,
            tags: vec![],
            limit: None,
        }
    }
}

/// A search result with relevance score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub domain: Domain,
    pub summary: String,
    pub score: f32,
}

/// Execute a search.
#[must_use]
pub fn search(registry: &Registry, query: &SearchQuery) -> Vec<SearchResult> {
    let mut results: Vec<SearchResult> = registry
        .list()
        .into_iter()
        .filter_map(|entry| score_entry(entry, query))
        .collect();

    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    if let Some(limit) = query.limit {
        results.truncate(limit);
    }

    results
}

fn score_entry(entry: &Entry, query: &SearchQuery) -> Option<SearchResult> {
    let mut score: f32 = 0.0;

    if let Some(domain) = query.domain
        && entry.domain != domain
    {
        return None;
    }

    for tag in &query.tags {
        if !entry.has_tag(tag) {
            return None;
        }
        score += 1.0;
    }

    if let Some(text) = &query.text {
        let lower = text.to_lowercase();
        let tokens: Vec<&str> = lower.split_whitespace().collect();
        let score_before = score;

        for token in &tokens {
            if entry.id.to_lowercase().contains(token) {
                score += 3.0;
            }
            if entry.title.to_lowercase().contains(token) {
                score += 2.0;
            }
            if entry.summary.to_lowercase().contains(token) {
                score += 1.0;
            }
            for tag in &entry.tags {
                if tag.to_lowercase().contains(token) {
                    score += 1.5;
                }
            }
        }

        if score == score_before {
            return None;
        }
    }

    if query.text.is_none() && query.tags.is_empty() && query.domain.is_none() {
        score = 1.0;
    }

    Some(SearchResult {
        id: entry.id.clone(),
        title: entry.title.clone(),
        domain: entry.domain,
        summary: entry.summary.clone(),
        score,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::{Constant, EntryKind};

    fn make_registry() -> Registry {
        let mut reg = Registry::new();
        reg.register(Entry {
            id: "speed_of_light".into(),
            title: "Speed of Light".into(),
            domain: Domain::Physics,
            summary: "Fundamental constant of nature.".into(),
            kind: EntryKind::Constant(Constant {
                symbol: "c".into(),
                value: "299792458".into(),
                unit: "m/s".into(),
                numeric: 299_792_458.0,
                uncertainty: None,
                authority: "CODATA".into(),
            }),
            source: "prakash".into(),
            tags: vec!["light".into(), "fundamental".into()],
        });
        reg.register(Entry {
            id: "pi".into(),
            title: "Pi".into(),
            domain: Domain::Mathematics,
            summary: "Ratio of circumference to diameter.".into(),
            kind: EntryKind::Constant(Constant {
                symbol: "π".into(),
                value: "3.14159265358979".into(),
                unit: "".into(),
                numeric: std::f64::consts::PI,
                uncertainty: None,
                authority: "exact".into(),
            }),
            source: "hisab".into(),
            tags: vec!["circle".into(), "geometry".into(), "fundamental".into()],
        });
        reg
    }

    #[test]
    fn search_text() {
        let reg = make_registry();
        let results = search(&reg, &SearchQuery::text("light"));
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "speed_of_light");
    }

    #[test]
    fn search_domain_filter() {
        let reg = make_registry();
        let mut q = SearchQuery::text("fundamental");
        q.domain = Some(Domain::Mathematics);
        let results = search(&reg, &q);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "pi");
    }

    #[test]
    fn search_no_match() {
        let reg = make_registry();
        let results = search(&reg, &SearchQuery::text("quantum"));
        assert!(results.is_empty());
    }
}
