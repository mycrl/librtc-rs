fn main() {
    println!("cargo:rustc-link-lib=dylib={}", "rtc");
    println!("cargo:rustc-link-search=all=E:/librtcrs/target/debug/examples");
}
