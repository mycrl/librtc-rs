use dotenv::dotenv;

fn main() {
    dotenv().unwrap();

    println!("cargo:rustc-link-lib=dylib={}", "rtc");
    if let Ok(path) = std::env::var("LIBRTC_PATH") {
        println!("cargo:rustc-link-search=all={}", path);
    }
}
