pub use crate::di::SERVICES;

mod di;
mod fs;

mod executable;
mod library;

mod debug_log;

pub mod runtime;

pub fn library() {
    let library_builder = &SERVICES.library_builder;
    library_builder.build();
}

pub use library::*;
