# better_enums
`better_enums` generates conversions support for enums whose variants map to
integer values, ranges, or bitflags.

## Usage

The enum must have a primitive integer representation and unit variants. The
supported representations are `u8`, `u16`, `u32`, `u64`, `u128`, `usize`,
`i8`, `i16`, `i32`, `i64`, `i128`, and `isize`. Generic enums are not
supported.

# Ranges

Use `#[better_enums]` or `#[better_enums(range)]` to generate a
`TryFrom<repr>` implementation:

```rust
use better_enums::better_enums;

#[better_enums]
#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
enum HttpStatus {
    Ok = 200,
    ClientError = 400..500,
    ServerError = 500..=599,
}

assert_eq!(HttpStatus::try_from(200), Ok(HttpStatus::Ok));
assert_eq!(HttpStatus::try_from(404), Ok(HttpStatus::ClientError));
assert!(HttpStatus::try_from(302).is_err());
```

An integer literal maps one value to a variant. Variants without an explicit
mapping receive the next available value, starting at zero. Mappings can also
contain inclusive ranges (`10..=20`), exclusive ranges (`10..20`), unbounded
ranges (`..10`, `10..`, or `..`), and arrays combining numbers and ranges:

```rust
use better_enums::better_enums;

#[better_enums]
#[repr(i8)]
enum Number {
    Negative = ..0,
    Zero,
    Positive = 1..,
}
```

An array maps every listed number or range to the same variant:

```rust
use better_enums::better_enums;

#[better_enums]
#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
enum Status {
    Initial,
    Success = [200..300, 304],
}

assert_eq!(Status::try_from(0), Ok(Status::Initial));
assert_eq!(Status::try_from(200), Ok(Status::Success));
assert_eq!(Status::try_from(304), Ok(Status::Success));
assert!(Status::try_from(301).is_err());
```

# Bitflags

Use `#[better_enums(bitflags)]` for a set of independent flags. Bitflags must
use an unsigned representation, and every variant must be a unit variant with
one non-zero power-of-two value. Ranges and arrays are not supported in this
mode.

Omitted discriminants are assigned automatically: if the first variant
omits its value, it defaults to `1`; each later omitted value is twice the
preceding variant's value.

```rust
use better_enums::{better_enums, flags::{TryFromBits, Bit}};

#[better_enums(bitflags)]
#[repr(u8)]
#[derive(Debug, PartialEq)]
enum Permission {
    Read = 0b0000_0001,
    Write = 0b0000_0010,
    Execute = 0b0000_0100,
}

let permissions = Permission::Read | Permission::Write;
assert!(permissions.contains(Permission::Read));
assert_eq!(permissions.value(), 0b0000_0011);
assert_eq!(Permission::try_from_bits(0b0000_0111),
    Ok(Permission::Read | Permission::Write | Permission::Execute));
assert!(Permission::try_from_bits(0b0000_1000).is_err());
```

When the enum defines every bit in its representation, you can use the
infallible `from_bits` version:

```rust
use better_enums::{better_enums, flags::{FromBits, Bit}};

#[better_enums(bitflags)]
#[repr(u8)]
enum ExhaustiveFlags {
    One = 0b0000_0001,
    Two = 0b0000_0010,
    Four = 0b0000_0100,
    Eight = 0b0000_1000,
    Sixteen = 0b0001_0000,
    ThirtyTwo = 0b0010_0000,
    SixtyFour = 0b0100_0000,
    OneHundredTwentyEight = 0b1000_0000,
}

let flags = ExhaustiveFlags::from_bits(0b0000_0011);
assert!(flags.contains(ExhaustiveFlags::One));
assert_eq!(flags.value(), 0b0000_0011);
```

## Validation

The macro rejects missing or non-integer representations, variants with data,
generic enums, invalid or out-of-range values, empty or reversed ranges,
overlapping mappings, and exhausted implicit values. Bitflags additionally
reject signed representations, zero or non-power-of-two values, duplicate
values, ranges, and arrays.

## Features
- default: none

- rename: Adds the possibility to rename the better_enums crate, but adds
  `proc-macro-crate` as an additional dependency.

## Known Limitations

- Mapping values must currently be integer literals. Named constants cannot be
  assigned as mappings.
- Mapping values cannot currently contain compile-time calculations. For
  example, `Value = 2 + 2` is rejected; write `Value = 4` instead.
- Range-based enums always implement `TryFrom<repr>`, even when their mappings
  cover the complete representation. They do not currently implement
  `From<repr>`.
- Enum variants cannot be shifted (`<<` or `>>`) directly; convert one to a
  `Bitflags` value with `Bitflags::from(variant)` first. Shifting is available
  only for exhaustive bitflags.
- The value used in `as` casts cannot be modified (`Enum::Variant as u8`);
  it is always the first element (the smaller number in case of ranges).

<!-- 
## Plans

- Parse and validate integer arithmetic and bitwise
  expressions at macro expansion time.
- Generate `From<repr>` for range-based enums whose mappings cover every value
  in the representation. Non-exhaustive mappings would continue to use
  `TryFrom<repr>`.
- Add a way to configure the value used in `as` casts, for example with a
  submacro such as `#[default = 15]`.
-->
