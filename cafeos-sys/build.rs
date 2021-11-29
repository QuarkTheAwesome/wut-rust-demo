extern crate bindgen;

use std::fs;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let cafe_dir = fs::read_dir("ext/wut/cafe").expect("Could not find wut submodule");
    let asm_files = cafe_dir.map(|rpl| {
        let rpl_path = rpl.unwrap().path();
        let asm_path = out_path.join(rpl_path.file_name().unwrap()).with_extension("S");

        Command::new("/opt/devkitpro/tools/bin/rplimportgen")
            .arg(&rpl_path)
            .arg(&asm_path)
            .status()
            .expect("Could not convert rpl!");

        return asm_path;
    });

    cc::Build::new()
        .files(asm_files)
        .compiler("clang") //could also use powerpc-eabi-gcc from devkitPPC
        .compile("cafe");

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

    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write out bindings!");
}
