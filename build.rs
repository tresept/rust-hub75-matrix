use std::{env, path::PathBuf, process::Command};

fn main() {
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR missing"));
    let upstream = manifest_dir.join("vendor/rpi-rgb-led-matrix");
    let header = upstream.join("include/led-matrix-c.h");
    if !header.exists() {
        panic!(
            "rpi-rgb-led-matrix submodule is missing; run: git submodule update --init --recursive"
        );
    }
    let jobs = env::var("NUM_JOBS").unwrap_or_else(|_| "1".into());
    let status = Command::new("make")
        .arg("-C")
        .arg(upstream.join("lib"))
        .arg(format!("-j{jobs}"))
        .status()
        .expect("failed to execute make for rpi-rgb-led-matrix");
    assert!(status.success(), "failed to build rpi-rgb-led-matrix");

    cc::Build::new()
        .cpp(true)
        .file("native/bridge.cc")
        .include("native")
        .include(upstream.join("include"))
        .flag_if_supported("-std=c++17")
        .warnings(true)
        .compile("rust_hub75_bridge");
    println!(
        "cargo:rustc-link-search=native={}",
        upstream.join("lib").display()
    );
    // The bridge calls the upstream C API, but Rust has no direct extern
    // declaration for those symbols. Force the complete upstream archive into
    // the final link so the linker retains the C API members needed by bridge.o.
    println!("cargo:rustc-link-lib=static:+whole-archive=rgbmatrix");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=rt");
    println!("cargo:rustc-link-lib=dylib=m");
    for path in ["native/bridge.h", "native/bridge.cc"] {
        println!("cargo:rerun-if-changed={path}");
    }
    println!("cargo:rerun-if-changed={}", header.display());
}
