use crate::model::{Domain, Number, Variant, VariantMapping};

use super::Setting;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

pub(super) struct BitflagsSetting;

impl Setting for BitflagsSetting {
    fn validate(
        &self,
        variants: &mut dyn Iterator<Item = &mut syn::Variant>,
        bounds: &Domain,
    ) -> Result<Vec<VariantMapping>, syn::Error> {
        if !bounds.unsigned {
            return Err(syn::Error::new_spanned(
                &bounds.ident,
                "better_enums: bitflags repr must be unsigned",
            ));
        }

        let mut next = Some(Number::Unsigned(1));
        let mut result = Vec::new();

        for variant in variants {
            if !matches!(variant.fields, syn::Fields::Unit) {
                return Err(syn::Error::new_spanned(
                    variant,
                    "better_enums: variant cannot have additional data",
                ));
            }

            let mappings = if let Some((_, expr)) = &variant.discriminant {
                Variant::parse(expr, bounds)?
            } else {
                let value = next.ok_or_else(|| {
                    syn::Error::new_spanned(
                        &variant,
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
                    &variant,
                    "better_enums: a variant must map to at least one value",
                ));
            }

            next = mappings
                .iter()
                .try_fold(Number::Signed(0), |current, last| match last {
                    Variant::Single { value, expr } => {
                        if !value.is_power_of_two() {
                            Err(syn::Error::new_spanned(
                                expr,
                                "better_enums: bitflag values must be a non-zero power of two",
                            ))
                        } else if value > &current {
                            Ok(*value)
                        } else {
                            Ok(current)
                        }
                    }

                    Variant::Range(v) => Err(syn::Error::new_spanned(
                        &v.expr,
                        "better_enums: Ranges are not supported in bitflags",
                    )),
                })
                .and_then(|value| Ok(value.get_unsigned().checked_shl(1).map(Number::Unsigned)))?
                .filter(|value| *value <= bounds.max);

            let current = VariantMapping {
                name: variant.ident.clone(),
                mappings,
            };

            if !current.valid() {
                return Err(syn::Error::new_spanned(
                    &variant.ident,
                    format!(
                        "better_enums: mappings for {} overlap or duplicate each other",
                        current.name
                    ),
                ));
            }

            for previous in &result {
                if current.overlaps(previous) {
                    return Err(syn::Error::new_spanned(
                        &variant.ident,
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
        enum_name: &syn::Ident,
        bounds: &Domain,
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

        let full = variants.iter().fold(0, |all, v| {
            all | v.mappings.iter().fold(0, |acc, m| match m {
                Variant::Single { value, .. } => acc | value.get_unsigned(),
                Variant::Range(_) => panic!("better_enums: Ranges are not supported in bitflags"),
            })
        });

        let repr = &bounds.ident;

        quote! {
            impl std::ops::BitOr for #enum_name {
                type Output = #krate::flags::Bitflags<Self>;
                fn bitor(self, rhs: Self) -> Self::Output {
                    Self::from_bits(self.value() | rhs.value())
                }
            }

            impl #krate::flags::Bit for #enum_name {
                type Repr = #repr;

                const FULL: Self::Repr = #full;
                const EMPTY: Self::Repr = 0;

                fn value(&self) -> Self::Repr {
                    *self as Self::Repr
                }
            }
        }
    }
}
