use crate::macros::data::FnData;
use crate::macros::{IErrorFormatter, IFnValidator, IMetadataProcessor};
use code_reload_core::services::IFnProcessor;
use code_reload_core::SourceCodeId;
use std::sync::Arc;
use syn::*;

pub trait IFnDataFactory {
    fn create(&self, item_syntax: Item, source_code_id: &SourceCodeId) -> FnData;
}

pub struct FnDataFactory {
    pub metadata_processor: Arc<dyn IMetadataProcessor>,
    pub fn_validator: Arc<dyn IFnValidator>,
    pub fn_processor: Arc<dyn IFnProcessor>,
    pub error_formatter: Arc<dyn IErrorFormatter>,
}

impl IFnDataFactory for FnDataFactory {
    fn create(&self, item_syntax: Item, source_code_id: &SourceCodeId) -> FnData {
        let mut source_fn_syntax = match item_syntax {
            Item::Fn(fn_syntax) => fn_syntax,
            _ => {
                panic!("Unsupported item: this attribute can be applied only to functions.");
            }
        };
        self.fn_validator.validate(&source_fn_syntax);
        let dynamic_library_filename = self.metadata_processor.get_dynamic_library_filename();

        let generated_function_vis = source_fn_syntax.vis.clone();
        let generated_function_signature = source_fn_syntax.sig.clone();

        let source_function_types_signature = self
            .fn_processor
            .get_bare_function_signature(&source_fn_syntax.sig);
        let source_function_variable_name = self
            .fn_processor
            .get_function_variable_name(&source_fn_syntax);

        self.fn_processor
            .mangle_function_name(&mut source_fn_syntax, source_code_id);
        self.fn_processor
            .set_inherited_visibility(&mut source_fn_syntax);
        self.fn_processor
            .add_export_name_attribute(&mut source_fn_syntax);
        self.fn_processor
            .add_doc_hidden_attribute(&mut source_fn_syntax);

        let generated_function_expr_call = self
            .fn_processor
            .get_call_expr(&source_function_variable_name, &source_fn_syntax.sig.inputs);
        let library_opening_error_format = self
            .error_formatter
            .get_library_opening_error_format(&dynamic_library_filename);
        let symbol_search_error_format = self
            .error_formatter
            .get_symbol_search_error_format(&dynamic_library_filename, &source_fn_syntax.sig.ident);

        

        FnData {
            dynamic_library_filename,
            source_fn_syntax,
            source_function_types_signature,
            source_function_variable_name,
            generated_function_vis,
            generated_function_signature,
            generated_function_expr_call,
            library_opening_error_format,
            symbol_search_error_format,
        }
    }
}
