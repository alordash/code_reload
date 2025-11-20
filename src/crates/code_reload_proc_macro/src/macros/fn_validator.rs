use syn::ItemFn;

pub trait IFnValidator {
    fn validate(&self, fn_syntax: &ItemFn);
}

pub struct FnValidator;

impl IFnValidator for FnValidator {
    fn validate(&self, fn_syntax: &ItemFn) {
        // Extract to validation function
        if fn_syntax.sig.generics.lt_token.is_some() {
            panic!("Hotreload of generic functions is not supported.")
        }

        if fn_syntax.attrs.iter().any(|attr| {
            attr.meta
                .path()
                .get_ident()
                .is_some_and(|attr_ident| attr_ident == stringify!(hotreload))
        }) {
            panic!("#[hotreload] attribute can not be used more than once on the same function.");
        }
    }
}