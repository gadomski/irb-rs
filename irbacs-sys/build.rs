extern crate bindgen;

use bindgen::Builder;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=irbacs_l64");
    let bindings = Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");
}
