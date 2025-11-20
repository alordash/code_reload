use syn::{Ident, TypeBareFn};

pub struct FnData {
    bare_signature: TypeBareFn,
    ident: Ident,
}

impl FnData {
    pub fn new(bare_signature: TypeBareFn, ident: Ident) -> Self {
        Self {
            bare_signature,
            ident,
        }
    }

    pub fn bare_signature(&self) -> &TypeBareFn {
        &self.bare_signature
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }
}
