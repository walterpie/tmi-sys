use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=tmi_cxx");
    println!("cargo:rustc-link-search=./tmi_cxx/build/Release");

    println!("cargo:rerun-if-changed=include/tmi_cxx.h");

    let bindings = bindgen::Builder::default()
        .header("include/tmi_cxx.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("tmi.rs"))
        .expect("Couldn't write bindings!");
}
