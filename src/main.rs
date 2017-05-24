//! The main crate.

extern crate libloading;

extern crate C;

pub fn hello_m() {
    println!("Hello from crate M!");
}

fn main() {
    hello_m();
    C::hello_c();
    unsafe {
        use libloading::*;
        let l = Library::new("target/debug/libL.so").unwrap();
        let sym: Symbol<extern "C" fn()> = l.get(b"hello_l\0").unwrap();
        sym();
    }
}
