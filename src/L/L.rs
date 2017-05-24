//! The dynamically-loaded library crate.

extern crate C;

#[no_mangle]
pub extern "C" fn hello_l() {
    println!("Hello from crate L!");
    C::hello_c();
}

