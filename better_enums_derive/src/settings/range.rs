use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Fields, Ident, Token};

use super::Setting;
use crate::{
    Domain,
    model::{Number, Variant, VariantMapping},
};

pub(super) struct RangeSetting;

impl RangeSetting {
    fn condition(mapping: &Variant) -> TokenStream {
        match mapping {
            Variant::Single { expr, .. } => quote!(value == #expr),
            Variant::Range(range) => match (range.start.as_ref(), range.end.as_ref()) {
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
    fn validate(
        &self,
        variants: &mut dyn Iterator<Item = &mut syn::Variant>,
        bounds: &Domain,
    ) -> Result<Vec<VariantMapping>, syn::Error> {
        let mut next = Some(if bounds.unsigned {
            Number::Unsigned(0)
        } else {
            Number::Signed(0)
        });
        let mut result = Vec::new();

        for variant in variants {
            if !matches!(variant.fields, Fields::Unit) {
                return Err(syn::Error::new_spanned(
                    &*variant,
                    "better_enums: variant cannot have additional data",
                ));
            }

            let mappings = if let Some((_, expr)) = &variant.discriminant {
                Variant::parse(expr, &bounds)?
            } else {
                let value = next.ok_or_else(|| {
                    syn::Error::new_spanned(
                        &*variant,
                        "better_enums: no implicit value remains in repr range",
                    )
                })?;
                vec![Variant::Single {
                    expr: value.into_expr(),
                    value,
                }]
            };

            if mappings.is_empty() {
                return Err(syn::Error::new_spanned(
                    &*variant,
                    "better_enums: a variant must map to at least one value",
                ));
            }

            let representative = mappings[0].lower();
            variant.discriminant = Some((<Token![=]>::default(), representative.into_expr()));
            next = mappings
                .iter()
                .map(Variant::upper)
                .max()
                .and_then(|value| match value {
                    Number::Signed(value) => value.checked_add(1).map(Number::Signed),
                    Number::Unsigned(value) => value.checked_add(1).map(Number::Unsigned),
                })
                .filter(|value| *value <= bounds.max);

            let current = VariantMapping {
                name: variant.ident.clone(),
                mappings,
            };

            if !current.valid() {
                return Err(syn::Error::new_spanned(
                    &current.name,
                    format!(
                        "better_enums: mappings for {} overlap or duplicate each other",
                        current.name
                    ),
                ));
            }

            for previous in &result {
                if current.overlaps(previous) {
                    return Err(syn::Error::new_spanned(
                        &current.name,
                        format!(
                            "better_enums: mapping for {} overlaps mapping for {}",
                            current.name, previous.name
                        ),
                    ));
                }
            }

            result.push(current);
        }

        Ok(result)
    }

    fn generate(
        &self,
        variants: &[VariantMapping],
        enum_name: &Ident,
        bounds: &Domain,
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

        let repr = &bounds.ident;

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
