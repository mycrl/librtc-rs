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
    let repository = env::var("CARGO_PKG_REPOSITORY").unwrap();
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let output = env::var("OUT_DIR").unwrap();

    let lib_name = get_lib_name(name, true);
    let path = join(&output, &lib_name);
    if !path.exists() {
        Command::new("curl")
            .arg("-L")
            .arg("-o")
            .arg(path.to_str().unwrap())
            .arg(&format!(
                "{}/releases/download/v{}/{}",
                repository, version, lib_name
            ))
            .output()
            .unwrap();
    }

    split(&path)
}

#[cfg(target_os = "windows")]
fn link() {
    println!("cargo:rustc-link-lib=winmm");
    println!("cargo:rustc-link-lib=secur32");
    println!("cargo:rustc-link-lib=msdmo");
    println!("cargo:rustc-link-lib=dmoguids");
    println!("cargo:rustc-link-lib=wmcodecdspuuid");
}

#[cfg(target_os = "macos")]
fn link() {
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=AudioToolbox");
    println!("cargo:rustc-link-lib=framework=AudioUnit");
    println!("cargo:rustc-link-lib=framework=CoreServices");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=CoreAudio");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
}

fn main() {
    let debug = env::var("DEBUG")
        .map(|label| label == "true")
        .unwrap_or(true);
    enver::init(debug).unwrap();

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
    link();
}
