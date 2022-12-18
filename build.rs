use std::env;
use std::path::*;
use std::process::*;

fn join(a: &str, b: &str) -> PathBuf {
    Path::new(a).join(b)
}

fn split(path: &Path) -> (String, String) {
    let name = path.file_stem().unwrap().to_str().unwrap().to_string();
    let dir = path.parent().unwrap().to_str().unwrap().to_string();

    (
        dir,
        name.starts_with("lib")
            .then(|| name.replacen("lib", "", 1))
            .unwrap_or(name),
    )
}

fn get_lib_name(key: &str, long: bool) -> String {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let ext = if cfg!(windows) { "lib" } else { "a" };
    let flag = if cfg!(windows) { "" } else { "lib" };
    let name = format!("{}-{}-{}", key, os, arch);
    if long {
        format!("{}{}.{}", flag, name, ext)
    } else {
        name
    }
}

fn download(name: &str) -> (String, String) {
    let repository = "https://github.com/colourful-rtc/batrachiatc";
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let output = env::var("OUT_DIR").unwrap();

    let lib_name = get_lib_name(name, true);
    let path = join(&output, &lib_name);
    if !path.exists() {
        Command::new("curl")
            .arg("-f")
            .arg("-L")
            .arg("-o")
            .arg(path.to_str().unwrap())
            .arg(&format!(
                "{}/releases/download/v{}/{}",
                repository, version, lib_name
            ))
            .output()
            .expect("There is no precompiled binary library file in git \
                releases, please try to compile it yourself according to the \
                README, see https://github.com/colourful-rtc/batrachiatc");
    }

    split(&path)
}

fn main() {
    for name in ["WEBRTC_LIBRARY_PATH", "SYS_LIBRARY_PATH"] {
        println!("cargo:cargo:rerun-if-env-changed={}", name);
        if let Ok(path) = env::var(name) {
            println!("cargo:rerun-if-changed={}", path);
        }
    }

    let (webrtc_lib_path, webrtc_lib_name) = env::var("WEBRTC_LIBRARY_PATH")
        .map(|path| split(Path::new(&path)))
        .unwrap_or_else(|_| download("webrtc"));

    let (sys_lib_path, sys_lib_name) = env::var("SYS_LIBRARY_PATH")
        .map(|path| split(Path::new(&path)))
        .unwrap_or_else(|_| download("batrachiatc"));

    println!("cargo:rustc-link-lib=static={}", webrtc_lib_name);
    println!("cargo:rustc-link-search=all={}", webrtc_lib_path);
    println!("cargo:rustc-link-lib=static={}", sys_lib_name);
    println!("cargo:rustc-link-search=all={}", sys_lib_path);
    
    if cfg!(windows) {
        println!("cargo:rustc-link-lib=winmm");
        println!("cargo:rustc-link-lib=secur32");
        println!("cargo:rustc-link-lib=msdmo");
        println!("cargo:rustc-link-lib=dmoguids");
        println!("cargo:rustc-link-lib=wmcodecdspuuid");
    }

    if cfg!(macos) {
        println!("cargo:rustc-link-lib=c++");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
        println!("cargo:rustc-link-lib=framework=AudioUnit");
        println!("cargo:rustc-link-lib=framework=CoreServices");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreAudio");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
    }

    if cfg!(linux) {
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=X11");
    }
}
