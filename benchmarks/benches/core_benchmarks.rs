use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rush::greet;

fn benchmark_function(c: &mut Criterion) {
    c.bench_function("greet", |b| {
        b.iter(|| {
            black_box(greet())
        })
    });
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
