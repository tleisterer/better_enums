use better_enums::better_enums;

#[better_enums]
#[repr(u64)]
enum Large {
    Everything = 0..=18446744073709551615,
}

#[better_enums]
#[repr(u8)]
enum Full {
    Everything = ..,
}

#[better_enums]
#[repr(u8)]
enum Mixed {
    First,
    Group = [5..10, 20, 30..=32, 11],
    Last,
}

#[better_enums]
#[repr(i8)]
enum Signed {
    Negative = ..0,
    Zero,
    Positive = 1..,
}

#[better_enums]
#[repr(i8)]
enum StartsAtZero {
    First,
    Second,
}

#[better_enums]
#[repr(u8)]
enum ReverseDisjointRanges {
    Higher = 5..10,
    Lower = 1..5,
}

#[better_enums]
#[repr(i128)]
enum MinimumSignedValue {
    Minimum = -170_141_183_460_469_231_731_687_303_715_884_105_728i128,
}

#[test]
fn large_ranges_are_checked_symbolically() {
    assert!(matches!(Large::try_from(0), Ok(Large::Everything)));
    assert!(matches!(Large::try_from(u64::MAX), Ok(Large::Everything)));
}

#[test]
fn fully_unbounded_ranges_reach_repr_maximum() {
    assert!(matches!(Full::try_from(0), Ok(Full::Everything)));
    assert!(matches!(Full::try_from(u8::MAX), Ok(Full::Everything)));
}

#[test]
fn arrays_and_implicit_values_work() {
    assert!(matches!(Mixed::try_from(0), Ok(Mixed::First)));
    assert!(matches!(Mixed::try_from(5), Ok(Mixed::Group)));
    assert!(matches!(Mixed::try_from(9), Ok(Mixed::Group)));
    assert!(matches!(Mixed::try_from(20), Ok(Mixed::Group)));
    assert!(matches!(Mixed::try_from(32), Ok(Mixed::Group)));
    assert!(matches!(Mixed::try_from(33), Ok(Mixed::Last)));
    assert!(Mixed::try_from(10).is_err());
}

#[test]
fn implicit_values_start_at_zero() {
    assert!(matches!(StartsAtZero::try_from(0), Ok(StartsAtZero::First)));
    assert!(matches!(StartsAtZero::try_from(1), Ok(StartsAtZero::Second)));
}

#[test]
fn disjoint_ranges_can_be_declared_in_reverse_order() {
    assert!(matches!(ReverseDisjointRanges::try_from(1), Ok(ReverseDisjointRanges::Lower)));
    assert!(matches!(ReverseDisjointRanges::try_from(5), Ok(ReverseDisjointRanges::Higher)));
}

#[test]
fn minimum_signed_value_is_valid() {
    assert!(matches!(
        MinimumSignedValue::try_from(i128::MIN),
        Ok(MinimumSignedValue::Minimum)
    ));
}

#[test]
fn signed_ranges_work() {
    assert!(matches!(Signed::try_from(-3), Ok(Signed::Negative)));
    assert!(matches!(Signed::try_from(i8::MAX), Ok(Signed::Positive)));
    assert!(matches!(Signed::try_from(-1), Ok(Signed::Negative)));
    assert!(matches!(Signed::try_from(0), Ok(Signed::Zero)));
    assert!(matches!(Signed::try_from(3), Ok(Signed::Positive)));
}

#[test]
fn error_value_is_exposed() {
    let error = match Mixed::try_from(99) {
        Ok(_) => panic!("99 should not convert"),
        Err(error) => error,
    };
    assert_eq!(format!("{}", error), "99 is not a valid discriminant");
    assert!(error.to_string().contains("not a valid discriminant"));
}

#[test]
fn errors_are_enum_specific() {
    let error = match Mixed::try_from(99) {
        Ok(_) => panic!("99 should not convert"),
        Err(error) => error,
    };
    assert!(error.to_string().contains("not a valid discriminant"));
}
