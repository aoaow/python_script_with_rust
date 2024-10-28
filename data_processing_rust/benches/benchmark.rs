// benches/benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use data_processing_rust::process_data;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("process_data", |b| {
        b.iter(|| {
            process_data(
                black_box("data/input.csv"),
                black_box("output.csv"),
                black_box("value"),
            )
            .unwrap()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
