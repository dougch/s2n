use criterion::{criterion_group, criterion_main, Criterion};
use s2n_tls_sys::*;
pub use libc::c_int as s2n_status_code;


pub fn criterion_bench_s2n(c: &mut Criterion) {

    #[inline]
    fn bench_stuffer_write(input_string: String) {
        let sample_text = String::from("Lorem ipsum dolor sit amet");
        let stuffer = s2n_stuffer {blob:0};
        s2n_stuffer_alloc_ro_from_string(stuffer, intput_string);
    }

    c.bench_function("bench_stuffer", |b| b.iter(|| bench_stuffer_write(sample_text)));

}


criterion_group!(benches, criterion_bench_s2n);
criterion_main!(benches);
