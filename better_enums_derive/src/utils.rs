use syn::{Attribute, Ident};

use crate::model::Domain;

pub(crate) fn extract_repr(attrs: &[Attribute], enum_name: &Ident) -> syn::Result<Domain> {
    for attr in attrs {
        if attr.path().is_ident("repr") {
            let repr = attr.parse_args().map_err(|e| {
                syn::Error::new(e.span(), "better_enums: repr must be an integer type")
            })?;
            return Domain::try_from(&repr);
        }
    }

    Err(syn::Error::new_spanned(
        enum_name,
        "better_enums: repr attribute missing",
    ))
}
