use proc_macro2::Span;
use quote::{format_ident, ToTokens};
use std::cell::LazyCell;
use std::str::FromStr;
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Paren, Pound};
use syn::*;

pub trait IFnProcessor {
    fn get_bare_function_signature(&self, signature: &Signature) -> TypeBareFn;

    fn mangle_function_name(&self, fn_syntax: &mut ItemFn);
    fn set_inherited_visibility(&self, fn_syntax: &mut ItemFn);
    fn get_function_variable_name(&self, fn_syntax: &ItemFn) -> Ident;
    fn get_call_expr(
        &self,
        function_variable_name: &Ident,
        args: &Punctuated<FnArg, Token![,]>,
    ) -> ExprCall;

    fn add_export_name_attribute(&self, fn_syntax: &mut ItemFn);
    fn add_doc_hidden_attribute(&self, fn_syntax: &mut ItemFn);
}

pub struct FnProcessor;

impl IFnProcessor for FnProcessor {
    fn get_bare_function_signature(&self, signature: &Signature) -> TypeBareFn {
        // TODO - create test cases using this model
        let bare_fn = TypeBareFn {
            lifetimes: None,
            unsafety: signature.unsafety,
            abi: signature.abi.clone(),
            fn_token: signature.fn_token,
            paren_token: signature.paren_token,
            inputs: signature
                .inputs
                .iter()
                .map(|fn_arg| match fn_arg {
                    FnArg::Receiver(r) => BareFnArg {
                        attrs: r.attrs.clone(),
                        name: None,
                        ty: *r.ty.clone(),
                    },
                    FnArg::Typed(t) => BareFnArg {
                        attrs: t.attrs.clone(),
                        name: None,
                        ty: *t.ty.clone(),
                    },
                })
                .collect(),
            variadic: signature.variadic.clone().map(|variadic| BareVariadic {
                attrs: variadic.attrs,
                name: None,
                dots: variadic.dots,
                comma: variadic.comma,
            }),
            output: signature.output.clone(),
        };

        return bare_fn;
    }

    fn mangle_function_name(&self, fn_syntax: &mut ItemFn) {
        fn_syntax.sig.ident =
            format_ident!("{}_{}", *Self::NAME_MANGLE_PREFIX, fn_syntax.sig.ident);
    }

    fn set_inherited_visibility(&self, fn_syntax: &mut ItemFn) {
        fn_syntax.vis = Visibility::Inherited;
    }

    fn get_function_variable_name(&self, fn_syntax: &ItemFn) -> Ident {
        let function_variable_name = format_ident!(
            "{}_{}",
            *Self::FUNCTION_VARIABLE_NAME_PREFIX,
            fn_syntax.sig.ident
        );

        return function_variable_name;
    }

    fn get_call_expr(
        &self,
        function_variable_name: &Ident,
        args: &Punctuated<FnArg, Token![,]>,
    ) -> ExprCall {
        let expr_call = ExprCall {
            attrs: Vec::new(),
            func: Box::new(Expr::Verbatim(function_variable_name.to_token_stream())),
            paren_token: Paren::default(),
            args: args
                .iter()
                .map(|arg| match arg {
                    FnArg::Receiver(receiver) => receiver.self_token.to_token_stream(),
                    FnArg::Typed(typed) => typed.pat.to_token_stream(),
                })
                .map(Expr::Verbatim)
                .collect(),
        };

        return expr_call;
    }

    fn add_export_name_attribute(&self, fn_syntax: &mut ItemFn) {
        fn_syntax.attrs.push(Attribute {
            pound_token: Pound::default(),
            style: AttrStyle::Outer,
            bracket_token: Bracket::default(),
            meta: Meta::List(MetaList {
                path: Path {
                    leading_colon: None,
                    segments: vec![PathSegment::from(format_ident!("unsafe"))]
                        .into_iter()
                        .collect(),
                },
                delimiter: MacroDelimiter::Paren(Paren::default()),
                tokens: proc_macro2::TokenStream::from_str(&format!(
                    "export_name = \"{}\"",
                    fn_syntax.sig.ident
                ))
                .unwrap(),
            }),
        });
    }

    fn add_doc_hidden_attribute(&self, fn_syntax: &mut ItemFn) {
        fn_syntax.attrs.push(Attribute {
            pound_token: Pound::default(),
            style: AttrStyle::Outer,
            bracket_token: Bracket::default(),
            meta: Meta::List(MetaList {
                path: Path {
                    leading_colon: None,
                    segments: vec![PathSegment::from(format_ident!("doc"))]
                        .into_iter()
                        .collect(),
                },
                delimiter: MacroDelimiter::Paren(Paren::default()),
                tokens: proc_macro2::TokenStream::from_str("hidden").unwrap(),
            }),
        });
    }
}

impl FnProcessor {
    const NAME_MANGLE_PREFIX: LazyCell<Ident> =
        LazyCell::new(|| Ident::new("__code_reload", Span::call_site()));
    const FUNCTION_VARIABLE_NAME_PREFIX: LazyCell<Ident> =
        LazyCell::new(|| Ident::new("library", Span::call_site()));
}
