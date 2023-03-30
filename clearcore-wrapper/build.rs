use std::env;
use std::path::PathBuf;
use bindgen;

fn main() {
    let output_path = "libClearCore_bindings.rs";
    let header_dirs = [
        "headers/libClearCore",
        "headers/LwIP",
        "headers/SAME53_DFP",
        "headers/CMSIS",
        "headers/arm-none-eabi",
    ];
    let allowlist = [
        // Only create bindings for ClearCore, not it's include dependencies
        "headers/libClearCore.*\\.h",
    ];

    // don't derive copy for these types
    let no_copy = [
        "ClearCore::Connector",
    ];

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.hpp");

    let mut bindopts = bindgen::Builder::default()
        .header("wrapper.hpp")

        // Use core instead of libstd in the generated bindings.
        .use_core()
        .ctypes_prefix("core::ffi")
        .respect_cxx_access_specs(true)

        .vtable_generation(true)
        .generate_inline_functions(true)

        .sort_semantically(true)
        .merge_extern_blocks(true);

    for dir in header_dirs.iter() {
        bindopts = bindopts.clang_arg("-I").clang_arg(*dir);
    }

    for file_pat in allowlist.iter() {
        bindopts = bindopts.allowlist_file(*file_pat);
    }

    for type_pat in no_copy.iter() {
        bindopts = bindopts.no_copy(*type_pat);
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
