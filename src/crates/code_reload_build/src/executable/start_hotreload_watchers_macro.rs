// TODO - add test that verifies that "__code_reload" and "HOTRELOAD" are same as in code_reload::constants.
#[macro_export]
macro_rules! start_hotreload_watchers {
    () => {};

    ( $crate_name:ident ) => {
        code_reload::runtime::start_hotreload_watch(& $crate_name :: __code_reload :: HOTRELOAD);
    };

    ( $head:ident, $($tail:ident),+ ) => {
        code_reload::runtime::start_hotreload_watchers!($head);
        code_reload::runtime::start_hotreload_watchers!($($tail),+)
    };
}

pub use start_hotreload_watchers;
