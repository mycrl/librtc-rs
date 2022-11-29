use std::env;

fn main() {
    let is_debug = env::var("DEBUG")
        .map(|label| label == "true")
        .unwrap_or(true);
    let target = env::var("TARGET").unwrap();

    println!("cargo:cargo:rerun-if-env-changed=WEBRTC_SOURCE_PATH");
    println!("cargo:cargo:rerun-if-env-changed=WEBRTC_LIBRARY_PATH");
    println!("cargo:cargo:rerun-if-env-changed=SYS_SOURCE_PATH");
    println!("cargo:cargo:rerun-if-env-changed=SYS_LIBRARY_PATH");
    println!("cargo:rerun-if-changed=batrachiatc");
    
    let mut cfgs = cc::Build::new();
    
    cfgs.cpp(true)
        .debug(is_debug)
        .static_crt(true)
        .target(&target)
        .warnings(false)
        .out_dir(&format!("../target/{}", match is_debug {
            true => "debug",
            false => "release",
        }));
        
    cfgs.file("../batrachiatc/src/base.cpp")
        .file("../batrachiatc/src/media_stream_track.cpp")
        .file("../batrachiatc/src/observer.cpp")
        .file("../batrachiatc/src/peer_connection.cpp")
        .file("../batrachiatc/src/peer_connection_config.cpp")
        .file("../batrachiatc/src/session_description.cpp")
        .file("../batrachiatc/src/ice_candidate.cpp")
        .file("../batrachiatc/src/data_channel.cpp");
    
    cfgs.include("../batrachiatc/third_party/webrtc/src/third_party/libyuv/include")
        .include("../batrachiatc/third_party/webrtc/src/third_party/abseil-cpp")
        .include("../batrachiatc/third_party/webrtc/src");
    
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
    
    #[cfg(target_os = "macos")]
    cfgs.define("WEBRTC_MAC", None)
        .define("WEBRTC_IOS", None)
        .define("WEBRTC_POSIX", None);
    
    #[cfg(target_os = "linux")]
    cfgs.define("WEBRTC_POSIX", None);
    
    println!(
        "cargo:rustc-link-search=all=target/{}",
        match is_debug {
            true => "debug",
            false => "release",
        }
    );

    #[cfg(not(target_os = "windows"))]
    println!(
        "cargo:rustc-link-search=all=batrachiatc/third_party/webrtc/src/out/\
        {}/obj",
        match is_debug {
            true => "Debug",
            false => "Release",
        }
    );
    
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-search=all=batrachiatc/third_party/webrtc/src/\
        out/Release/obj");
    
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=winmm");
        println!("cargo:rustc-link-lib=secur32");
        println!("cargo:rustc-link-lib=msdmo");
        println!("cargo:rustc-link-lib=dmoguids");
        println!("cargo:rustc-link-lib=wmcodecdspuuid");
    }
    
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=c++");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
        println!("cargo:rustc-link-lib=framework=AudioUnit");
        println!("cargo:rustc-link-lib=framework=CoreServices");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreAudio");  
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
    }
    
    println!("cargo:rustc-link-lib=static=batrachiatc");
    println!("cargo:rustc-link-lib=webrtc");
    
    cfgs.compile("batrachiatc");
}
