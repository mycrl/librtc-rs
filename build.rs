use std::{env, fs, path::Path, process::Command};

use anyhow::{anyhow, Result};
use dotenv::dotenv;

fn join(root: &str, next: &str) -> String {
    Path::new(root).join(next).to_str().unwrap().to_string()
}

fn is_exsit(dir: &str) -> bool {
    fs::metadata(dir).is_ok()
}

#[cfg(target_os = "windows")]
fn exec(cmd: &str, work_dir: &str) -> Result<String> {
    let output = Command::new("powershell")
        .args([
            "-command",
            &format!("$ProgressPreference = 'SilentlyContinue'; {}", cmd),
        ])
        .current_dir(work_dir)
        .output()?;
    if !output.status.success() {
        return Err(anyhow!("{}", String::from_utf8(output.stderr)?));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[cfg(not(target_os = "windows"))]
fn exec(command: &str, work_dir: &str) -> std::io::Result<()> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .current_dir(work_dir)
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("{}", String::from_utf8(output.stderr)?));
    }

    Ok(String::from_utf8(output.stdout)?)
}

fn build_main() -> Result<()> {
    let is_debug = env::var("DEBUG")
        .map(|label| label == "true")
        .unwrap_or(true);
    let profile = if is_debug { "Debug" } else { "Release" };

    let output_dir = env::var("OUT_DIR")?;
    let librtc_dir = join(&output_dir, "./librtc");
    let third_party_dir = join(&librtc_dir, "./third_party");
    let depot_tools_dir = join(&third_party_dir, "./depot_tools");
    let webrtc_dir = join(&third_party_dir, "./webrtc");
    let webrtc_src_dir = join(&webrtc_dir, "./src");
    let webrtc_profile_dir = join(&webrtc_dir, &format!("./src/out/{}", &profile));
    
    // Clone librtc source.
    if !is_exsit(&librtc_dir) {
        exec("git clone https://github.com/mycrl/librtc", &output_dir)?;
    }

    // Used to download or initialize third-party dependent libraries.
    if !is_exsit(&third_party_dir) {
        fs::create_dir(&third_party_dir)?;
    }

    // Clone the depot_tools repository.
    if !is_exsit(&depot_tools_dir) {
        exec(
            "git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git",
            &third_party_dir,
        )?;
    }

    // Compile with native toolchain (only Windows).
    #[cfg(target_os = "windows")]
    env::set_var("DEPOT_TOOLS_WIN_TOOLCHAIN", "0");

    // On Windows operating systems, the PATH environment variable uses a
    // semicolon ; to separate paths. On Linux and macOS operating systems,
    // use a colon : to separate paths.
    env::set_var(
        "PATH",
        format!(
            "{}{}{}",
            &depot_tools_dir,
            if cfg!(target_os = "windows") {
                ";"
            } else {
                ":"
            },
            env::var("PATH")?
        ),
    );

    // Sync webrtc toolchain and many dependencies. The checkout size is large due
    // the use of the Chromium build toolchain and many dependencies.
    if !is_exsit(&webrtc_dir) {
        fs::create_dir(&webrtc_dir)?;
        exec(
            &[
                "fetch --nohooks webrtc",
                "cd src",
                "git checkout branch-heads/5735",
                "gclient sync",
            ]
            .join(if cfg!(target_os = "windows") {
                "; "
            } else {
                " && "
            }),
            &webrtc_dir,
        )?;
    }

    // Compile the Debug/Release target of the webrtc static library.
    if !is_exsit(&webrtc_profile_dir) {
        exec(
            &[
                format!(
                    "gn gen out/{} --args='is_debug={} \
                        is_component_build=false \
                        use_lld=false \
                        treat_warnings_as_errors=false \
                        use_rtti=true \
                        rtc_include_tests=false \
                        rtc_build_examples=false \
                        use_custom_libcxx=false \
                        rtc_use_h264=false'",
                    profile,
                    if is_debug { "true" } else { "false" }
                ),
                format!("ninja -C out/{}", profile),
            ]
            .join(if cfg!(target_os = "windows") {
                "; "
            } else {
                " && "
            }),
            &webrtc_src_dir,
        )?;
    }

    // In addition to windows, other platforms use package management
    // to install ffmpeg.
    let mut ffmpeg_prefix = "".to_string();

    #[cfg(target_os = "macos")]
    {
        ffmpeg_prefix = exec("brew --prefix ffmpeg", &output_dir)?;
    }

    #[cfg(target_os = "windows")]
    {
        ffmpeg_prefix = join(&output_dir, "./ffmpeg-5.1.2-full_build-shared");
        if !is_exsit(&webrtc_profile_dir) {
            exec(
                "Invoke-WebRequest \
                    -Uri https://www.gyan.dev/ffmpeg/builds/packages/ffmpeg-5.1.2-full_build-shared.7z \
                    -OutFile ffmpeg.7z; \
                7z x ffmpeg.7z -aoa; \
                del ffmpeg.7z", 
            &third_party_dir
            )?;
        }
    }

    #[cfg(target_os = "linux")]
    {
        ffmpeg_prefix = env::var("FFMPEG_PREFIX")
            .expect("On linux, you need to manually specify the prefix of ffmpeg!");
    }

    let target = env::var("TARGET")?;
    let mut compiler = cc::Build::new();
    compiler
        .cpp(true)
        .debug(is_debug)
        .static_crt(true)
        .target(&target)
        .warnings(false)
        .out_dir(&output_dir);

    if cfg!(target_os = "windows") {
        compiler.flag("/std:c++20");
    } else {
        compiler.flag("-std=c++20");
    }

    for source in [
        "thread.cpp",
        "base.cpp",
        "frame.cpp",
        "media_stream_track.cpp",
        "video_track.cpp",
        "audio_track.cpp",
        "audio_capture_module.cpp",
        "observer.cpp",
        "peer_connection.cpp",
        "peer_connection_config.cpp",
        "session_description.cpp",
        "ice_candidate.cpp",
        "data_channel.cpp",
        "video_encoder.cpp",
        "video_decoder.cpp",
        "h264.cpp",
        "h264_encoder.cpp",
        "h264_decoder.cpp",
    ] {
        compiler.file(&join(&librtc_dir, source));
    }

    #[cfg(target_os = "windows")]
    for item in [
        "_CONSOLE",
        "USE_AURA=1",
        "_HAS_EXCEPTIONS=0",
        "__STD_C",
        "_CRT_RAND_S",
        "_CRT_SECURE_NO_DEPRECATE",
        "_SCL_SECURE_NO_DEPRECATE",
        "_ATL_NO_OPENGL",
        "_WINDOWS",
        "CERT_CHAIN_PARA_HAS_EXTRA_FIELDS",
        "PSAPI_VERSION=2",
        "WIN32",
        "_SECURE_ATL",
        "WINUWP",
        "__WRL_NO_DEFAULT_LIB__",
        "WINAPI_FAMILY=WINAPI_FAMILY_PC_APP",
        "WIN10=_WIN32_WINNT_WIN10",
        "WIN32_LEAN_AND_MEAN",
        "NOMINMAX",
        "_UNICODE",
        "UNICODE",
        "NTDDI_VERSION=NTDDI_WIN10_RS2",
        "_WIN32_WINNT=0x0A00",
        "WINVER=0x0A00",
        "NVALGRIND",
        "DYNAMIC_ANNOTATIONS_ENABLED=0",
        "WEBRTC_ENABLE_PROTOBUF=0",
        "WEBRTC_INCLUDE_INTERNAL_AUDIO_DEVICE",
        "RTC_ENABLE_VP9",
        "HAVE_SCTP",
        "WEBRTC_LIBRARY_IMPL",
        "WEBRTC_NON_STATIC_TRACE_EVENT_HANDLERS=0",
        "WEBRTC_WIN",
        "ABSL_ALLOCATOR_NOTHROW=1",
        "WEBRTC_VIDEO_CAPTURE_WINRT",
        "_HAS_ITERATOR_DEBUGGING=0",
    ] {
        let mut iter = item.split("=");
        compiler.define(iter.next().unwrap(), iter.next());
    }

    #[cfg(target_os = "macos")]
    for item in ["MACOS", "WEBRTC_MAC", "WEBRTC_IOS", "WEBRTC_POSIX"] {
        let mut iter = item.split("=");
        compiler.define(iter.next().unwrap(), iter.next());
    }

    compiler
        .include(join(&webrtc_src_dir, "./third_party/libyuv/include"))
        .include(join(&webrtc_src_dir, "./third_party/abseil-cpp"))
        .include(&webrtc_src_dir)
        .include(join(&ffmpeg_prefix, "./include"))
        .compile("rtc");

    println!(
        "cargo:rustc-link-search=all={}",
        join(&webrtc_profile_dir, "./obj")
    );
    println!(
        "cargo:rustc-link-search=all={}",
        join(&ffmpeg_prefix, "./lib")
    );

    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=static=webrtc");
    println!("cargo:rustc-link-lib=static=rtc");

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=winmm");
        println!("cargo:rustc-link-lib=secur32");
        println!("cargo:rustc-link-lib=msdmo");
        println!("cargo:rustc-link-lib=dmoguids");
        println!("cargo:rustc-link-lib=wmcodecdspuuid");
        println!("cargo:rustc-link-lib=iphlpapi");
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

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=stdc++");
    }

    Ok(())
}

fn main() {
    dotenv().unwrap();
    build_main().unwrap();
}
