pub use code_reload_proc_macro::hotreload;

#[doc(hidden)]
pub use code_reload_core::LibraryWrapper;

#[cfg(feature = "runtime")]
pub mod runtime {
    pub use code_reload_build::runtime::{
        start_watch, HotreloadLibrary, IHotreloadPayload, LockedHotreloadLibrary,
    };
    pub use code_reload_build::start_watchers;
    pub use code_reload_build::{add_runtime, add_tests_runtime};
    pub use code_reload_build::{build, build_dir, build_tests};
}
