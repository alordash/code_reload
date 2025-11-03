use std::path::PathBuf;
use crate::fs::ISourceFilesProvider;
use std::sync::Arc;
use crate::debug_log::p;

pub trait ILibraryBuilder {
    fn build(&self);
}

pub struct LibraryBuilder {
    pub source_files_provider: Arc<dyn ISourceFilesProvider>,
}

impl ILibraryBuilder for LibraryBuilder {
    fn build(&self) {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let src_dir: PathBuf = [&manifest_dir, "src"].iter().collect();
        let rust_files = self.source_files_provider.provide(&src_dir);
        for rust_file in rust_files {
            p!("'{:?}'", rust_file);
        }
    }
}
