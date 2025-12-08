use crate::library::{FileProcessor, ILibraryBuilder, LibraryBuilder};
use crate::{
    IFileProcessor, IImplTypeExporter, IOutputGenerator, ImplTypeExporter, ItemFnMapper,
    OutputGenerator, OutputWriter, SourceFilesProvider,
};
use code_reload_core::services::FnProcessor;
use std::cell::LazyCell;
use std::rc::Rc;

pub const SERVICES: LazyCell<ServiceCollection> = LazyCell::new(create_services);

pub struct ServiceCollection {
    pub library_builder: Rc<dyn ILibraryBuilder>,

    pub file_processor: Rc<dyn IFileProcessor>,
    pub impl_type_exporter: Rc<dyn IImplTypeExporter>,
    pub output_generator: Rc<dyn IOutputGenerator>,
}

fn create_services() -> ServiceCollection {
    let source_file_paths_provider = Rc::new(SourceFilesProvider);
    let fn_processor = Rc::new(FnProcessor);
    let item_fn_mapper = Rc::new(ItemFnMapper { fn_processor });
    let file_processor = Rc::new(FileProcessor { item_fn_mapper });

    let impl_type_exporter = Rc::new(ImplTypeExporter);
    let output_generator = Rc::new(OutputGenerator);
    let output_writer = Rc::new(OutputWriter);

    let library_builder = Rc::new(LibraryBuilder {
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
