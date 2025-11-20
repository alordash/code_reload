use std::path::PathBuf;

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_dir: PathBuf = [&manifest_dir, "src"].iter().collect();
    let rust_files = code_reload_build::get_all_rust_file_paths(&src_dir);
    for rust_file in rust_files {
        p!("'{:?}'", rust_file);
    }
    
    let out_dir = std::env::var("OUT_DIR").unwrap();
    p!("OUT_DIR: '{:?}'", out_dir);
    
    for (key, value) in std::env::vars() {
        p!("'{}' = {}", key, value);
    }
}