use crate::macros::data::FnData;
use proc_macro2::{Literal, TokenStream};
use quote::quote;

pub trait ISyntaxFactory {
    fn create(&self, fn_data: FnData) -> TokenStream;
}

pub struct SyntaxFactory;

impl ISyntaxFactory for SyntaxFactory {
    fn create(&self, fn_data: FnData) -> TokenStream {
        let FnData {
            dynamic_library_path,
            source_fn_syntax,
            source_function_types_signature,
            source_function_variable_name,
            generated_function_vis,
            generated_function_signature,
            generated_function_expr_call,
            library_opening_error_format,
            symbol_search_error_format,
        } = fn_data;

        let source_function_export_name_literal =
            Literal::byte_string(source_fn_syntax.sig.ident.to_string().as_bytes());

        let dynamic_library_path_string = dynamic_library_path.to_str().unwrap();

        println!(
            "source_function_export_name_literal: {:?}",
            source_function_export_name_literal.to_string()
        );

        let result = quote! {
            #generated_function_vis #generated_function_signature {
                use libloading::Library;
                unsafe {
                    let library = Library::new(#dynamic_library_path_string)
                        .map_err(|e| format!(#library_opening_error_format))
                        .unwrap();
                    let #source_function_variable_name = library.get::<#source_function_types_signature>(#source_function_export_name_literal)
                        .map_err(|e| format!(#symbol_search_error_format))
                        .unwrap();
                    return #generated_function_expr_call;
                }
            }

            #source_fn_syntax
        };

        return result;
    }
}
