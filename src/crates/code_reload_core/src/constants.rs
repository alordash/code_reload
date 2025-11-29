use std::clone::Clone;
use std::convert::Into;
use std::path::PathBuf;
use std::sync::LazyLock;

pub const RUNTIME_TARGET_KEYWORD: &'static str = "runtime";

pub const GENERATED_STATIC_HOTRELOAD_VARIABLE_NAME: &'static str = "HOTRELOAD";
// TODO - add test to verify that `GENERATED_CODE_FILE_NAME` starts with `GENERATED_CODE_PREFIX`
pub const GENERATED_CODE_PREFIX: &'static str = "__code_reload";
pub const GENERATED_CODE_FILE_NAME_SUFFIX: &'static str = "hotreload.rs";

pub const SRC_DIR: &'static str = "src";
pub const TESTS_DIR: &'static str = "tests";

pub const MANIFEST_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| std::env::var("CARGO_MANIFEST_DIR").unwrap().into());

pub const OUTPUT_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| std::env::var("OUT_DIR").unwrap().into());
pub const IMPL_BLOCK_TYPES_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| OUTPUT_DIR.join("impl_block_types"));

#[cfg(target_os = "windows")]
pub const DYNAMIC_LIBRARY_EXTENSION: &'static str = "dll";
#[cfg(target_os = "macos")]
pub const DYNAMIC_LIBRARY_EXTENSION: &'static str = "dylib";
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub const DYNAMIC_LIBRARY_EXTENSION: &'static str = "so";
