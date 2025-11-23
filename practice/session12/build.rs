fn main() {
    // Standard linker args for embedded
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");

    // Build the C library
    cc::Build::new()
        .file("src/mathlib.c")
        .compile("mathlib");

    // Rebuild if C files change
    println!("cargo:rerun-if-changed=src/mathlib.c");
    println!("cargo:rerun-if-changed=src/mathlib.h");
}
