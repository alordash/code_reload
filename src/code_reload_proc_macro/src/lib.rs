// #![feature(proc_macro_span)]

use crate::di::SERVICES;
use code_reload_core::{constants, SourceCodeId};
use std::collections::HashSet;

mod di;
mod macros;

#[proc_macro_attribute]
pub fn hotreload(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let macro_handler = &SERVICES.macro_handler;

    let result = macro_handler.handle(proc_macro_attribute, proc_macro_item);

    return result;
}
