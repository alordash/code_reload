use code_reload_core::services::IFnProcessor;
use code_reload_runtime::models::FnData;
use std::sync::Arc;
use syn::ItemFn;

pub trait IItemFnMapper {
    fn map(&self, item_fn: ItemFn) -> FnData;
}

pub struct ItemFnMapper {
    pub fn_processor: Arc<dyn IFnProcessor>,
}

impl IItemFnMapper for ItemFnMapper {
    fn map(&self, item_fn: ItemFn) -> FnData {
        let bare_signature = self.fn_processor.get_bare_function_signature(&item_fn.sig);
        let ident = item_fn.sig.ident;
        let fn_data = FnData::new(bare_signature, ident);
        return fn_data;
    }
}
