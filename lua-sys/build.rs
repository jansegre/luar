#![feature(path, io, os, core)]

extern crate "pkg-config" as pkg_config;

use std::os;
use std::old_io::{self, fs, Command, File};
use std::old_io::process::InheritFd;

fn build_lua() {
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

    let src = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap()).join("lua");
    let dst = Path::new(os::getenv("OUT_DIR").unwrap());
    let build = dst.join("build");
    let _ = fs::mkdir(&build, old_io::USER_DIR);

    let mut cmd = Command::new("cmake");
    cmd.arg(src)
       .cwd(&build);
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
                .cwd(&build));

    println!("cargo:rustc-flags=-L {} -l lua:static", dst.join("lib").display());
    println!("cargo:root={}", dst.display());
}

fn gen_defs() {
    let mut cflags = os::getenv("CFLAGS").unwrap_or(String::new());
    let target = os::getenv("TARGET").unwrap();
    let mingw = target.contains("windows-gnu");

    if target.contains("i686") {
        cflags.push_str(" -m32");
    } else if target.as_slice().contains("x86_64") {
        cflags.push_str(" -m64");
    }
    if !target.contains("i686") {
        cflags.push_str(" -fPIC");
    }

    let src = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap()).join("defs");
    let dst = Path::new(os::getenv("OUT_DIR").unwrap());
    let build = dst.join("defs");
    let _ = fs::mkdir(&build, old_io::USER_DIR);

    let mut cmd = Command::new("cmake");
    cmd.arg(src)
       .cwd(&build);
    if mingw {
        cmd.arg("-G").arg("Unix Makefiles");
    }
    run(cmd.arg(format!("-DCMAKE_C_FLAGS={}", cflags))
           .arg(format!("-DLUA_INCLUDE_DIR={}", dst.join("include").display()))
           .cwd(&build));
    run(Command::new("cmake")
                .arg("--build").arg(".")
                .cwd(&build));
    let mut cmd = Command::new(build.join("rust-lua-defs"));
    let defs = Path::new(os::getenv("OUT_DIR").unwrap()).join("defs.rs");
    println!("running: {:?} > {}", cmd, defs.display());
    let out = cmd
        .stderr(InheritFd(2))
        .output()
        .unwrap();
    assert!(out.status.success());
    let mut f = File::create(&defs).unwrap();
    f.write_all(out.output.as_slice()).unwrap();
}

fn main() {
    build_lua();
    gen_defs();
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    assert!(cmd.stdout(InheritFd(1))
               .stderr(InheritFd(2))
               .status()
               .unwrap()
               .success());

}
