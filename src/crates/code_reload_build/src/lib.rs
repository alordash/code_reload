pub use crate::di::SERVICES;

mod di;
mod fs;

mod executable;
mod library;
mod add_runtime_macro;
mod debug_log;

pub mod runtime;

pub use library::*;

pub fn build() {
    let library_builder = &SERVICES.library_builder;
    library_builder.build();
}
