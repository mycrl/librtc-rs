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

fn compiler(debug: bool) -> String {
    let target = env::var("TARGET").unwrap();
    let output = env::var("OUT_DIR").unwrap();
    let sys_source_path = env::var("SYS_SOURCE_PATH").unwrap();
    let webrtc_source_path = env::var("WEBRTC_SOURCE_PATH").unwrap();
    let lib_name = get_lib_name(debug, "batrachiatc", false);
    let mut cfgs = cc::Build::new();

    cfgs.cpp(true)
        .debug(debug)
        .static_crt(true)
        .target(&target)
        .warnings(false)
        .out_dir(output);

    cfgs.file(join(&sys_source_path, "src/base.cpp"))
        .file(join(&sys_source_path, "src/media_stream_track.cpp"))
        .file(join(&sys_source_path, "src/observer.cpp"))
        .file(join(&sys_source_path, "src/peer_connection.cpp"))
        .file(join(&sys_source_path, "src/peer_connection_config.cpp"))
        .file(join(&sys_source_path, "src/session_description.cpp"))
        .file(join(&sys_source_path, "src/ice_candidate.cpp"))
        .file(join(&sys_source_path, "src/data_channel.cpp"));

    cfgs.include(join(&webrtc_source_path, "third_party/libyuv/include"))
        .include(join(&webrtc_source_path, "third_party/abseil-cpp"))
        .include(&webrtc_source_path);

    #[cfg(target_os = "macos")]
    cfgs.define("WEBRTC_MAC", None)
        .define("WEBRTC_IOS", None)
        .define("WEBRTC_POSIX", None);

    #[cfg(target_os = "linux")]
    cfgs.define("WEBRTC_POSIX", None);

    #[cfg(target_os = "windows")]
    cfgs.define("NODEBUG", None)
        .define("_CONSOLE", None)
        .define("USE_AURA", Some("1"))
        .define("_HAS_EXCEPTIONS", Some("0"))
        .define("__STD_C", None)
        .define("_CRT_RAND_S", None)
        .define("_CRT_SECURE_NO_DEPRECATE", None)
        .define("_SCL_SECURE_NO_DEPRECATE", None)
        .define("_ATL_NO_OPENGL", None)
        .define("_WINDOWS", None)
        .define("CERT_CHAIN_PARA_HAS_EXTRA_FIELDS", None)
        .define("PSAPI_VERSION", Some("2"))
        .define("WIN32", None)
        .define("_SECURE_ATL", None)
        .define("WINUWP", None)
        .define("__WRL_NO_DEFAULT_LIB__", None)
        .define("WINAPI_FAMILY", Some("WINAPI_FAMILY_PC_APP"))
        .define("WIN10", Some("_WIN32_WINNT_WIN10"))
        .define("WIN32_LEAN_AND_MEAN", None)
        .define("NOMINMAX", None)
        .define("_UNICODE", None)
        .define("UNICODE", None)
        .define("NTDDI_VERSION", Some("NTDDI_WIN10_RS2"))
        .define("_WIN32_WINNT", Some("0x0A00"))
        .define("WINVER", Some("0x0A00"))
        .define("NVALGRIND", None)
        .define("DYNAMIC_ANNOTATIONS_ENABLED", Some("0"))
        .define("WEBRTC_ENABLE_PROTOBUF", Some("0"))
        .define("WEBRTC_INCLUDE_INTERNAL_AUDIO_DEVICE", None)
        .define("RTC_ENABLE_VP9", None)
        .define("HAVE_SCTP", None)
        .define("WEBRTC_LIBRARY_IMPL", None)
        .define("WEBRTC_NON_STATIC_TRACE_EVENT_HANDLERS", Some("0"))
        .define("WEBRTC_WIN", None)
        .define("ABSL_ALLOCATOR_NOTHROW", Some("1"))
        .define("WEBRTC_VIDEO_CAPTURE_WINRT", None);

    cfgs.compile(&lib_name);
    lib_name
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
    let output = env::var("OUT_DIR").unwrap();
    let debug = env::var("DEBUG")
        .map(|label| label == "true")
        .unwrap_or(true);
    enver::init(debug).unwrap();

    for name in [
        "WEBRTC_LIBRARY_PATH",
        "SYS_LIBRARY_PATH",
        "SYS_SOURCE_PATH",
    ] {
        println!("cargo:cargo:rerun-if-env-changed={}", name);
        if let Ok(path) = env::var(name) {
            println!("cargo:rerun-if-changed={}", path);
        }
    }

    let (webrtc_lib_path, webrtc_lib_name) = env::var("WEBRTC_LIBRARY_PATH")
        .map(|path| split(Path::new(&path)))
        .unwrap_or_else(|_| download(debug, "webrtc"));

    let is_webrtc_source = env::var("WEBRTC_SOURCE_PATH").is_ok();
    let is_sys_source = env::var("SYS_SOURCE_PATH").is_ok();

    let (sys_lib_path, sys_lib_name) = env::var("SYS_LIBRARY_PATH")
        .map(|path| split(Path::new(&path)))
        .unwrap_or_else(|_| {
            if is_webrtc_source && is_sys_source {
                (output, compiler(debug))
            } else {
                download(debug, "batrachiatc")
            }
        });

    println!("cargo:rustc-link-lib=static={}", webrtc_lib_name);
    println!("cargo:rustc-link-search=all={}", webrtc_lib_path);
    println!("cargo:rustc-link-lib=static={}", sys_lib_name);
    println!("cargo:rustc-link-search=all={}", sys_lib_path);
    link();
}
