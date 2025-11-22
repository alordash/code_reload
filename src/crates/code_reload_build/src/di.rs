use crate::library::{FileProcessor, ILibraryBuilder, LibraryBuilder};
use crate::{
    IFileProcessor, IImplTypeExporter, IOutputGenerator, ImplTypeExporter, ItemFnMapper,
    OutputGenerator, OutputWriter, SourceFilesProvider,
};
use code_reload_core::services::FnProcessor;
use std::cell::LazyCell;
use std::sync::Arc;

pub const SERVICES: LazyCell<ServiceCollection> = LazyCell::new(create_services);

pub struct ServiceCollection {
    pub library_builder: Arc<dyn ILibraryBuilder>,

    pub file_processor: Arc<dyn IFileProcessor>,
    pub impl_type_exporter: Arc<dyn IImplTypeExporter>,
    pub output_generator: Arc<dyn IOutputGenerator>,
}

fn create_services() -> ServiceCollection {
    let source_file_paths_provider = Arc::new(SourceFilesProvider);
    let fn_processor = Arc::new(FnProcessor);
    let item_fn_mapper = Arc::new(ItemFnMapper { fn_processor });
    let file_processor = Arc::new(FileProcessor { item_fn_mapper });

    let impl_type_exporter = Arc::new(ImplTypeExporter);
    let output_generator = Arc::new(OutputGenerator);
    let output_writer = Arc::new(OutputWriter);

    let library_builder = Arc::new(LibraryBuilder {
        source_file_paths_provider,
        file_processor: file_processor.clone(),
        impl_type_exporter: impl_type_exporter.clone(),
        output_generator: output_generator.clone(),
        output_writer,
    });

    let services = ServiceCollection {
        library_builder,
        file_processor,
        impl_type_exporter,
        output_generator,
    };
    return services;
}
