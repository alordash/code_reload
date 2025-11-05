use crate::debug_log::log;
use crate::fs::ISourceFilePathsProvider;
use crate::library::IFileProcessor;
use std::path::PathBuf;
use std::sync::Arc;
use crate::{IOutputGenerator, IOutputWriter};

pub trait ILibraryBuilder {
    fn build(&self);
}

pub struct LibraryBuilder {
    pub source_file_paths_provider: Arc<dyn ISourceFilePathsProvider>,
    pub file_processor: Arc<dyn IFileProcessor>,
    pub output_generator: Arc<dyn IOutputGenerator>,
    pub output_writer: Arc<dyn IOutputWriter>,
}

impl ILibraryBuilder for LibraryBuilder {
    fn build(&self) {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let src_dir: PathBuf = [&manifest_dir, "src"].iter().collect();
        let rust_file_paths = self.source_file_paths_provider.provide(&src_dir);
        let all_build_fn_datas: Vec<_> = rust_file_paths
            .iter()
            .flat_map(|rust_file_path| self.file_processor.process(rust_file_path))
            .collect();
        let output = self.output_generator.generate(all_build_fn_datas);
        self.output_writer.write(&output).unwrap();
        
        log!("output:");
        log!("{}", output);
    }
}
