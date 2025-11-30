#[allow(unused)]
macro_rules! log {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}

pub(crate) use log;