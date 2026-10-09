mod bitflags;
mod range;

use proc_macro2::TokenStream;
use syn::{Ident, Variant};

use crate::{Domain, model::VariantMapping};

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
