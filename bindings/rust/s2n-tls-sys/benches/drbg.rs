use criterion::{criterion_group, criterion_main, Criterion};
use s2n_tls_sys::s2n_drbg_test;
use libc::{c_char,c_int};


fn s2n_drbg() {
    unsafe {
        let argc: c_int = 0;
        let mut arg: *mut c_char = std::ptr::null_mut();
        let argv: *mut *mut c_char = &mut arg;
        let result = s2n_drbg_test(argc, argv);
        println!("s2n_drbg_test returned {:?}",result);
    }
}

fn criterion_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("smallsample");
    group.significance_level(0.1).sample_size(10);
    group.bench_function("s2ndrbgtest", |b| b.iter(|| s2n_drbg()));
    group.finish();
}

criterion_group!(benches, criterion_bench);
criterion_main!(benches);
