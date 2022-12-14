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

fn get_lib_name(debug: bool, key: &str, long: bool) -> String {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let ext = if cfg!(windows) { "lib" } else { "a" };
    let flag = if cfg!(windows) { "" } else { "lib" };
    let kind = if cfg!(windows) || !debug {
        "release"
    } else {
        "debug"
    };

    let name = format!("{}-{}-{}-{}", key, os, arch, kind);
    if long {
        format!("{}{}.{}", flag, name, ext)
    } else {
        name
    }
}

fn download(debug: bool, name: &str) -> (String, String) {
    let repository = env::var("CARGO_PKG_REPOSITORY").unwrap();
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let output = env::var("OUT_DIR").unwrap();

    let lib_name = get_lib_name(debug, name, true);
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

fn main() {
    let debug = env::var("DEBUG")
        .map(|label| label == "true")
        .unwrap_or(true);
    enver::init(debug).unwrap();

    for name in ["YUV_LIBRARY_PATH"] {
        println!("cargo:cargo:rerun-if-env-changed={}", name);
        if let Ok(path) = env::var(name) {
            println!("cargo:rerun-if-changed={}", path);
        }
    }
    
    let (yuv_lib_path, yuv_lib_name) = env::var("YUV_LIBRARY_PATH")
        .map(|path| split(Path::new(&path)))
        .unwrap_or_else(|_| download(debug, "yuv"));
    
    println!("cargo:rustc-link-lib={}", yuv_lib_name);
    println!("cargo:rustc-link-search=all={}", yuv_lib_path);
}
