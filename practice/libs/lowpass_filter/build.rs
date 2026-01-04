use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();

    let bindings = bindgen::Builder::default()
        .header("include/lowpass_filter.h")
        .use_core() // no_std
        .ctypes_prefix("core::ffi")
        .derive_copy(true)
        .derive_debug(false)
        .derive_default(false)
        .layout_tests(false) // IMPORTANT for no_std
        .clang_arg("--target=arm-none-eabi")
        .clang_arg("-march=armv8-m.main")
        .clang_arg("-mfloat-abi=hard")
        .clang_arg("-mfpu=fpv5-sp-d16")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=include/lowpass_filter.h");

    let mut lib_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    lib_path.push("lib");

    println!("cargo:rustc-link-search={}", lib_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=lowpass_filter");
}
