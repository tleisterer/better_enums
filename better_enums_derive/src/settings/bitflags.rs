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

            if mappings.len() != 1 {
                return Err(syn::Error::new_spanned(
                    &variant,
                    "better_enums: a variant must map to exactly one value",
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

            current.check_valid()?;

            result
                .iter()
                .try_for_each(|previous| current.check_overlap(previous))?;

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

        let exhaustive = (full == bounds.max.get_unsigned()).then(|| {
            quote! { impl #krate::flags::ExhaustiveBit for #enum_name {} }
        });

        quote! {
            impl std::ops::BitOr for #enum_name {
                type Output = #krate::flags::Bitflags<Self>;
                fn bitor(self, rhs: Self) -> Self::Output {
                    // unwrap is safe to call here, because it only converts valid enum values
                    <Self as #krate::flags::TryFromBits>::try_from_bits(self.value() | rhs.value()).unwrap()
                }
            }

            impl std::ops::BitAnd<#krate::flags::Bitflags<Self>> for #enum_name {
                type Output = #krate::flags::Bitflags<Self>;
                #[inline(always)]
                fn bitand(self, rhs: #krate::flags::Bitflags<Self>) -> Self::Output {
                    rhs.bitand(self)
                }
            }

            impl std::ops::BitOr<#krate::flags::Bitflags<Self>> for #enum_name {
                type Output = #krate::flags::Bitflags<Self>;
                #[inline(always)]
                fn bitor(self, rhs: #krate::flags::Bitflags<Self>) -> Self::Output {
                    rhs.bitor(self)
                }
            }

            impl std::ops::BitXor<#krate::flags::Bitflags<Self>> for #enum_name {
                type Output = #krate::flags::Bitflags<Self>;
                #[inline(always)]
                fn bitxor(self, rhs: #krate::flags::Bitflags<Self>) -> Self::Output {
                    rhs.bitxor(self)
                }
            }

            impl #krate::flags::Bit for #enum_name {
                type Repr = #repr;

                const FULL: Self::Repr = #full as Self::Repr;
                const EMPTY: Self::Repr = 0;

                fn value(&self) -> Self::Repr {
                    // SAFETY: `#enum_name` is #[repr(#repr)], so its representation
                    // is an integer of type `#repr` containing the discriminant.
                    unsafe { std::ptr::read(self as *const #enum_name as *const #repr) }
                }
            }

            #exhaustive
        }
    }
}
