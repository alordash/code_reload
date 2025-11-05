use crate::macros::data::FnDataFactory;
use crate::macros::*;
use code_reload_core::services::FnProcessor;
use std::cell::LazyCell;
use std::sync::Arc;

pub const SERVICES: LazyCell<ServiceCollection> = LazyCell::new(create_services);

pub struct ServiceCollection {
    pub macro_handler: Arc<dyn IMacroHandler>,
}

fn create_services() -> ServiceCollection {
    let metadata_processor = Arc::new(MetadataProcessor);
    let fn_validator = Arc::new(FnValidator);
    let fn_processor = Arc::new(FnProcessor);
    let error_formatter = Arc::new(ErrorFormatter);

    let fn_data_factory = Arc::new(FnDataFactory {
        metadata_processor,
        fn_validator,
        fn_processor,
        error_formatter,
    });

    let syntax_factory = Arc::new(SyntaxFactory);

    let macro_handler = Arc::new(MacroHandler {
        fn_data_factory,
        syntax_factory,
    });
    let services = ServiceCollection { macro_handler };
    return services;
}
