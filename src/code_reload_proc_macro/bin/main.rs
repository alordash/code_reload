extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::*;

fn main() {
    let q: TokenStream = "invoke(a, b)".parse().unwrap();
    // println!("{q:?}");
    let function_item = syn::parse2::<ExprCall>(q).unwrap();
    println!("{:#?}", function_item.func.to_token_stream());
    let f = function_item.to_token_stream();
    println!("{f:?}");
    println!("Done");
}