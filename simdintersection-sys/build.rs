extern crate cc;

use std::env;
use std::fs;
use std::path::{PathBuf, Path};
use std::process::Command;

fn main() {
    if !Path::new("SIMDCompressionAndIntersection/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                   .status();
    }

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let include = dst.join("include");
    let build = dst.join("build");

    fs::create_dir_all(include.join("SIMDCompressionAndIntersection")).unwrap();
    fs::copy("SIMDCompressionAndIntersection/include/intersection.h", include.join("SIMDCompressionAndIntersection/intersection.h")).unwrap();
    fs::copy("SIMDCompressionAndIntersection/include/common.h", include.join("SIMDCompressionAndIntersection/common.h")).unwrap();
    fs::copy("SIMDCompressionAndIntersection/include/platform.h", include.join("SIMDCompressionAndIntersection/platform.h")).unwrap();

    let mut cfg = cc::Build::new();
    cfg.out_dir(&build)
        .include("SIMDCompressionAndIntersection/include")
        .file("SIMDCompressionAndIntersection/src/intersection.cpp")

        .flag("-fpic")
        .flag("-mavx")
        .flag("-std=c++11")
        .flag("-Weffc++")
        .flag("-pedantic")
        .flag("-O3")
        .flag("-Wall")
        .flag("-Wextra")

        .cpp(true)

        .compile("SIMDCompressionAndIntersection");
}

