use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    println!("cargo:rustc-link-search=native=.rust/lib/x86_64-apple-darwin");
    // println!("cargo:rustc-link-lib=static=hello");
}
