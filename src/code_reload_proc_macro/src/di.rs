use crate::macros::data::FnDataFactory;
use crate::macros::*;
use code_reload_core::services::FnProcessor;
use std::cell::LazyCell;
use std::rc::Rc;

pub const SERVICES: LazyCell<ServiceCollection> = LazyCell::new(create_services);

pub struct ServiceCollection {
    pub macro_handler: Rc<dyn IMacroHandler>,
}

fn create_services() -> ServiceCollection {
    let metadata_processor = Rc::new(MetadataProcessor);
    let fn_validator = Rc::new(FnValidator);
    let fn_processor = Rc::new(FnProcessor);
    let error_formatter = Rc::new(ErrorFormatter);

    let fn_data_factory = Rc::new(FnDataFactory {
        metadata_processor,
        fn_validator,
        fn_processor,
        error_formatter,
    });

    let syntax_factory = Rc::new(SyntaxFactory);

    let macro_handler = Rc::new(MacroHandler {
        fn_data_factory,
        syntax_factory,
    });

    ServiceCollection { macro_handler }
}
