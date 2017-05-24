//! The common crate

static mut COUNT: usize = 0;

pub fn hello_c() {
    unsafe {
        println!("Hello from crate C: COUNT = {}", COUNT);
        if COUNT == 1 {
            println!("Yay! :D");
        }
        COUNT += 1;
    }
}
