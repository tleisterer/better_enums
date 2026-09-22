use std::ops::BitOr;

use better_enums::flags::{Bit, Bitflags, ExhaustiveBit, FromBits};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(unused)]
#[repr(u8)]
enum Flag {
    One = 0b0000_0001,
    Two = 0b0000_0010,
    Four = 0b0000_0100,
    Eight = 0b0000_1000,
    Sixteen = 0b0001_0000,
    ThirtyTwo = 0b0010_0000,
    SixtyFour = 0b0100_0000,
    OneTwentyEight = 0b1000_0000,
}

impl BitOr for Flag {
    type Output = Bitflags<Self>;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from_bits(self.value() | rhs.value())
    }
}

impl Bit for Flag {
    type Repr = u8;

    const FULL: Self::Repr = 0b1111_1111;
    const EMPTY: Self::Repr = 0b0000_0000;

    fn value(&self) -> Self::Repr {
        *self as Self::Repr
    }
}

impl ExhaustiveBit for Flag {}

#[test]
fn test_bitflags() {
    let flags = Flag::One | Flag::Two;
    let t = 0b0000_0011;
    assert_eq!(flags, Flag::from_bits(t))
}

#[test]
fn test_shift() {
    let flags = Flag::from_bits(Flag::FULL);
    let shift = flags << 5;

    assert_eq!(shift, Flag::from_bits(0b1110_0000))
}

#[test]
fn test_bitflags_contains() {
    let flags = Flag::One | Flag::Two | Flag::Four;
    assert!(flags.contains(Flag::One));
    assert!(flags.contains(Flag::Two));
    assert!(flags.contains(Flag::Four));
    assert!(!flags.contains(Flag::Eight));
}

#[test]
fn test_bitflags_is_empty() {
    let flags = Flag::empty();
    assert!(flags.is_empty());
    assert!(!flags.is_full());
}
