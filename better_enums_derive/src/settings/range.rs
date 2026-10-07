use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, Ident, Token};

use super::{Setting, get_crate_name};
use crate::model::{Domain, Number, RangeValue, Variant, VariantMapping};

pub(super) struct RangeSetting;

impl RangeSetting {
    fn arm(variant: &VariantMapping, enum_name: &Ident) -> TokenStream {
        let conditions = variant.mappings.iter().map(|mapping| match mapping {
            Variant::Single { expr, .. } => quote!(value == #expr),
            Variant::Range(RangeValue { expr, .. }) => {
                quote! { <_ as std::ops::RangeBounds<_>>::contains(&(#expr), &value) }
            }
        });
        let name = &variant.name;
        quote! { else if #(#conditions)||* { Ok(#enum_name::#name) } }
    }

    fn next_value(mappings: &[Variant], bounds: &Domain) -> Option<Number> {
        mappings
            .iter()
            .map(Variant::upper)
            .max()
            .and_then(|value| Number::checked_add(value, 1))
            .filter(|value| *value <= bounds.max)
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
                    variant,
                    "better_enums: variant cannot have additional data",
                ));
            }

            let mappings = if let Some((_, expr)) = &variant.discriminant {
                Variant::parse(expr, bounds)?
            } else {
                let value = next.ok_or(syn::Error::new_spanned(
                    &variant,
                    "better_enums: no implicit value remains in repr range",
                ))?;

                vec![Variant::Single {
                    expr: value.into_expr(),
                    value,
                }]
            };

            if mappings.is_empty() {
                return Err(syn::Error::new_spanned(
                    variant,
                    "better_enums: a variant must map to at least one value",
                ));
            }

            let representative = mappings[0].lower();
            variant.discriminant = Some((<Token![=]>::default(), representative.into_expr()));

            let current = VariantMapping {
                name: variant.ident.clone(),
                mappings,
            };

            current.check_valid()?;
            result
                .iter()
                .try_for_each(|previous| current.check_overlap(previous))?;

            next = Self::next_value(&current.mappings, bounds);

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
        let arms = variants.iter().map(|variant| Self::arm(variant, enum_name));

        let krate = get_crate_name();

        let repr = &bounds.ident;

        quote! {
            impl std::convert::TryFrom<#repr> for #enum_name {
                type Error = #krate::error::BetterEnumsError<#repr>;
                fn try_from(value: #repr) -> Result<Self, Self::Error> {
                    if false { unreachable!() }
                    #(#arms)*
                    else {
                        Err(Self::Error::Discriminant(value))
                    }
                }
            }
        }
    }
}
