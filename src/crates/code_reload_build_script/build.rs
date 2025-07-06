macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    p!("OUT_DIR: '{:?}'", out_dir);
}
