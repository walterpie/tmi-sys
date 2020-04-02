use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let status = Command::new("git")
        .arg("submodule")
        .arg("init")
        .spawn()
        .expect("Couldn't run `git submodule init`")
        .wait()
        .expect("Waiting for the child failed");
    if !status.success() {
        panic!("`git submodule init` returned a failure");
    }

    let status = Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--recursive")
        .spawn()
        .expect("Couldn't run `git submodule update --recursive`")
        .wait()
        .expect("Waiting for the child failed");
    if !status.success() {
        panic!("`git submodule update --recursive` returned a failure");
    }

    let status = Command::new("npm")
        .arg("install")
        .current_dir(fs::canonicalize("./tmi_cxx").expect("./tmi_cxx does not exist"))
        .spawn()
        .expect("Couldn't run `npm install`")
        .wait()
        .expect("Waiting for the child failed");
    if !status.success() {
        panic!("`npm install` returned a failure");
    }

    let status = Command::new("ln")
        .arg("-s")
        .arg("tmi_cxx.node")
        .arg("libtmi_cxx.so")
        .current_dir(fs::canonicalize("./tmi_cxx/build/Release").expect("./tmi_cxx does not exist"))
        .spawn()
        .expect("Couldn't run `ln`")
        .wait()
        .expect("Waiting for the child failed");
    if !status.success() {
        panic!("`ln` returned a failure");
    }

    println!("cargo:rustc-link-lib=tmi_cxx");
    let search_path = fs::canonicalize("./tmi_cxx/build/Release")
        .expect("./tmi_cxx/build/Release does not exist");
    // note this shouldn' fail, so to_string_lossy() is fine
    println!("cargo:rustc-link-search={}", search_path.to_string_lossy());

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
