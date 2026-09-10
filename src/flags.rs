use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

use crate::error::BetterEnumsError;

pub use traits::*;

macro_rules! for_each_number_primitive {
    ($d:tt $name:ident => $($code:tt)*) => {
        macro_rules! implement {
            ($d $name:ty) => {
                $($code)*
            };
        }

        implement!(u8);
        implement!(u16);
        implement!(u32);
        implement!(u64);
        implement!(u128);
        implement!(usize);
        implement!(i8);
        implement!(i16);
        implement!(i32);
        implement!(i64);
        implement!(i128);
        implement!(isize);
    };
}

mod traits {

    use super::*;

    #[doc(hidden)]
    /// Used to mark enums as bitflags
    /// This trait is not intended to be implemented by users.
    pub trait Bit: Sized {
        type Repr: Copy
            + Clone
            + Sized
            + BitAnd<Self::Repr, Output = Self::Repr>
            + BitOr<Self::Repr, Output = Self::Repr>
            + BitXor<Self::Repr, Output = Self::Repr>
            + Shl<u8, Output = Self::Repr>
            + Shl<u16, Output = Self::Repr>
            + Shl<u32, Output = Self::Repr>
            + Shl<u64, Output = Self::Repr>
            + Shl<u128, Output = Self::Repr>
            + Shl<usize, Output = Self::Repr>
            + Shl<i8, Output = Self::Repr>
            + Shl<i16, Output = Self::Repr>
            + Shl<i32, Output = Self::Repr>
            + Shl<i64, Output = Self::Repr>
            + Shl<i128, Output = Self::Repr>
            + Shl<isize, Output = Self::Repr>
            + Shr<u8, Output = Self::Repr>
            + Shr<u16, Output = Self::Repr>
            + Shr<u32, Output = Self::Repr>
            + Shr<u64, Output = Self::Repr>
            + Shr<u128, Output = Self::Repr>
            + Shr<usize, Output = Self::Repr>
            + Shr<i8, Output = Self::Repr>
            + Shr<i16, Output = Self::Repr>
            + Shr<i32, Output = Self::Repr>
            + Shr<i64, Output = Self::Repr>
            + Shr<i128, Output = Self::Repr>
            + Shr<isize, Output = Self::Repr>
            + Not<Output = Self::Repr>
            + PartialEq
            + Eq
            + PartialOrd
            + Ord
            + 'static;

        /// The value of all enum variants combined.
        const FULL: Self::Repr;
        /// The value of no enum variants combined.
        const EMPTY: Self::Repr;

        /// Returns the underlying value of the enum variant.
        fn value(&self) -> Self::Repr;

        /// Returns an empty set of flags.
        #[inline(always)]
        fn empty() -> Bitflags<Self> {
            Bitflags(Self::EMPTY)
        }

        /// Returns a set of flags with all possible flags set.
        #[inline(always)]
        fn full() -> Bitflags<Self> {
            Bitflags(Self::FULL)
        }
    }

    #[doc(hidden)]
    /// A type that represents a set of flags for a given enum.
    /// This trait is not intended to be implemented by users, but is used to mark enums that have all possible flags defined.
    pub trait ExhaustiveBit: Bit {}

    /// Used to do Bit::Repr-to-Bitflags conversions
    ///
    /// **Note: This trait must not fail**. The `FromBits` trait is intended for perfect conversions.
    /// If the enum is not exhaustive, use [`TryFromBits`].
    pub trait FromBits: Sized + ExhaustiveBit {
        /// Converts the underlying representation into a set of flags.
        #[must_use]
        fn from_bits(value: Self::Repr) -> Bitflags<Self>;
    }

    /// Simple and safe Bit::Repr-to-Bitflags conversion that may fail in a controlled
    /// way under some circumstances.
    ///
    /// Note: If the enum is exhaustive, use [`FromBits`] instead.
    pub trait TryFromBits: Sized + Bit {
        /// The type returned in the event of a conversion error.
        type Error;

        /// Performs the conversion.
        fn try_from_bits(value: Self::Repr) -> Result<Bitflags<Self>, Self::Error>;
    }
}

/// A type that represents a set of flags for a given enum.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Bitflags<T: Bit>(T::Repr);

impl<T: Bit> Bitflags<T> {
    pub fn contains(&self, flag: T) -> bool {
        self.0 & flag.value() == flag.value()
    }

    pub fn is_empty(&self) -> bool {
        self.0 == T::EMPTY
    }

    pub fn is_full(&self) -> bool {
        self.0 == T::FULL
    }

    pub fn value(&self) -> T::Repr {
        self.0
    }
}

impl<T: ExhaustiveBit> FromBits for T {
    fn from_bits(value: Self::Repr) -> Bitflags<Self> {
        Bitflags(value)
    }
}

impl<T: Bit> TryFromBits for T {
    type Error = BetterEnumsError<T::Repr>;

    fn try_from_bits(value: Self::Repr) -> Result<Bitflags<Self>, Self::Error> {
        if T::FULL & value == value {
            Ok(Bitflags(value))
        } else {
            Err(BetterEnumsError::Bit(value))
        }
    }
}

impl<T: Bit> BitAnd<T> for Bitflags<T> {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: T) -> Self::Output {
        Self(self.0.bitand(rhs.value()))
    }
}

impl<T: Bit> BitOr<T> for Bitflags<T> {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: T) -> Self::Output {
        Self(self.0.bitor(rhs.value()))
    }
}

impl<T: Bit> BitXor<T> for Bitflags<T> {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: T) -> Self::Output {
        Self(self.0.bitxor(rhs.value()))
    }
}

impl<T: Bit> BitAndAssign<T> for Bitflags<T> {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: T) -> () {
        self.0 = self.0.bitand(rhs.value());
    }
}

impl<T: Bit> BitOrAssign<T> for Bitflags<T> {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: T) -> () {
        self.0 = self.0.bitor(rhs.value());
    }
}

impl<T: Bit> BitXorAssign<T> for Bitflags<T> {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: T) -> () {
        self.0 = self.0.bitxor(rhs.value());
    }
}

impl<T: ExhaustiveBit> Not for Bitflags<T> {
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(self.0.not())
    }
}

for_each_number_primitive!($type =>
    impl<T: ExhaustiveBit> Shr<$type> for Bitflags<T> {
        type Output = Self;
        #[inline(always)]
        fn shr(self, rhs: $type) -> Self::Output {
            Self(self.0.shr(rhs))
        }
    }

    impl<T: ExhaustiveBit> Shl<$type> for Bitflags<T> {
        type Output = Self;
        #[inline(always)]
        fn shl(self, rhs: $type) -> Self::Output {
            Self(self.0.shl(rhs))
        }
    }
);
