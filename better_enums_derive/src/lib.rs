//! This crate is an implementation detail; use
//! [`better_enums`](https://docs.rs/better_enums/latest/better_enums/) instead.
mod model;
mod settings;
mod utils;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, parse_macro_input};

use crate::{settings::SettingFactory, utils::extract_repr};

#[doc(hidden)]
fn better_enums_impl(attr: TokenStream, mut input: ItemEnum) -> Result<TokenStream, syn::Error> {
    if input.generics.lt_token.is_some() || input.generics.where_clause.is_some() {
        return Err(syn::Error::new_spanned(
            &input.generics,
            "better_enums: enum cannot be generic",
        ));
    }

    let setting = SettingFactory::create(attr)?;
    let bounds = extract_repr(&input.attrs, &input.ident)?;
    let variants = setting.validate(&mut input.variants.iter_mut(), &bounds)?;
    let code = setting.generate(&variants, &input.ident, &bounds);

    Ok(quote! { #input #code })
}

/// Generates conversion support for enums whose variants map to integer
/// values, ranges, or bitflags. Range-based enums implement `TryFrom<repr>`;
/// bitflag enums use `TryFromBits`, and exhaustive bitflag enums also use
/// `FromBits`.
///
/// # Ranges
/// Use `#[better_enums]` or `#[better_enums(range)]` to generate a
/// `TryFrom<repr>` implementation:
///
/// ```
/// use better_enums::better_enums;
///
/// #[better_enums]
/// #[repr(u16)]
/// #[derive(Debug, PartialEq, Eq)]
/// enum HttpStatus {
///     Ok = 200,
///     ClientError = 400..500,
///     ServerError = 500..=599,
/// }
///
/// # fn main() {
/// assert_eq!(HttpStatus::try_from(200), Ok(HttpStatus::Ok));
/// assert_eq!(HttpStatus::try_from(404), Ok(HttpStatus::ClientError));
/// assert!(HttpStatus::try_from(302).is_err());
/// # }
/// ```
///
/// An integer literal maps one value to a variant. Variants without an explicit
/// mapping receive the next available value, starting at zero. Mappings can also
/// contain inclusive ranges (`10..=20`), exclusive ranges (`10..20`), unbounded
/// ranges (`..10`, `10..`, or `..`), and arrays combining numbers and ranges:
///
/// ```
/// # use better_enums::better_enums;
/// #[better_enums]
/// #[repr(i8)]
/// enum Number {
///     Negative = ..0,
///     Zero,
///     Positive = 1..,
/// }
/// ```
///
/// An array maps every listed number or range to the same variant:
///
/// ```
/// # use better_enums::better_enums;
/// #[better_enums]
/// #[repr(u16)]
/// #[derive(Debug, PartialEq, Eq)]
/// enum Status {
///     Initial,
///     Success = [200..300, 304],
/// }
///
/// # fn main() {
/// assert_eq!(Status::try_from(0), Ok(Status::Initial));
/// assert_eq!(Status::try_from(200), Ok(Status::Success));
/// assert_eq!(Status::try_from(304), Ok(Status::Success));
/// assert!(Status::try_from(301).is_err());
/// # }
/// ```
///
/// # Bitflags
/// Use `#[better_enums(bitflags)]` for a set of independent flags. Bitflags must
/// use an unsigned representation, and every variant must be a unit variant with
/// one non-zero power-of-two value. Ranges and arrays are not supported in this
/// mode.
///
/// Omitted discriminants are assigned automatically: if the first variant
/// omits its value, it defaults to `1`; each later omitted value is twice the
/// preceding variant's value.
///
/// ```
/// use better_enums::{better_enums, flags::{TryFromBits, Bit}};
///
/// #[better_enums(bitflags)]
/// #[repr(u8)]
/// #[derive(Debug, PartialEq)]
/// enum Permission {
///     Read = 0b0000_0001,
///     Write = 0b0000_0010,
///     Execute = 0b0000_0100,
/// }
///
/// # fn main() {
/// let permissions = Permission::Read | Permission::Write;
/// assert!(permissions.contains(Permission::Read));
/// assert_eq!(permissions.value(), 0b0000_0011);
///
/// assert_eq!(Permission::try_from_bits(0b0000_0111),
///     Ok(Permission::Read | Permission::Write | Permission::Execute));
/// assert!(Permission::try_from_bits(0b0000_1000).is_err());
/// # }
/// ```
///
/// When the enum defines every bit in its representation, you can use the
/// infallible [`from_bits`](https://docs.rs/better_enums/latest/better_enums/flags/trait.FromBits.html#tymethod.from_bits) conversion:
///
/// ```
/// use better_enums::{better_enums, flags::{FromBits, Bit}};
///
/// #[better_enums(bitflags)]
/// #[repr(u8)]
/// enum ExhaustiveFlags {
///     One = 0b0000_0001,
///     Two = 0b0000_0010,
///     Four = 0b0000_0100,
///     Eight = 0b0000_1000,
///     Sixteen = 0b0001_0000,
///     ThirtyTwo = 0b0010_0000,
///     SixtyFour = 0b0100_0000,
///     OneHundredTwentyEight = 0b1000_0000,
/// }
///
/// # fn main() {
/// let flags = ExhaustiveFlags::from_bits(0b0000_0011);
/// assert!(flags.contains(ExhaustiveFlags::One));
/// assert_eq!(flags.value(), 0b0000_0011);
/// # }
/// ```
#[proc_macro_attribute]
pub fn better_enums(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match better_enums_impl(attr.into(), parse_macro_input!(input as ItemEnum)) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}
