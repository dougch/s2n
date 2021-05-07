use criterion::{black_box, criterion_group, criterion_main, Criterion};
use criterion_gettingstarted::{fibonacci_slow,fibonacci_fast};

pub fn criterion_bench_fib10(c: &mut Criterion) {
    c.bench_function("fib 20 - slow", |b| b.iter(|| fibonacci_slow(black_box(20))));
}
pub fn criterion_bench_fib20(c: &mut Criterion) {
    c.bench_function("fib 20 - fast", |b| b.iter(|| fibonacci_fast(black_box(20))));
}

mod perf;

criterion_group!{
    name = benches;
    config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100));
    targets = criterion_bench_fib10,criterion_bench_fib20
}

criterion_main!(benches);
