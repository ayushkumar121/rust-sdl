use std::env;
// use std::path::Path;
// use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // if cfg!(target_os = "linux") {
    //     Command::new("cc")
    //         .args(["src/hello.c", "-c", "-o"])
    //         .arg(format!("{}/hello.o", out_dir))
    //         .status()
    //         .expect("cannot compile c code with gcc");

    //     Command::new("ar")
    //         .args(["crus", "libhello.a", "hello.o"])
    //         .current_dir(Path::new(&out_dir))
    //         .status()
    //         .expect("cannot create library");
    // } else {
    //     panic!("windows, macos compile target not supported");
    // }

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-search=libs");
    println!("cargo:rustc-link-lib=static=SDL2");
    println!("cargo:rustc-link-lib=static=hello");
    println!("cargo:rerun-if-changed=src/hello.c");
}
