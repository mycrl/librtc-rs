use std::fs::File;
use std::env;
use std::io::{
    Result,
    BufRead,
    BufReader,
};

pub fn init(is_debug: bool) -> Result<()> {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let kind = if is_debug { "develop" } else { "release" };
    if let Ok(fs) = File::open(format!("{}/{}.env", root_dir, kind)) {
        let mut buf = String::with_capacity(1024);
        let mut reader = BufReader::new(fs);

        while let Ok(size) = reader.read_line(&mut buf) {
            if size == 0 {
                break;
            }

            if let Some((key, value)) = &buf[..size].rsplit_once('=') {
                env::set_var(key, value);
                buf.clear();
            }
        }
    }

    Ok(())
}
