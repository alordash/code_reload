use crate::macros::data::IFnDataFactory;
use crate::macros::ISyntaxFactory;
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
        let item_syntax = parse_macro_input!(proc_macro_item as Item);

        let fn_data = self.fn_data_factory.create(item_syntax);

        let result = if self.is_targeting_runtime(proc_macro_attribute) {
            self.syntax_factory.create_for_runtime(fn_data)
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
            "`hotreload` attribute accepts only '{}' as parameter (optional). Received: '{}'.",
            code_reload_core::constants::RUNTIME_TARGET_KEYWORD,
            received
        );
    }
}
