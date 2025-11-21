use crate::runtime::models::BuildFnData;
use code_reload_core::SourceCodeId;
use code_reload_core::services::IFnProcessor;
use std::sync::Arc;
use syn::ItemFn;

pub trait IItemFnMapper {
    fn map(
        &self,
        item_fn: ItemFn,
        source_code_id: SourceCodeId,
        impl_block_type: Option<&[u8]>,
    ) -> BuildFnData;
}

pub struct ItemFnMapper {
    pub fn_processor: Arc<dyn IFnProcessor>,
}

impl IItemFnMapper for ItemFnMapper {
    fn map(
        &self,
        item_fn: ItemFn,
        source_code_id: SourceCodeId,
        impl_block_type: Option<&[u8]>,
    ) -> BuildFnData {
        let bare_signature = self.fn_processor.get_bare_function_signature(&item_fn.sig);
        let ident = item_fn.sig.ident;
        let build_fn_data =
            BuildFnData::new(bare_signature, ident, source_code_id, impl_block_type);
        return build_fn_data;
    }
}
