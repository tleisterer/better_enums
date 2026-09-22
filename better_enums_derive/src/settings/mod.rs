mod bitflags;
mod range;

use proc_macro2::TokenStream;
use syn::{Ident, Variant};

use crate::{Domain, settings::range::RangeSetting};

pub(crate) trait Setting {
    type Mapping;
    fn validate<'a>(
        &self,
        variants: impl Iterator<Item = &'a mut Variant>,
        bounds: Domain,
    ) -> Result<Vec<Self::Mapping>, syn::Error>;

    fn generate(&self, enum_name: &Ident, repr: &Ident, variants: &[Self::Mapping]) -> TokenStream;
}

pub(crate) struct SettingFactory;

impl SettingFactory {
    pub(crate) fn create(attr: TokenStream) -> syn::Result<impl Setting> {
        let _ = attr; // TODO: parse attr to determine which setting to create

        Ok(RangeSetting)
    }
}
