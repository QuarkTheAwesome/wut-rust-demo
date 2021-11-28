extern crate bindgen;

use std::fs;
use std::env;
use std::path::PathBuf;

fn main() {
    //TODO rplimportgen
    //TODO link rpls

    println!("cargo:rerun-if-changed=src/ffi/cafe.h");

    let libgcc_dirs = fs::read_dir("/opt/devkitpro/devkitPPC/lib/gcc/powerpc-eabi")
        .expect("Could not find devkitPPC headers");
    //am I misreading the API doc here
    let libgcc = libgcc_dirs.last()
        .expect("Could not find devkitPPC headers")
        .expect("Could not find devkitPPC headers")
        .path();

    let bindings = bindgen::Builder::default()
        .header("src/ffi/cafe.h")
        .blocklist_type("OSSpinLock") //see lib.rs
        .blocklist_type("MEMHeapHeader")
        .default_enum_style(bindgen::EnumVariation::NewType{ is_bitfield: true })
        .use_core()
        .ctypes_prefix("cty")
        .detect_include_paths(false)
        .clang_arg("-nostdinc")
        .clang_arg("--sysroot=/opt/devkitpro/devkitPPC/powerpc-eabi")
        .clang_arg("-Iext/wut/include")
        .clang_arg("-I/opt/devkitpro/devkitPPC/powerpc-eabi/include")
        .clang_arg(format!("-I{}", libgcc.join("include").to_str().unwrap()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate Cafe bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write out bindings!");
}
