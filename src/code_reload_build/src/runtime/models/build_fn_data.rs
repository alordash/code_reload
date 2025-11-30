use code_reload_core::SourceCodeId;
use syn::{Ident, TypeBareFn};

pub struct BuildFnData {
    bare_signature: TypeBareFn,
    ident: Ident,
    source_code_id: SourceCodeId,
    impl_block_type: Option<Vec<u8>>,

    // TODO - it's for debug only
    // impl_block_type_string: Option<String>,
}

impl BuildFnData {
    pub fn new(
        bare_signature: TypeBareFn,
        ident: Ident,
        source_code_id: SourceCodeId,
        impl_block_type: Option<&[u8]>,
    ) -> Self {
        Self {
            bare_signature,
            ident,
            source_code_id,
            impl_block_type: impl_block_type.map(Vec::from),
            // impl_block_type_string: impl_block_type
            //     .map(str::from_utf8)
            //     .map(Result::unwrap)
            //     .map(String::from),
        }
    }

    pub fn bare_signature(&self) -> &TypeBareFn {
        &self.bare_signature
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn source_code_id(&self) -> &SourceCodeId {
        &self.source_code_id
    }

    pub fn impl_block_type(&self) -> Option<&[u8]> {
        self.impl_block_type.as_deref()
    }
}
