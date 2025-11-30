use std::convert::Into;
use std::path::PathBuf;
use std::sync::LazyLock;

pub const RUNTIME_TARGET_KEYWORD: &str = "runtime";

pub const GENERATED_STATIC_HOTRELOAD_VARIABLE_NAME: &str = "HOTRELOAD";
// TODO - add test to verify that `GENERATED_CODE_FILE_NAME` starts with `GENERATED_CODE_PREFIX`
pub const GENERATED_CODE_PREFIX: &str = "__code_reload";
pub const GENERATED_CODE_FILE_NAME_SUFFIX: &str = "hotreload.rs";

pub const SRC_DIR: &str = "src";
pub const TESTS_DIR: &str = "tests";

pub const MANIFEST_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| std::env::var("CARGO_MANIFEST_DIR").unwrap().into());

pub const OUTPUT_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| std::env::var("OUT_DIR").unwrap().into());
pub const IMPL_BLOCK_TYPES_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| OUTPUT_DIR.join("impl_block_types"));
