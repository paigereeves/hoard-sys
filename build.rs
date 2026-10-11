fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    assert_eq!(
        target_os, "linux",
        "hoard-sys currently supports Linux targets"
    );

    for path in ["build.rs", "Hoard/src", "Heap-Layers"] {
        println!("cargo:rerun-if-changed={}", path);
    }

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++20")
        .file("Hoard/src/source/libhoard.cpp")
        .file("Hoard/src/source/unixtls.cpp")
        .include("Hoard/src")
        .include("Hoard/src/include")
        .include("Hoard/src/include/util")
        .include("Hoard/src/include/hoard")
        .include("Hoard/src/include/superblocks")
        .include("Heap-Layers")
        .define("NDEBUG", None)
        .define("_REENTRANT", "1")
        // Calls can originate in threads created before Hoard is initialized.
        .define("HOARD_NO_LOCK_OPT", "1")
        .flag_if_supported("-fno-builtin-malloc");
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("musl") {
        build.define("__MUSL__", None);
    }
    build.compile("hoard");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");
}
