#[cfg(target_os = "windows")]
use std::{env, path::Path, process::Command};

#[cfg(target_os = "windows")]
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap();
    let profile = env::var("PROFILE")
        .unwrap()
        .replace('r', "R")
        .replace('d', "D");
    let vs_home = env::var("VS_HOME")
        .unwrap_or("C:/Program Files (x86)/Microsoft Visual Studio/2019/Community".into());
    Command::new(format!("{}/Common7/IDE/devenv.exe", vs_home))
        .arg("./ffi/ffi.sln")
        .arg("/Build")
        .arg(format!("{}|x64", profile))
        .output()
        .unwrap();
    println!("cargo:rustc-link-search=all={}", target_dir);
}

#[cfg(target_os = "macos")]
fn main() {
    println!("warn: not support the macos system!")
}

#[cfg(target_os = "linux")]
fn main() {
    println!("warn: not support the linux system!")
}
