use crate::model::Domain;

use super::Setting;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error as SynError, Fields, Ident, ItemEnum};

pub(super) struct BitflagsSetting;

impl Setting for BitflagsSetting {
    fn validate(&self, variants: &[VariantMapping]) -> Result<(), SynError> {
        for v in variants {
            if !v.1.is_power_of_two() || v.1 == 0 {
                return Err(SynError::new(
                    v.0,
                    "better_enums: Variant must be a non zero power of 2",
                ));
            }
        }
        Ok(())
    }

    fn generate(
        &self,
        enum_name: &syn::Ident,
        repr: &syn::Ident,
        variants: &[VariantMapping],
    ) -> TokenStream {
        let krate =
            crate_name("better_enums").expect("If this crate is not included something went wrong");

        let krate = match krate {
            FoundCrate::Itself => quote! { better_enums },
            FoundCrate::Name(name) => {
                let ident = Ident::new(&name, Span::call_site());
                quote! { #ident }
            }
        };

        let full = variants.iter().fold(0, |all, v| all | v.1);

        quote! {
            impl std::ops::BitOr for #enum_name {
                type Output = #krate::flags::Bitflags<Self>;
                fn bitor(self, rhs: Self) -> Self::Output {
                    Self::from_bits(self.value() | rhs.value())
                }
            }

            impl #krate::flags::Bit for #enum_name {
                type Repr = #repr;

                const FULL: Self::Repr = 0b1111_1111;
                const EMPTY: Self::Repr = #full as #repr;

                fn value(&self) -> Self::Repr {
                    *self as Self::Repr
                }
            }
        }
    }

    fn collect_variants(
        input: &mut ItemEnum,
        domain: Domain,
    ) -> Result<Vec<VariantMapping>, SynError> {
        if !domain.unsigned {
            return Err(SynError::new(domain.span, "better_enums: bitflags repr must be unsigned"));
        }

        let mut result = Vec::new();

        for variant in &mut input.variants {
            if !matches!(variant.fields, Fields::Unit) {
                return Err(SynError::new_spanned(
                    &*variant,
                    "better_enums: variant cannot have additional data",
                ));
            }


        }

        Ok(result)
    }
}
