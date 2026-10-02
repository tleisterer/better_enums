use better_enums::{
    better_enums,
    flags::{Bit, FromBits, TryFromBits},
};

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
#[better_enums(bitflags)]
#[allow(unused)]
enum ExhaustiveBitflag {
    One = 0b0000_0001,
    Two = 0b0000_0010,
    Four = 0b0000_0100,
    Eight = 0b0000_1000,
    Sixteen = 0b0001_0000,
    ThirtyTwo = 0b0010_0000,
    SixtyFour = 0b0100_0000,
    OneTwentyEight = 0b1000_0000,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
#[better_enums(bitflags)]
#[allow(unused)]
enum NonExhaustiveBitflag {
    One = 0b0000_0001,
    Two = 0b0000_0010,
    Four = 0b0000_0100,
    Eight = 0b0000_1000,
}

#[test]
fn test_bitflags() {
    let flags = ExhaustiveBitflag::One | ExhaustiveBitflag::Two;
    let t = 0b0000_0011;
    assert_eq!(flags, ExhaustiveBitflag::from_bits(t));

    let flags = NonExhaustiveBitflag::One | NonExhaustiveBitflag::Two;
    let t = 0b0000_0011;
    assert_eq!(Ok(flags), NonExhaustiveBitflag::try_from_bits(t))
}

#[test]
fn test_shift() {
    let flags = ExhaustiveBitflag::from_bits(ExhaustiveBitflag::FULL);
    let mut shift = flags << 5;

    assert_eq!(shift, ExhaustiveBitflag::from_bits(0b1110_0000));
    shift >>= 5;
}

#[test]
fn test_bitflags_contains() {
    let flags = ExhaustiveBitflag::One | ExhaustiveBitflag::Two | ExhaustiveBitflag::Four;
    assert!(flags.contains(ExhaustiveBitflag::One));
    assert!(flags.contains(ExhaustiveBitflag::Two));
    assert!(flags.contains(ExhaustiveBitflag::Four));
    assert!(!flags.contains(ExhaustiveBitflag::Eight));

    let flags = NonExhaustiveBitflag::One | NonExhaustiveBitflag::Two | NonExhaustiveBitflag::Four;
    assert!(flags.contains(NonExhaustiveBitflag::One));
    assert!(flags.contains(NonExhaustiveBitflag::Two));
    assert!(flags.contains(NonExhaustiveBitflag::Four));
    assert!(!flags.contains(NonExhaustiveBitflag::Eight));
}

#[test]
fn test_bitflags_is_empty() {
    let mut flags = ExhaustiveBitflag::empty();
    assert!(flags.is_empty());
    assert!(!flags.is_full());
    assert_eq!(flags.value(), 0);

    flags = !flags;
    assert!(!flags.is_empty());
    assert!(flags.is_full());
    assert_eq!(flags.value(), !0);
    assert_eq!(flags.value(), ExhaustiveBitflag::FULL);
    assert_eq!(flags, ExhaustiveBitflag::full());

    let flags = NonExhaustiveBitflag::empty();
    assert!(flags.is_empty());
    assert!(!flags.is_full());
    assert_eq!(flags.value(), 0);

    let flags = NonExhaustiveBitflag::full();
    assert!(!flags.is_empty());
    assert!(flags.is_full());
    assert_eq!(flags.value(), 1 | 2 | 4 | 8);
}
