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

    let call_site = proc_macro::Span::call_site();
    let source_code_id = SourceCodeId::new(
        &call_site.local_file().unwrap(),
        call_site.line(),
        call_site.column(),
    );
    println!("[proc_macro] source_code_id: {source_code_id}");

    let result = macro_handler.handle(proc_macro_attribute, proc_macro_item);

    return result;
}
