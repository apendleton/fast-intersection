extern crate cc;

use std::env;
use std::path::{PathBuf, Path};
use std::process::Command;

fn main() {
    if !Path::new("streamvbyte/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                   .status();
    }

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let build = dst.join("build");

    let mut cfg = cc::Build::new();
    cfg.out_dir(&build)
        .include("streamvbyte/include")

        .file("streamvbyte/src/streamvbyte_0124_decode.c")
        .file("streamvbyte/src/streamvbyte_0124_encode.c")
        .file("streamvbyte/src/streamvbyte_decode.c")
        .file("streamvbyte/src/streamvbyte_encode.c")
        .file("streamvbyte/src/streamvbytedelta_decode.c")
        .file("streamvbyte/src/streamvbytedelta_encode.c")

        .flag("-fPIC")
        .flag("-march=native")
        .flag("-std=c99")
        .flag("-O3")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-pedantic")
        .flag("-Wshadow")

        .compile("streamvbyte");
}

