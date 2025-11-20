use code_reload_core::SourceCodeId;
use syn::{Ident, TypeBareFn};

pub struct BuildFnData {
    bare_signature: TypeBareFn,
    ident: Ident,
    source_code_id: SourceCodeId,
}

impl BuildFnData {
    pub fn new(bare_signature: TypeBareFn, ident: Ident, source_code_id: SourceCodeId) -> Self {
        Self {
            bare_signature,
            ident,
            source_code_id,
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
}
