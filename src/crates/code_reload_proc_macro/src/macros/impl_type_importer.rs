use code_reload_core::{constants, SourceCodeId};

pub trait IImplTypeImporter {
    fn try_import(&self, source_code_id: &SourceCodeId) -> Option<Vec<u8>>;
}

pub struct ImplTypeImporter;

impl IImplTypeImporter for ImplTypeImporter {
    fn try_import(&self, source_code_id: &SourceCodeId) -> Option<Vec<u8>> {
        let path = source_code_id.get_path();
        let file_path = constants::IMPL_BLOCK_TYPES_DIR.join(path);
        // println!("reading impl block type from: '{:?}'", file_path);
        let impl_block_type = std::fs::read(file_path).ok();
        return impl_block_type;
    }
}
