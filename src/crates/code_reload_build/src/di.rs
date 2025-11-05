use crate::executable::{ExecutableBuilder, IExecutableBuilder};
use crate::fs::SourceFilesProvider;
use crate::library::{FileProcessor, FnSyntaxExtractor, ILibraryBuilder, LibraryBuilder};
use crate::{IFileProcessor, IOutputGenerator, ItemFnMapper, OutputGenerator};
use code_reload_core::services::FnProcessor;
use std::cell::LazyCell;
use std::sync::Arc;

pub const SERVICES: LazyCell<ServiceCollection> = LazyCell::new(create_services);

pub struct ServiceCollection {
    pub library_builder: Arc<dyn ILibraryBuilder>,
    pub executable_builder: Arc<dyn IExecutableBuilder>,

    pub file_processor: Arc<dyn IFileProcessor>,
    pub output_generator: Arc<dyn IOutputGenerator>
}

fn create_services() -> ServiceCollection {
    let source_file_paths_provider = Arc::new(SourceFilesProvider);
    let fn_processor = Arc::new(FnProcessor);
    let item_fn_mapper = Arc::new(ItemFnMapper { fn_processor });
    let fn_syntax_extractor = Arc::new(FnSyntaxExtractor { item_fn_mapper });
    let file_processor = Arc::new(FileProcessor {
        fn_syntax_extractor,
    });
    
    let output_generator = Arc::new(OutputGenerator);

    let library_builder = Arc::new(LibraryBuilder {
        source_file_paths_provider,
        file_processor: file_processor.clone(),
        output_generator: output_generator.clone(),
    });
    let executable_builder = Arc::new(ExecutableBuilder);

    let services = ServiceCollection {
        library_builder,
        executable_builder,
        file_processor,
        output_generator,
    };
    return services;
}
