use code_reload_core::services::IFnProcessor;
use std::sync::Arc;
use syn::ItemFn;
use crate::runtime::models::BuildFnData;

pub trait IItemFnMapper {
    fn map(&self, item_fn: ItemFn) -> BuildFnData;
}

pub struct ItemFnMapper {
    pub fn_processor: Arc<dyn IFnProcessor>,
}

impl IItemFnMapper for ItemFnMapper {
    fn map(&self, item_fn: ItemFn) -> BuildFnData {
        let bare_signature = self.fn_processor.get_bare_function_signature(&item_fn.sig);
        let ident = item_fn.sig.ident;
        let build_fn_data = BuildFnData::new(bare_signature, ident);
        return build_fn_data;
    }
}
