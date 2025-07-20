use criterion::{black_box, criterion_group, criterion_main, Criterion};
use my_rust_project_core::*;

fn benchmark_function(c: &mut Criterion) {
    c.bench_function("example benchmark", |b| {
        b.iter(|| {
            // Benchmark code here
            black_box(2 + 2)
        })
    });
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
