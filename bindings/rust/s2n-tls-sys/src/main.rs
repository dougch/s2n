use s2n_tls_sys::*;
use s2n_utils::*;
use libc::{c_char,c_int};


mod s2n_utils {
    use std::sync::Once;
    static mut VAL: i32= 0;
    static INIT: Once = Once::new();

    pub fn s2n_init() -> i32 {
        unsafe {
            INIT.call_once(|| {
                VAL = s2n_tls_sys::s2n_init();
            });
            VAL
        }
    }
}

fn s2n_drbg() {
    unsafe {
        let argc: c_int = 0;
        let mut arg: *mut c_char = std::ptr::null_mut();
        let argv: *mut *mut c_char = &mut arg;
        s2n_in_unit_test_set(true);
        let result = s2n_drbg_test(argc, argv);
        println!("s2n_drbg_test returned {:?}",result);
    }
}


fn main() {
    s2n_utils::s2n_init();
    println!("Starting...");
    s2n_drbg();
    println!("once more...");
    s2n_drbg();
    println!("Done.");
}

