# better_enums

Adds more advanced fetures to enums

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
better_enums = "0.2"
```

### Range-based enums

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

### Bitflags

Annotate an enum with an integer representation and the `better_enums`
attribute:

```rust
use better_enums::better_enums;

#[better_enums(bitflags)]
#[repr(u8)]
enum OneThroughSixteen {
    One = 0b0000_0001,
    Two = 0b0000_0010,
    Four = 0b0000_0100,
    Eight = 0b0000_1000,
    Sixteen = 0b0001_0000,
}

assert!(matches!(OneThroughSixteen::try_from(0b0000_0011), Ok(OneThroughSixteen::Two | OneThroughSixteen::One)));
assert!(OneThroughSixteen::try_from(0b0001_0001).is_err());
```

If the enum is exhaustive, you can use `FromBits`
```Rust
#[better_enums(bitflags)]
#[repr(u8)]
enum ExhausiveFlags {
    One = 0b0000_0001,
    Two = 0b0000_0010,
    Four = 0b0000_0100,
    Eight = 0b0000_1000,
    Sixteen = 0b0001_0000,
    ThirtyTwo = 0b0010_0000,
    SixtyFour = 0b0100_0000,
    OneHundredTwentyEight = 0b1000_0000,
}

assert!(matches!(ExhausiveFlags::from_bits(0b0000_0011), ExhausiveFlags::Two | ExhausiveFlags::One));
```
Note: `FromBits` and `TryFromBits` 

Failed conversions return `BetterEnumsError<T>`.
