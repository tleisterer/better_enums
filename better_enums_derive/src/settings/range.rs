use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error as SynError, Ident};

use super::Setting;
use crate::model::{Mapping, VariantMapping};

pub(super) struct RangeSetting;

impl RangeSetting {
    fn condition(mapping: &Mapping) -> TokenStream {
        match mapping {
            Mapping::Single { expr, .. } => quote!(value == #expr),
            Mapping::Range(range) => match (range.start.as_ref(), range.end.as_ref()) {
                (Some(start), Some(end)) => {
                    if range.inclusive {
                        quote!((#start..=#end).contains(&value))
                    } else {
                        quote!((#start..#end).contains(&value))
                    }
                }
                (Some(start), None) => quote!(value >= #start),
                (None, Some(end)) => {
                    if range.inclusive {
                        quote!(value <= #end)
                    } else {
                        quote!(value < #end)
                    }
                }
                (None, None) => quote!(true),
            },
        }
    }
}

impl Setting for RangeSetting {
    fn validate(&self, variants: &[VariantMapping]) -> Result<(), syn::Error> {
        for (index, current) in variants.iter().enumerate() {
            if !current.valid() {
                return Err(SynError::new(
                    current.span,
                    format!(
                        "better_enums: mappings for {} overlap or duplicate each other",
                        current.name
                    ),
                ));
            }

            for previous in &variants[..index] {
                if current.overlaps(previous) {
                    return Err(SynError::new(
                        current.span,
                        format!(
                            "better_enums: mapping for {} overlaps mapping for {}",
                            current.name, previous.name
                        ),
                    ));
                }
            }
        }

        Ok(())
    }

    fn generate(
        &self,
        enum_name: &Ident,
        repr: &Ident,
        variants: &[VariantMapping],
    ) -> TokenStream {
        let arms = variants.iter().map(|variant| {
            let conditions = variant.mappings.iter().map(Self::condition);
            let name = &variant.name;
            quote! { if #(#conditions)||* { return Ok(#enum_name::#name); } }
        });

        let krate =
            crate_name("better_enums").expect("If this crate is not included something went wrong");

        let krate = match krate {
            FoundCrate::Itself => quote! { better_enums },
            FoundCrate::Name(name) => {
                let ident = Ident::new(&name, Span::call_site());
                quote! { #ident }
            }
        };

        quote! {
            impl std::convert::TryFrom<#repr> for #enum_name {
                type Error = #krate::error::BetterEnumsError<#repr>;
                fn try_from(value: #repr) -> Result<Self, Self::Error> {
                    #(#arms)*
                    Err(Self::Error::Discriminant(value))
                }
            }
        }
    }
}
