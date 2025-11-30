// TODO - add test that `__code_reload_hotreload.rs` is equal to constants::GENERATED_CODE_FILE_NAME.
#[macro_export]
macro_rules! add_runtime {
    () => {
        include!(concat!(env!("OUT_DIR"), "/__code_reload_src_hotreload.rs"));
    };
    
    ($code_dir_name:literal) => {
        include!(concat!(env!("OUT_DIR"), "/__code_reload_", $code_dir_name, "_hotreload.rs"));
    }
}

#[macro_export]
macro_rules! add_tests_runtime {
    () => {
        include!(concat!(
            env!("OUT_DIR"),
            "/__code_reload_tests_hotreload.rs"
        ));
    };
}
