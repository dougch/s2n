use s2n_tls_sys::*;
use libc::{c_char,c_int};

pub fn s2n_pq_init() -> s2n_result{
    let result = s2n_result {
        __error_signal: 0,
    };
    return result
}

fn main() {
    println!("Starting...");
    println!("Running everything inside s2n_init (minus s2n_pq_init)...");
    unsafe{

        let result =s2n_fips_init();
        println!("s2n_fips_init result: {:?}", result);
        let result =s2n_mem_init();
        println!("s2n_mem_init result: {:?}", result);
        let result = s2n_rand_init();
        println!("s2n_rand_init result: {:?}", result);
        let result =s2n_cipher_suites_init();
        println!("s2n_cipher_suites_init result: {:?}", result);
        let result =s2n_security_policies_init();
        println!("s2n_security_policies_init result: {:?}", result);
        let result =s2n_config_defaults_init();
        println!("s2n_config_defaults_init result: {:?}", result);
        let result =s2n_extension_type_init();
        println!("s2n_extentions_type_init result: {:?}", result);
    }
    println!("Running s2n_init...");
    unsafe{
        let result = s2n_init();
        println!("s2n_init result: {:?}", result)
    }
    unsafe {
        let argc: c_int = 0;
        let mut arg: *mut c_char = std::ptr::null_mut();
        let argv: *mut *mut c_char = &mut arg;

        let result: c_int = s2n_drbg_test(argc, argv);
        println!("s2n_drbg_test: {:?}", result);
    }


    println!("Done.");
}