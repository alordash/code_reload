pub const RUNTIME_TARGET_KEYWORD: &'static str = "runtime";

pub const GENERATED_STATIC_HOTRELOAD_VARIABLE_NAME: &'static str = "HOTRELOAD";
// TODO - add test to verify that `GENERATED_CODE_FILE_NAME` starts with `GENERATED_CODE_PREFIX`
pub const GENERATED_CODE_PREFIX: &'static str = "__code_reload";
pub const GENERATED_CODE_FILE_NAME: &'static str = "__code_reload_hotreload.rs";
