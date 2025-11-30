// #![feature(proc_macro_span)]

use crate::di::SERVICES;

mod di;
mod macros;

#[proc_macro_attribute]
pub fn hotreload(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let macro_handler = &SERVICES.macro_handler;

    

    macro_handler.handle(proc_macro_attribute, proc_macro_item)
}
