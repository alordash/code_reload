use crate::macros::data::IFnDataFactory;
use crate::macros::ISyntaxFactory;
use code_reload_core::SourceCodeId;
use proc_macro::TokenStream;
use std::sync::Arc;
use syn::*;

pub trait IMacroHandler {
    fn handle(
        &self,
        proc_macro_attribute: proc_macro::TokenStream,
        proc_macro_item: proc_macro::TokenStream,
    ) -> proc_macro::TokenStream;
}

pub struct MacroHandler {
    pub fn_data_factory: Arc<dyn IFnDataFactory>,
    pub syntax_factory: Arc<dyn ISyntaxFactory>,
}

impl IMacroHandler for MacroHandler {
    fn handle(
        &self,
        proc_macro_attribute: TokenStream,
        proc_macro_item: TokenStream,
    ) -> TokenStream {
        let call_site = proc_macro::Span::call_site();
        let relative_file_path =
            SourceCodeId::get_source_code_relative_file_path(&call_site.local_file().unwrap());
        let source_code_id =
            SourceCodeId::new(relative_file_path, call_site.line(), call_site.column());
        println!("[proc_macro] source_code_id: {source_code_id}");

        let item_syntax = parse_macro_input!(proc_macro_item as Item);

        let fn_data = self.fn_data_factory.create(item_syntax);

        let result = if self.is_targeting_runtime(proc_macro_attribute) {
            self.syntax_factory
                .create_for_runtime(fn_data, &source_code_id)
        } else {
            self.syntax_factory.create_for_standalone(fn_data)
        };

        return result.into();
    }
}

impl MacroHandler {
    fn is_targeting_runtime(&self, proc_macro_attribute: TokenStream) -> bool {
        if proc_macro_attribute.is_empty() {
            return false;
        }
        let received = proc_macro_attribute.to_string();
        if let Ok(keyword) = syn::parse::<Ident>(proc_macro_attribute) {
            if keyword == code_reload_core::constants::RUNTIME_TARGET_KEYWORD {
                return true;
            }
        }
        panic!(
            "`hotreload` attribute accepts only '{}' as argument. Received: '{}'.",
            code_reload_core::constants::RUNTIME_TARGET_KEYWORD,
            received
        );
    }
}
