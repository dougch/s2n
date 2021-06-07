use s2n_tls_sys::*;
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


fn main() {
    println!("Starting...");
    s2n_drbg();
    println!("once more...");
    s2n_drbg();
    println!("Done.");
}
