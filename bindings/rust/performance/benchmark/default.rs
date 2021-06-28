use s2n_tls::raw::*; 
use criterion::{criterion_group, criterion_main, Criterion};

fn simple_benchmark(){
}

fn criterion_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("smallsample");
    group.significance_level(0.1).sample_size(10);
    group.bench_function("s2ndrbgtest", |b| b.iter(|| simple_benchmark()));
    group.finish();
}

criterion_group!(benches, criterion_bench);
criterion_main!(benches);

