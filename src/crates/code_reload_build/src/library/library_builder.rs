use crate::debug_log::log;
use crate::fs::ISourceFilePathsProvider;
use crate::library::IFileProcessor;
use crate::{IOutputGenerator, IOutputWriter};
use code_reload_core::constants;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::library::impl_type_exporter::IImplTypeExporter;

pub trait ILibraryBuilder {
    fn build(&self);
    fn build_tests(&self);
    fn build_dir(&self, code_dir_name: &str);
}

pub struct LibraryBuilder {
    pub source_file_paths_provider: Arc<dyn ISourceFilePathsProvider>,
    pub file_processor: Arc<dyn IFileProcessor>,
    pub impl_type_exporter: Arc<dyn IImplTypeExporter>,
    pub output_generator: Arc<dyn IOutputGenerator>,
    pub output_writer: Arc<dyn IOutputWriter>,
}

impl ILibraryBuilder for LibraryBuilder {
    fn build(&self) {
        self.build_core(code_reload_core::constants::SRC_DIR);
    }

    fn build_tests(&self) {
        self.build_core(code_reload_core::constants::TESTS_DIR);
    }

    fn build_dir(&self, code_dir_name: &str) {
        self.build_core(code_dir_name);
    }
}

impl LibraryBuilder {
    fn build_core(&self, code_dir_name: &str) {
        for var in std::env::vars() {
            log!("'{}' = '{}'", var.0, var.1);
        }
        
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let code_dir = Path::new(&manifest_dir).join(code_dir_name);
        let rust_file_paths = self.source_file_paths_provider.provide(&code_dir);
        let all_build_fn_datas: Vec<_> = rust_file_paths
            .iter()
            .flat_map(|rust_file_path| self.file_processor.process(rust_file_path))
            .collect();
        // self.impl_type_exporter.export(&all_build_fn_datas);
        let output = self.output_generator.generate(all_build_fn_datas);
        self.output_writer.write(code_dir_name, &output).unwrap();

        log!("done, output length: {}", output.len());
    }
}
