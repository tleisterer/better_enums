use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error as SynError, Fields, Ident, Token, Variant, spanned::Spanned};

use super::Setting;
use crate::{
    Domain,
    model::{RangeMapping, Number, RangeVariantMapping},
};

pub(super) struct RangeSetting;

impl RangeSetting {
    fn condition(mapping: &RangeMapping) -> TokenStream {
        match mapping {
            RangeMapping::Single { expr, .. } => quote!(value == #expr),
            RangeMapping::Range(range) => match (range.start.as_ref(), range.end.as_ref()) {
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
    type Mapping = crate::model::RangeVariantMapping;
    fn validate<'a>(
        &self,
        mut variants: impl Iterator<Item = &'a mut Variant>,
        bounds: Domain,
    ) -> Result<Vec<Self::Mapping>, SynError> {
        let mut next = Some(if bounds.unsigned {
            Number::Unsigned(0)
        } else {
            Number::Signed(0)
        });
        let mut result = Vec::new();

        for variant in &mut variants {
            if !matches!(variant.fields, Fields::Unit) {
                return Err(SynError::new_spanned(
                    &*variant,
                    "better_enums: variant cannot have additional data",
                ));
            }

            let mappings = if let Some((_, expr)) = &variant.discriminant {
                RangeMapping::parse(expr, bounds)?
            } else {
                let value = next.ok_or_else(|| {
                    SynError::new_spanned(
                        &*variant,
                        "better_enums: no implicit value remains in repr range",
                    )
                })?;
                vec![RangeMapping::Single {
                    expr: value.into_expr(),
                    value,
                }]
            };

            if mappings.is_empty() {
                return Err(SynError::new_spanned(
                    &*variant,
                    "better_enums: a variant must map to at least one value",
                ));
            }

            let representative = mappings[0].lower();
            variant.discriminant = Some((<Token![=]>::default(), representative.into_expr()));
            next = mappings
                .iter()
                .map(RangeMapping::upper)
                .max()
                .and_then(|value| match value {
                    Number::Signed(value) => value.checked_add(1).map(Number::Signed),
                    Number::Unsigned(value) => value.checked_add(1).map(Number::Unsigned),
                })
                .filter(|value| *value <= bounds.max);

            let current = RangeVariantMapping {
                name: variant.ident.clone(),
                span: variant.span(),
                mappings,
            };

            if !current.valid() {
                return Err(SynError::new(
                    current.span,
                    format!(
                        "better_enums: mappings for {} overlap or duplicate each other",
                        current.name
                    ),
                ));
            }

            for previous in &result {
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

            result.push(current);
        }

        Ok(Vec::new())
    }

    fn generate(
        &self,
        enum_name: &Ident,
        repr: &Ident,
        variants: &[RangeVariantMapping],
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
