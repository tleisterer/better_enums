//! Map enum variants to single values or ranges of a primitive integer type.
//!
//! The [`better_enums`] attribute generates an implementation of
//! [`TryFrom`] for the enum. Each variant can declare one or more values,
//! ranges, or arrays of values. Variants without an explicit mapping receive
//! the next available value, starting at zero.
//!
//! # Example
//!
//! ```
//! use better_enums::better_enums;
//!
//! #[better_enums]
//! #[repr(u16)]
//! enum HttpStatus {
//!     Ok = 200,
//!     ClientError = 400..500,
//!     ServerError = 500..=599,
//! }
//!
//! assert!(matches!(HttpStatus::try_from(200), Ok(HttpStatus::Ok)));
//! assert!(matches!(HttpStatus::try_from(404), Ok(HttpStatus::ClientError)));
//! assert!(HttpStatus::try_from(302).is_err());
//! ```
//!
//! The enum must have an integer `repr` such as `u8`, `i16`, or `u128`, and it
//! cannot be generic. Mappings must be non-empty, within the representation's
//! range, and cannot overlap.

pub mod error;
pub use better_enums_derive::better_enums;