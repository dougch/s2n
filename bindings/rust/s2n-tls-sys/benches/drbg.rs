use criterion::{criterion_group, criterion_main, Criterion};
use s2n_tls_sys::s2n_drbg_test;
use libc::{c_char,c_int};


pub fn criterion_s2n_drbg(c: &mut Criterion) {
    unsafe {
        let argc: c_int = 0;
        let mut arg: *mut c_char = std::ptr::null_mut();
        let argv: *mut *mut c_char = &mut arg;

        c.bench_function("s2n_drbg_test", |b| b.iter(|| dbg!(s2n_drbg_test(argc, argv))));
    }
}

criterion_group!(benches, criterion_s2n_drbg);
criterion_main!(benches);
