use crate::macros::data::FnData;
use crate::macros::IImplTypeImporter;
use code_reload_core::{constants, SourceCodeId};
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::sync::{Arc, LazyLock};
use syn::{parse_macro_input, Expr, FnArg, Pat, PatType};

pub trait ISyntaxFactory {
    fn create_for_standalone(&self, fn_data: FnData) -> TokenStream;
    fn create_for_runtime(&self, fn_data: FnData, source_code_id: &SourceCodeId) -> TokenStream;
}

pub struct SyntaxFactory {
    pub impl_type_importer: Arc<dyn IImplTypeImporter>,
}

impl ISyntaxFactory for SyntaxFactory {
    fn create_for_standalone(&self, fn_data: FnData) -> TokenStream {
        let FnData {
            dynamic_library_filename,
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

        // println!(
        //     "source_function_export_name_literal: {:?}",
        //     source_function_export_name_literal.to_string()
        // );

        let result = quote! {
            #generated_function_vis #generated_function_signature {
                unsafe {
                    let dynamic_library_path = std::env::current_exe().unwrap().parent().unwrap().join(#dynamic_library_filename);
                    let library = code_reload::LibraryWrapper::new(dynamic_library_path)
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

    fn create_for_runtime(&self, fn_data: FnData, source_code_id: &SourceCodeId) -> TokenStream {
        let FnData {
            source_fn_syntax,
            generated_function_vis,
            generated_function_signature,
            generated_function_expr_call,
            ..
        } = fn_data;

        let hotreload_module_ident = format_ident!("{}", constants::GENERATED_CODE_PREFIX);
        let hotreload_static_variable_ident =
            format_ident!("{}", constants::GENERATED_STATIC_HOTRELOAD_VARIABLE_NAME);
        let source_fn_ident = &source_fn_syntax.sig.ident;
        let args = generated_function_expr_call.args;

        let result = quote! {
            #generated_function_vis #generated_function_signature {
                return (crate:: #hotreload_module_ident :: #hotreload_static_variable_ident
                    .read()
                    .unwrap()
                    .payload
                    . #source_fn_ident)(#args);
            }

            #source_fn_syntax
        };

        return result;
    }
}

impl SyntaxFactory {
    const SELF_ARG_NAME: &'static str = "__code_reload_self";
    const SELF_ARG: LazyLock<Expr> = LazyLock::new(|| syn::parse_str(Self::SELF_ARG_NAME).unwrap());
}
