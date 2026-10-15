use std::{env, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // The runtime parser is Rust; ggml is built only as the test oracle.
    if env::var_os("CARGO_FEATURE_REFERENCE_ORACLE").is_none() {
        return;
    }
    let source = Path::new("vendor/ggml");
    assert!(
        source.join("include/gguf.h").exists(),
        "ggml submodule is missing: run git submodule update --init --recursive"
    );
    for path in ["include", "src", "cmake", "CMakeLists.txt"] {
        println!("cargo:rerun-if-changed={}", source.join(path).display());
    }
    println!("cargo:rerun-if-changed=.git/modules/vendor/ggml/HEAD");

    // Build only ggml-base, which contains the GGUF reader. No CPU/GPU
    // execution backend is built or called by our engine.
    let dst = cmake::Config::new(source)
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("GGML_CPU", "OFF")
        .define("GGML_NATIVE", "OFF")
        .define("GGML_OPENMP", "OFF")
        .define("GGML_METAL", "OFF")
        .define("GGML_BLAS", "OFF")
        .define("GGML_BUILD_TESTS", "OFF")
        .define("GGML_BUILD_EXAMPLES", "OFF")
        .define("GGML_CCACHE", "OFF")
        .build_target("ggml-base")
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("build/src").display()
    );
    println!("cargo:rustc-link-lib=static=ggml-base");
    match env::var("CARGO_CFG_TARGET_OS").as_deref() {
        Ok("linux") => {
            println!("cargo:rustc-link-lib=stdc++");
            println!("cargo:rustc-link-lib=pthread");
            println!("cargo:rustc-link-lib=m");
        }
        Ok("macos") => println!("cargo:rustc-link-lib=c++"),
        _ => panic!("GGUF native build currently supports Linux and macOS"),
    }
}
