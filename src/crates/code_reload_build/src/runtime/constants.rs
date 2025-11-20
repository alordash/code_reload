use std::cell::LazyCell;
use std::path::PathBuf;

pub const CARGO_TARGET_DIR_DEBUG: LazyCell<PathBuf> = LazyCell::new(|| {
    [
        std::env::var("CARGO_TARGET_DIR").unwrap(),
        "debug".to_string(),
    ]
    .into_iter()
    .collect()
});
