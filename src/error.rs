//! Errors returned when an integer does not map to an enum variant.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BetterEnumsError<T> {
    Discriminant(T),
    Bit(T),
}

impl<T: std::fmt::Display> std::fmt::Display for BetterEnumsError<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bit(t) => write!(formatter, "{} contains an invalid bit", t),
            Self::Discriminant(t) => write!(formatter, "{} is not a valid discriminant", t),
        }
    }
}

impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for BetterEnumsError<T> {}
