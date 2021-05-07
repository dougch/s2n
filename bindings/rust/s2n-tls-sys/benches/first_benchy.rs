use criterion::{criterion_group, criterion_main, Criterion};
use s2n_tls_sys::*;
pub use libc::c_int as s2n_status_code;


pub fn criterion_s2n_stuffer(c: &mut Criterion) {

    #[inline]
    fn bench_stuffer_write(input_string: String) {
        let sample_text = String::from("Lorem ipsum dolor sit amet");
        let stuffer = s2n_stuffer {blob:0};
        s2n_stuffer_alloc_ro_from_string(stuffer, intput_string);
    }

    c.bench_function("bench_stuffer", |b| b.iter(|| bench_stuffer_write(sample_text)));

}

mod perf;

criterion_group!{
    name = benches;
    config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100));
    targets = criterion_s2n_stuffer
}
criterion_main!(benches);
