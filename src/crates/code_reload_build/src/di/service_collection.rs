use crate::executable::{ExecutableBuilder, IExecutableBuilder};
use crate::fs::{ISourceFilesProvider, SourceFilesProvider};
use crate::library::{ILibraryBuilder, LibraryBuilder};
use std::cell::LazyCell;
use std::sync::Arc;

pub const SERVICES: LazyCell<ServiceCollection> = LazyCell::new(create_services);

pub struct ServiceCollection {
    pub library_builder: Arc<dyn ILibraryBuilder>,
    pub executable_builder: Arc<dyn IExecutableBuilder>,
}

fn create_services() -> ServiceCollection {
    let source_files_provider = Arc::new(SourceFilesProvider);

    let library_builder = Arc::new(LibraryBuilder {
        source_files_provider: source_files_provider.clone(),
    });
    let executable_builder = Arc::new(ExecutableBuilder);

    let services = ServiceCollection {
        library_builder,
        executable_builder,
    };
    return services;
}
