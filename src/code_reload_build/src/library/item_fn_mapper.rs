use crate::runtime::models::BuildFnData;
use code_reload_core::services::IFnProcessor;
use code_reload_core::SourceCodeId;
use quote::ToTokens;
use std::sync::Arc;
use syn::ItemFn;

pub trait IItemFnMapper {
    fn map(
        &self,
        item_fn: ItemFn,
        source_code_id: SourceCodeId,
        maybe_impl_block_type: Option<&[u8]>,
    ) -> BuildFnData;
}

pub struct ItemFnMapper {
    pub fn_processor: Arc<dyn IFnProcessor>,
}

impl IItemFnMapper for ItemFnMapper {
    fn map(
        &self,
        mut item_fn: ItemFn,
        source_code_id: SourceCodeId,
        maybe_impl_block_type: Option<&[u8]>,
    ) -> BuildFnData {
        self.fn_processor
            .mangle_function_name(&mut item_fn, &source_code_id);
        let mut bare_signature = self.fn_processor.get_bare_function_signature(&item_fn.sig);
        if let Some(impl_block_type) = maybe_impl_block_type {
            let impl_block_type_str = str::from_utf8(impl_block_type).unwrap();
            let module = source_code_id.get_module();
            let new_self_type = format!("{}::{}", module, impl_block_type_str);
            for arg in bare_signature
                .inputs
                .iter_mut()
                .filter(|arg| arg.ty.to_token_stream().to_string().contains("Self"))
            {
                let new_ident = &arg.ty
                                    .to_token_stream()
                                    .to_string()
                                    .replace("Self", &new_self_type);
                // println!("impl_block_type: '{}'", impl_block_type_str);
                // println!("source: '{:?}'", source_code_id.get_path());
                // println!("new_ident: '{}'", new_ident);
                arg.ty = syn::parse_str(
                    new_ident
                )
                .unwrap();
            }
            bare_signature.output = syn::parse_str(
                &bare_signature
                    .output
                    .to_token_stream()
                    .to_string()
                    .replace("Self", &new_self_type),
            )
            .unwrap();
        }
        let ident = item_fn.sig.ident;
        // log!("ident = '{}'", ident.to_string());
        let build_fn_data =
            BuildFnData::new(bare_signature, ident, source_code_id, maybe_impl_block_type);
        return build_fn_data;
    }
}
