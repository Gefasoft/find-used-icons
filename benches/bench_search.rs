#[macro_use]
extern crate criterion;

use {
    criterion::{Benchmark, Criterion},
    find_used_icons_lib::{find_used_icons, find_used_icons_seq, read_possible_icons},
};

// insert path to sapient-root
const SAPIENT_APP_ROOT: &'static str = r#"."#;

fn seq() {
    read_possible_icons(SAPIENT_APP_ROOT);
    find_used_icons_seq(SAPIENT_APP_ROOT);
}

fn par() {
    read_possible_icons(SAPIENT_APP_ROOT);
    find_used_icons(SAPIENT_APP_ROOT);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench(
        "Search in files",
        Benchmark::new("Sequentiell", |b| b.iter(|| seq()))
            .sample_size(20)
            .with_function("Parallel", |b| b.iter(|| par())),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
