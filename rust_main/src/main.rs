extern "C" {
    pub fn hello_world();
}

fn main() {
    unsafe {
        hello_world();
    }
    println!("Hello, world! From rust_main");
}
