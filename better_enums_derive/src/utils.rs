use syn::{Attribute, Ident};

pub(crate) fn extract_repr(attrs: &[Attribute], enum_name: &Ident) -> syn::Result<Ident> {
    for attr in attrs {
        if attr.path().is_ident("repr") {
            let repr = attr
                .meta
                .path()
                .get_ident()
                .cloned()
                .ok_or_else(|| syn::Error::new_spanned(attr, "better_enums: repr is missing"));
            return repr;
        }
    }

    Err(syn::Error::new_spanned(
        enum_name,
        "better_enums: repr attribute missing",
    ))
}
