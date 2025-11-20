// TODO - add test that `__code_reload_hotreload.rs` is equal to constants::GENERATED_CODE_FILE_NAME.
#[macro_export]
macro_rules! add_runtime {
    () => {
        include!(concat!(env!("OUT_DIR"), "/__code_reload_hotreload.rs"));
    };
}

pub use add_runtime;