mod range;

use proc_macro2::TokenStream;
use syn::{Ident, ItemEnum};

use crate::model::{Domain, VariantMapping};
use range::RangeSetting;

pub(crate) trait Setting {
    fn validate(&self, variants: &[VariantMapping]) -> Result<(), syn::Error>;

    fn generate(&self, enum_name: &Ident, repr: &Ident, variants: &[VariantMapping])
    -> TokenStream;
}

pub(crate) fn parse_setting(_attributes: TokenStream) -> Result<Box<dyn Setting>, syn::Error> {
    Ok(Box::new(RangeSetting))
}

pub(crate) fn prepare(
    setting: &dyn Setting,
    input: &mut ItemEnum,
    bounds: Domain,
) -> Result<Vec<VariantMapping>, syn::Error> {
    let variants = crate::parse::collect_variants(input, bounds)?;
    setting.validate(&variants)?;
    Ok(variants)
}
