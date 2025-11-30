use std::path::PathBuf;
use syn::*;

pub struct FnData {
    pub dynamic_library_filename: String,
    pub source_fn_syntax: ItemFn,
    pub source_function_types_signature: TypeBareFn,
    pub source_function_variable_name: Ident,
    pub generated_function_vis: Visibility,
    pub generated_function_signature: Signature,
    pub generated_function_expr_call: ExprCall,
    pub library_opening_error_format: String,
    pub symbol_search_error_format: String,
}