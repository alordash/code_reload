use crate::debug_log::log;
use crate::library::fn_syntax_extractor::IFnSyntaxExtractor;
use memmap2::Mmap;
use std::path::Path;
use std::sync::Arc;
use syn::__private::ToTokens;
use crate::runtime::models::FnData;

pub trait IFileProcessor {
    fn process(&self, file_path: &Path) -> Vec<FnData>;
}

pub struct FileProcessor {
    pub fn_syntax_extractor: Arc<dyn IFnSyntaxExtractor>,
}

impl IFileProcessor for FileProcessor {
    fn process(&self, file_path: &Path) -> Vec<FnData> {
        let file = std::fs::File::open(file_path).unwrap();
        let file_memory = unsafe { Mmap::map(&file).unwrap() };

        let fn_datas = self.fn_syntax_extractor.extract(&file_memory);
        for (fn_syntax_number, fn_data) in fn_datas.iter().enumerate() {
            log!(
                "fn_syntax №{}:\n'{}': '{}'",
                fn_syntax_number,
                fn_data.ident().to_token_stream(),
                fn_data.bare_signature().to_token_stream()
            );
        }

        return fn_datas;
    }
}
