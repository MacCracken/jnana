use criterion::{Criterion, criterion_group, criterion_main};
use jnana::entry::{Constant, EntryKind};
use jnana::portal;
use jnana::search::search;
use jnana::source::{Source, SourceKind};
use jnana::{Domain, Entry, Registry, SearchQuery};

/// Build a registry with N synthetic entries across all domains.
fn populated_registry(n: usize) -> Registry {
    let domains = Domain::all();
    let mut reg = Registry::new();
    for i in 0..n {
        let domain = domains[i % domains.len()];
        reg.register(Entry::new(
            format!("entry_{i}"),
            format!("Entry Title {i} for {}", domain.display_name()),
            domain,
            format!("Summary of entry {i} covering {}", domain.description()),
            EntryKind::Constant(Constant {
                symbol: format!("x{i}"),
                value: format!("{}", i as f64 * 1.23),
                unit: "unit".into(),
                numeric: i as f64 * 1.23,
                uncertainty: None,
                authority: "test".into(),
            }),
            "benchmark",
            vec![
                format!("tag_{}", i % 10),
                format!("domain_{}", domain.display_name().to_lowercase()),
                "common".into(),
            ],
        ));
    }
    reg
}

fn bench_search_text(c: &mut Criterion) {
    let reg = populated_registry(1000);
    let query = SearchQuery::text("light");
    c.bench_function("search_text_1000", |b| {
        b.iter(|| std::hint::black_box(search(&reg, &query)));
    });
}

fn bench_search_broad(c: &mut Criterion) {
    let reg = populated_registry(1000);
    let query = SearchQuery::text("Entry Title");
    c.bench_function("search_broad_1000", |b| {
        b.iter(|| std::hint::black_box(search(&reg, &query)));
    });
}

fn bench_search_domain_filter(c: &mut Criterion) {
    let reg = populated_registry(1000);
    let mut query = SearchQuery::text("Entry");
    query.domain = Some(Domain::Physics);
    c.bench_function("search_domain_filter_1000", |b| {
        b.iter(|| std::hint::black_box(search(&reg, &query)));
    });
}

fn bench_registry_get_hit(c: &mut Criterion) {
    let reg = populated_registry(1000);
    c.bench_function("registry_get_hit_1000", |b| {
        b.iter(|| std::hint::black_box(reg.get("entry_500")));
    });
}

fn bench_registry_get_miss(c: &mut Criterion) {
    let reg = populated_registry(1000);
    c.bench_function("registry_get_miss_1000", |b| {
        b.iter(|| std::hint::black_box(reg.get("nonexistent")));
    });
}

fn bench_registry_list(c: &mut Criterion) {
    let reg = populated_registry(1000);
    c.bench_function("registry_list_1000", |b| {
        b.iter(|| std::hint::black_box(reg.list()));
    });
}

fn bench_registry_by_domain(c: &mut Criterion) {
    let reg = populated_registry(1000);
    c.bench_function("registry_by_domain_1000", |b| {
        b.iter(|| std::hint::black_box(reg.by_domain(Domain::Physics)));
    });
}

fn bench_total_size(c: &mut Criterion) {
    let reg = populated_registry(1000);
    c.bench_function("total_size_1000", |b| {
        b.iter(|| std::hint::black_box(reg.total_size()));
    });
}

fn bench_linker(c: &mut Criterion) {
    c.bench_function("linker_resolve_500", |b| {
        b.iter(|| {
            let mut reg = populated_registry(500);
            jnana::linker::resolve_links(&mut reg);
        });
    });
}

fn bench_portal_generate(c: &mut Criterion) {
    let reg = populated_registry(200);
    let sources: Vec<Source> = (0..10)
        .map(|i| {
            Source::new(
                format!("source_{i}"),
                format!("Source {i}"),
                Domain::all()[i % Domain::all().len()],
                SourceKind::Zim,
                "",
                (i as u64 + 1) * 500,
            )
        })
        .collect();
    let config = portal::PortalConfig::default();
    c.bench_function("portal_generate_200", |b| {
        b.iter(|| std::hint::black_box(portal::generate(&config, &reg, &sources)));
    });
}

fn bench_content_index(c: &mut Criterion) {
    let sources: Vec<Source> = (0..50)
        .map(|i| {
            Source::new(
                format!("source_{i}"),
                format!("Source {i}"),
                Domain::all()[i % Domain::all().len()],
                SourceKind::Zim,
                "",
                100,
            )
        })
        .collect();
    c.bench_function("content_index_50", |b| {
        b.iter(|| {
            let mut reg = Registry::new();
            jnana::content::index_sources(&mut reg, &sources);
            std::hint::black_box(reg.len());
        });
    });
}

criterion_group!(
    benches,
    bench_search_text,
    bench_search_broad,
    bench_search_domain_filter,
    bench_registry_get_hit,
    bench_registry_get_miss,
    bench_registry_list,
    bench_registry_by_domain,
    bench_total_size,
    bench_linker,
    bench_portal_generate,
    bench_content_index,
);
criterion_main!(benches);
