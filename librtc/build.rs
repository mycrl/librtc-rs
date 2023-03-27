fn main() {
    println!("cargo:rustc-link-lib=dylib={}", "rtc");
    if let Ok(prefix) = std::env::var("LIBRTC_PREFIX") {
        println!("cargo:rustc-link-search=dependency={}", prefix);
    }
}
