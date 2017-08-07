extern crate gcc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let root_base = env::var("ROOTSYS").expect("ROOT include not found!");
    let root_inc = format!("{}/include", root_base);
    let root_lib = format!("{}/lib", root_base);

    let mut cfg = gcc::Config::new();
    cfg
        .cpp(true) // Switch to C++ library compilation.
        .include(&root_inc)
        // The auto-generated file for the ESD root tree
        .file("src/ffi/cpp_src/ESD.cxx");

    // ROOT libraries
    for lib in ["Tree", "Physics", "EG", "Geom", "Minuit", "VMC"].iter() {
        cfg.object(format!("{}/lib{}.so", root_lib, lib));
    }
    cfg.compile("libMyESD.a");

    // Use this extra bit of configuration to avoid the constructor
    // (and idealy the other not-working member functions)
    let mut config = bindgen::CodegenConfig::nothing();
    config.functions = true;
    config.types = true;
    config.vars = true;
    
    let bindings = bindgen::Builder::default()
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .clang_arg("-I/home/christian/repos/alice/sw/ubuntu1604_x86-64/ROOT/latest/include/")
        .whitelisted_type("ESD")
        // Whitelist esd help functions in that file
        .whitelisted_function("esd_.*")
        .whitelisted_function("tobjarray_.*")
        .opaque_type(r"T\w+")
        // Do not generate unstable Rust code that
        // requires a nightly rustc and enabling
        // unstable features.
        .unstable_rust(false)
        // Don't generate comments
        .generate_comments(false)
        // The input header we would like to generate
        // bindings for.
        .header("src/ffi/cpp_src/ESDmerged.h")
        .raw_line("#[allow(non_camel_case_types)]")
        .raw_line("#[allow(non_snake_case)]")
        .with_codegen_config(config)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
