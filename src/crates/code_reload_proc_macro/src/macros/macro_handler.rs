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
    pub syntax_factory: Arc<dyn ISyntaxFactory>
}

impl IMacroHandler for MacroHandler {
    fn handle(
        &self,
        _proc_macro_attribute: TokenStream,
        proc_macro_item: TokenStream,
    ) -> TokenStream {
        let item_syntax = parse_macro_input!(proc_macro_item as Item);

        let fn_data = self.fn_data_factory.create(item_syntax);

        // let result = self.syntax_factory.create_for_standalone(fn_data);
        let result = self.syntax_factory.create_for_runtime(fn_data);

        return result.into();
    }
}
