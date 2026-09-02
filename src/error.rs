#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BetterEnumsError<T> {
    pub(crate) value: T,
}

impl<T> BetterEnumsError<T> {
    pub fn new(value: T) -> Self {
        BetterEnumsError { value }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for BetterEnumsError<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} is not a valid discriminant", self.value)
    }
}

impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for BetterEnumsError<T> {}
