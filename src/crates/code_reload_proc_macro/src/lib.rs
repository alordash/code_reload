use crate::di::SERVICES;

mod di;
mod macros;

#[proc_macro_attribute]
pub fn hotreload(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let macro_handler = &SERVICES.macro_handler;
    
    let call_site = proc_macro::Span::call_site();
    println!("call_site file: {}", call_site.file());
    println!("call_site local_file: {:?}", call_site.local_file());
    println!("call_site source_text: {:?}", call_site.source_text());

    let result = macro_handler.handle(proc_macro_attribute, proc_macro_item);

    return result;
}