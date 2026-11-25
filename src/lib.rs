//! Map enum variants to values, ranges, or bitflags of a primitive integer type.
//!
//! The [`macro@better_enums`] attribute generates an implementation of
//! [`TryFrom`] for the enum. Each variant can declare one or more values,
//! ranges, or arrays of values. Variants without an explicit mapping receive
//! the next available value, starting at zero.
//!
//! The enum must have a primitive integer representation (`u8`, `i16`, and so
//! on), use unit variants, and cannot be generic.
//!
//! # Range-based enums
//!
//! ```
//! use better_enums::better_enums;
//!
//! #[better_enums]
//! #[repr(u16)]
//! #[derive(Debug, PartialEq, Eq)]
//! enum HttpStatus {
//!     Ok = 200,
//!     ClientError = 400..500,
//!     ServerError = 500..=599,
//! }
//!
//! assert_eq!(HttpStatus::try_from(200), Ok(HttpStatus::Ok));
//! assert_eq!(HttpStatus::try_from(404), Ok(HttpStatus::ClientError));
//! assert!(HttpStatus::try_from(302).is_err());
//! ```
//!
//! Mappings can contain integer literals, inclusive or exclusive ranges,
//! unbounded ranges, and arrays combining numbers and ranges:
//!
//! ```
//! # use better_enums::better_enums;
//! #[better_enums]
//! #[repr(u16)]
//! #[derive(Debug, PartialEq, Eq)]
//! enum Status {
//!     Initial,
//!     Success = [200..300, 304],
//! }
//!
//! assert_eq!(Status::try_from(0), Ok(Status::Initial));
//! assert_eq!(Status::try_from(200), Ok(Status::Success));
//! assert_eq!(Status::try_from(304), Ok(Status::Success));
//! assert!(Status::try_from(301).is_err());
//! ```
//!
//! # Bitflags
//!
//! Use `#[better_enums(bitflags)]` for independent flags. Bitflags must use an
//! unsigned representation, and each variant must be a unit variant with one
//! non-zero power-of-two value.
//!
//! ```
//! use better_enums::{better_enums, flags::TryFromBits};
//!
//! #[better_enums(bitflags)]
//! #[repr(u8)]
//! enum Permission {
//!     Read = 0b0000_0001,
//!     Write = 0b0000_0010,
//!     Execute = 0b0000_0100,
//! }
//!
//! let permissions = Permission::Read | Permission::Write;
//! assert!(permissions.contains(Permission::Read));
//! assert_eq!(permissions.value(), 0b0000_0011);
//! assert!(Permission::try_from_bits(0b0000_1000).is_err());
//! ```
//!
//! When every bit in the representation is defined, import
//! [`flags::FromBits`] and use the infallible `from_bits` conversion instead.
//! [`flags::Bitflags`] provides `contains`, `is_empty`, `is_full`, and `value`.
//!
//! Mappings must be non-empty, within the representation's range, and cannot
//! overlap. Bitflags additionally reject signed representations, zero or
//! non-power-of-two values, duplicate values, ranges, and arrays.

pub mod error;
pub use better_enums_derive::better_enums;

pub mod flags;

pub extern crate self as better_enums;
