# tmi-sys

This crate provides raw Rust bindings (generated by bindgen) to tmi.cxx.

# Instructions

## Building example(-s)

NOTE: this is only confirmed to work on Linux.

1. Install `node` (and `npm`) and `libclang`
2. `git clone` this repository
3. Download the tmi.cxx submodule using `git submodule init --recursive`
4. `cd` into `tmi-sys/tmi_cxx`
5. Run `npm install`
6. `cd` into `tmi-sys/tmi_cxx/build/Release`
7. Make a symlink to `tmi_cxx.node` by running `ln -s tmi_cxx.node libtmi_cxx.so`
8. Run `cargo build`.
   NOTE: You might have to set the `LIBCLANG_PATH` to the directory that 
   contains libclang.so if you get an error message.
9. Run `cargo build --example bot`
10. Create a `config.json` and a `secret.json` for your bot.  The format is
   described in the [tmi.cxx](https://github.com/walterpie/tmi.cxx) repo.
11. Now you can run the example with `node tmi_cxx/tmi_cxx.js ./target/debug/examples/libbot.so config.json secret.json`
