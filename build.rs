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
        .args(["-command", cmd])
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

fn lib_name(module: &str, is_path: bool) -> String {
    if is_path {
        format!(
            "{}{}-{}-{}-release.{}",
            if cfg!(target_os = "windows") {
                ""
            } else {
                "lib"
            },
            module,
            env::var("CARGO_CFG_TARGET_OS").unwrap(),
            env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
            if cfg!(target_os = "windows") {
                "lib"
            } else {
                "a"
            },
        )
    } else {
        format!(
            "{}-{}-{}-release",
            module,
            env::var("CARGO_CFG_TARGET_OS").unwrap(),
            env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
        )
    }
}

fn main() {
    let _ = dotenv();
    println!("cargo:rerun-if-changed=./src");
    println!("cargo:rerun-if-changed=./build.rs");

    let version = "v0.1.x";
    let output_dir = env::var("OUT_DIR").unwrap();
    let temp = env::var("TEMP").unwrap();

    for lib_name in [lib_name("rtc", true), lib_name("webrtc", true)] {
        if !is_exsit(&join(&output_dir, &lib_name)) {
            exec(&format!(
                "Invoke-WebRequest -Uri https://github.com/mycrl/librtc/releases/download/{}/{} -OutFile {}",
                version,
                &lib_name,
                &lib_name,
            ), &output_dir).unwrap();
        }
    }

    // In addition to windows, other platforms use package management
    // to install ffmpeg.
    #[allow(unused)]
    let mut ffmpeg_prefix = "".to_string();

    #[cfg(target_os = "macos")]
    {
        ffmpeg_prefix = exec("brew --prefix ffmpeg", &output_dir)?;
    }

    #[cfg(target_os = "windows")]
    {
        ffmpeg_prefix = join(&output_dir, "./ffmpeg-5.1.2-full_build-shared");
       if !is_exsit(&ffmpeg_prefix) {
           exec(
               "Invoke-WebRequest \
                   -Uri https://www.gyan.dev/ffmpeg/builds/packages/ffmpeg-5.1.2-full_build-shared.7z \
                   -OutFile ffmpeg.7z; \
               7z x ffmpeg.7z -aoa; \
               del ffmpeg.7z",
               &output_dir,
           )
           .unwrap();
       }
    }

    #[cfg(target_os = "linux")]
    {
        ffmpeg_prefix = env::var("FFMPEG_PREFIX")
            .expect("On linux, you need to manually specify the prefix of ffmpeg!");
    }

    println!("cargo:rustc-link-search=all={}", &output_dir);
    println!(
        "cargo:rustc-link-search=all={}",
        join(&ffmpeg_prefix, "./lib")
    );

    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=static={}", lib_name("webrtc", false));
    println!("cargo:rustc-link-lib=static={}", lib_name("rtc", false));

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

    #[cfg(target_os = "windows")]
    {
        let temp_dir = join(&temp, &format!("librtc-{}", &version));
        if !is_exsit(&temp_dir) {
            fs::create_dir(&temp_dir).unwrap();
            exec(
                &format!("cp -r {}/bin/*.dll {}", &ffmpeg_prefix, &temp_dir),
                &output_dir,
            )
            .unwrap();
        }
    }
}
