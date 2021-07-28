#[cfg(feature = "use-bindgen")]
extern crate bindgen;

fn main() {
    println!("cargo:rustc-link-lib=dylib=asound");

    #[cfg(feature = "use-bindgen")]
    generate_bindings(&_alsa_library);
}

#[cfg(feature = "use-bindgen")]
fn generate_bindings(alsa_library: &pkg_config::Library) {
    use std::env;
    use std::path::PathBuf;

    let clang_include_args = alsa_library.include_paths.iter().map(|include_path| {
        format!(
            "-I{}",
            include_path.to_str().expect("include path was not UTF-8")
        )
    });

    let mut codegen_config = bindgen::CodegenConfig::empty();
    codegen_config.insert(bindgen::CodegenConfig::FUNCTIONS);
    codegen_config.insert(bindgen::CodegenConfig::TYPES);

    let builder = bindgen::Builder::default()
        .size_t_is_usize(true)
        .whitelist_recursively(false)
        .prepend_enum_name(false)
        .layout_tests(false)
        .whitelist_function("snd_.*")
        .whitelist_type("_?snd_.*")
        .whitelist_type(".*va_list.*")
        .with_codegen_config(codegen_config)
        .clang_args(clang_include_args)
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("generated.rs"))
        .expect("Couldn't write bindings");
}
