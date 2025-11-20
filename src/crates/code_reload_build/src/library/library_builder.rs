use crate::debug_log::log;
use crate::fs::ISourceFilePathsProvider;
use crate::library::IFileProcessor;
use std::path::PathBuf;
use std::sync::Arc;

pub trait ILibraryBuilder {
    fn build(&self);
}

pub struct LibraryBuilder {
    pub source_file_paths_provider: Arc<dyn ISourceFilePathsProvider>,
    pub file_processor: Arc<dyn IFileProcessor>,
}

impl ILibraryBuilder for LibraryBuilder {
    fn build(&self) {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let src_dir: PathBuf = [&manifest_dir, "src"].iter().collect();
        let rust_file_paths = self.source_file_paths_provider.provide(&src_dir);
        let all_fn_syntaxes: Vec<_> = rust_file_paths
            .iter()
            .flat_map(|rust_file_path| self.file_processor.process(rust_file_path))
            .collect();
        for rust_file_path in rust_file_paths {
            log!("RUST FILE: '{:?}'", rust_file_path);
            self.file_processor.process(&rust_file_path);
        }
    }
}
