#[macro_export]
macro_rules! define_global_hotreload_variable {
    ($hotreload_payload_type:ty) => {
        pub static HOTRELOAD: code_reload::LockedHotreloadLibrary<$hotreload_payload_type> =
            std::sync::LazyLock::new(|| {
                code_reload::HotreloadLibrary::load_locked(std::env!("CARGO_PKG_NAME"))
            });
    };
}

pub use define_global_hotreload_variable;