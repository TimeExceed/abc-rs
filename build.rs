use std::env;
use std::fs;
use std::path::*;

fn main() -> Result<(), String> {
    println!("cargo:rerun-if-changed=./abc");

    let cpps: Vec<_> = {
        let contents = fs::read_to_string("./cpps").unwrap_or_else(|err| {
            panic!("./cpps: {err:?}")
        });
        let base_dir = Path::new("abc");
        contents.split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| base_dir.join(x).canonicalize().unwrap_or_else(|err| {
                panic!("src file {}: {err:?}", base_dir.join(x).display())
            }))
            .collect()
    };
    let mut build = cc::Build::new();
    build.files(cpps)
        .includes(["abc/src", "/usr/include/c++/v1", "/usr/include/"])
        .define("ABC_USE_PIC", "0")
        .define("ABC_USE_NO_READLINE", "1")
        .define("ABC_USE_NO_PTHREADS", "1")
        .define("ABC_USE_STDINT_H", "1")
        .define("ABC_USE_CUDD", "1")
        .define("SIZEOF_VOID_P", "8")
        .define("SIZEOF_LONG", "8")
        .define("SIZEOF_INT", "4")
        .no_default_flags(true)
        .cpp(true)
        .cpp_link_stdlib(None)
        .warnings(false)
        .opt_level(3)
        .std("c++17")
        .static_flag(true)
        .shared_flag(false)
        .flag("-nostdinc")
        .flag("-nostdlib")
        .flag("-Wno-write-strings")
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-but-set-variable")
        .flag("-Wno-deprecated")
        .flag("-Wno-shift-op-parentheses");
    if let Ok(target) = std::env::var("TARGET") {
        if target.contains("musl") {
            if let Ok(wrapper) = std::env::var("RUST_WRAPPER") {
                if wrapper.contains("sccache") {
                    unsafe {std::env::set_var("CXX", "sccache clang++")};
                }
            }
        }
    }
    build.compile("abc");


    let out_dir = &env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(out_dir);
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=abc");
    Ok(())
}
