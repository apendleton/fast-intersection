extern crate cpp_build;

use std::path::Path;
use std::process::Command;

fn main() {
    if !Path::new("SIMDCompressionAndIntersection/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                   .status();
    }

    cpp_build::Config::new()
        .include("SIMDCompressionAndIntersection/include")
        .file("SIMDCompressionAndIntersection/src/intersection.cpp")
        .flag("-fpic")
        .flag("-mavx")
        .flag("-mavx2")
        .flag("-std=c++11")
        .flag("-Weffc++")
        .flag("-pedantic")
        .flag("-O3")
        .build("lib.rs");
}

