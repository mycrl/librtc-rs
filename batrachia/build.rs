use std::env;

fn main() {
    let is_debug = env::var("DEBUG")
        .map(|label| label == "true")
        .unwrap_or(true);

    println!("cargo:rustc-link-lib=dylib=batrachiatc");
    println!(
        "cargo:rustc-link-search=all=target/{}",
        match is_debug {
            true => "debug",
            false => "release",
        }
    );
}
