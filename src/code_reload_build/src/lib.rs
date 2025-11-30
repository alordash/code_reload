pub use crate::di::SERVICES;

mod di;

mod add_runtime_macros;
mod debug_log;
mod executable;
mod library;

pub mod runtime;

pub use library::*;

pub fn build() {
    let library_builder = &SERVICES.library_builder;
    library_builder.build();
}

pub fn build_tests() {
    let library_builder = &SERVICES.library_builder;
    library_builder.build_tests();
}

pub fn build_dir(code_dir_name: &str) {
    let library_builder = &SERVICES.library_builder;
    library_builder.build_dir(code_dir_name);
}
