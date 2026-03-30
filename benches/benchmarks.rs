use criterion::{Criterion, criterion_group, criterion_main};
use jnana::search::search;
use jnana::{Registry, SearchQuery};

fn bench_search_text(c: &mut Criterion) {
    let reg = Registry::new();
    let query = SearchQuery::text("light");
    c.bench_function("search_text_empty", |b| {
        b.iter(|| std::hint::black_box(search(&reg, &query)));
    });
}

fn bench_registry_get(c: &mut Criterion) {
    let reg = Registry::new();
    c.bench_function("registry_get_miss", |b| {
        b.iter(|| std::hint::black_box(reg.get("nonexistent")));
    });
}

criterion_group!(benches, bench_search_text, bench_registry_get);
criterion_main!(benches);
