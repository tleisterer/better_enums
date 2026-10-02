use better_enums::better_enums;

#[better_enums]
#[repr(i128)]
enum SignedLiteralTooSmall {
    Value = -170141183460469231731687303715884105729,
}

fn main() {}