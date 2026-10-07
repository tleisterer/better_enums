mod bitflags;
mod range;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Variant};

use crate::model::{Domain, VariantMapping};

#[cfg(feature = "rename")]
pub(crate) fn get_crate_name() -> TokenStream {
    use proc_macro_crate::{FoundCrate, crate_name};
    let krate = crate_name("better_enums")
        .expect("Dont use better_enums_derive directly: use better_enums instead");

    match krate {
        FoundCrate::Itself => quote! { crate },
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, proc_macro2::Span::call_site());
            quote! { ::#ident }
        }
    }
}

#[cfg(not(feature = "rename"))]
pub(crate) fn get_crate_name() -> TokenStream {
    quote! { ::better_enums }
}

pub(crate) trait Setting {
    fn validate(
        &self,
        variants: &mut dyn Iterator<Item = &mut Variant>,
        bounds: &Domain,
    ) -> Result<Vec<VariantMapping>, syn::Error>;
    fn generate(
        &self,
        variants: &[VariantMapping],
        enum_name: &Ident,
        repr: &Domain,
    ) -> TokenStream;
}

pub(crate) struct SettingFactory;

impl SettingFactory {
    pub(crate) fn create(attr: TokenStream) -> syn::Result<Box<dyn Setting>> {
        match attr.to_string().as_str() {
            "" | "range" => Ok(Box::new(range::RangeSetting)),
            "bitflags" => Ok(Box::new(bitflags::BitflagsSetting)),
            _ => Err(syn::Error::new_spanned(
                attr,
                "better_enums: unknown setting",
            )),
        }
    }
}
