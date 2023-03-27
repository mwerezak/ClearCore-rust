use std::env;
use std::path::PathBuf;
use bindgen;

fn main() {
    let output_path = "libClearCore_bindgen.rs";
    let header_dirs = [
        "headers/libClearCore",
        "headers/LwIP",
        "headers/SAME53_DFP",
        "headers/CMSIS",
        "headers/arm-none-eabi",
    ];
    let allowlist = [
        // Only create bindings for ClearCore, not it's include dependencies
        "headers/libClearCore/\\w+\\.h"
    ];

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.hpp");

    let mut bindopts = bindgen::Builder::default()
        .header("wrapper.hpp")
        // Use core instead of libstd in the generated bindings.
        .use_core()
        .ctypes_prefix("core::ffi");

    for path in header_dirs.iter() {
        bindopts = bindopts
            .clang_arg("-I").clang_arg(*path);
    }

    for pattern in allowlist.iter() {
        bindopts = bindopts
            .allowlist_file(*pattern);
    }

    let bindings = bindopts
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join(output_path))
        .expect("Couldn't write bindings!");
}