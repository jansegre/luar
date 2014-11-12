extern crate "pkg-config" as pkg_config;

use std::os;
use std::io::{mod, fs, Command};
use std::io::process::InheritFd;

fn main() {
    let mut opts = pkg_config::default_options("lua");
    opts.atleast_version = Some("5.2".to_string());
    match pkg_config::find_library_opts("lua", &opts) {
        Ok(()) => return,
        Err(..) => {}
    }

    let mut cflags = os::getenv("CFLAGS").unwrap_or(String::new());
    let target = os::getenv("TARGET").unwrap();
    let mingw = target.contains("windows-gnu");
    cflags.push_str(" -ffunction-sections -fdata-sections");

    if target.contains("i686") {
        cflags.push_str(" -m32");
    } else if target.as_slice().contains("x86_64") {
        cflags.push_str(" -m64");
    }
    if !target.contains("i686") {
        cflags.push_str(" -fPIC");
    }

    let src = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
    let dst = Path::new(os::getenv("OUT_DIR").unwrap());
    let _ = fs::mkdir(&dst.join("build"), io::USER_DIR);

    let mut cmd = Command::new("cmake");
    cmd.arg(src.join("lua"))
       .cwd(&dst.join("build"));
    if mingw {
        cmd.arg("-G").arg("Unix Makefiles");
    }
    run(cmd.arg("-DCMAKE_BUILD_TYPE=RelWithDebInfo")
           .arg(format!("-DBUILD_SHARED_LIBS=OFF"))
           .arg(format!("-DCMAKE_INSTALL_PREFIX={}", dst.display()))
           .arg(format!("-DCMAKE_C_FLAGS={}", cflags)));
    run(Command::new("cmake")
                .arg("--build").arg(".")
                .arg("--target").arg("install")
                .cwd(&dst.join("build")));

    println!("cargo:rustc-flags=-L {} -l lua:static",
             dst.join("lib").display());
    println!("cargo:root={}", dst.display());
    //if mingw || target.contains("windows") {
    //    println!("cargo:rustc-flags=-l winhttp -l rpcrt4 -l ole32 \
    //                                -l ws2_32 -l bcrypt -l crypt32");
    //} else if target.contains("apple") {
    //    println!("cargo:rustc-flags=-l iconv");
    //}
}

fn run(cmd: &mut Command) {
    println!("running: {}", cmd);
    assert!(cmd.stdout(InheritFd(1))
               .stderr(InheritFd(2))
               .status()
               .unwrap()
               .success());

}
