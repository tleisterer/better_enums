# better_enums

Generate `TryFrom` implementations for enums whose variants represent single
integer values or ranges of values.

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
better_enums = "0.1"
```

Annotate an enum with an integer representation and the `better_enums`
attribute:

```rust
use better_enums::better_enums;

#[better_enums]
#[repr(u16)]
enum HttpStatus {
    Ok = 200,
    ClientError = 400..500,
    ServerError = 500..=599,
}

assert!(matches!(HttpStatus::try_from(200), Ok(HttpStatus::Ok)));
assert!(matches!(HttpStatus::try_from(404), Ok(HttpStatus::ClientError)));
assert!(HttpStatus::try_from(302).is_err());
```

The macro supports single values, inclusive and exclusive ranges, unbounded
ranges, and arrays combining these forms:

```rust
#[better_enums]
#[repr(i8)]
enum Number {
    Negative = ..0,
    Zero,
    Positive = 1..,
}
```

Variants without an explicit mapping receive the next available value,
starting at zero. Mappings must be non-empty, fit the enum representation, and
must not overlap. Generic enums are not supported.

Failed conversions return `BetterEnumsError<T>`, which retains the original
value and displays as `"<value> is not a valid discriminant"`.

## License

Licensed under either of:

* Apache License, Version 2.0
* MIT License

at your option.