use better_enums::better_enums;

#[better_enums]
#[repr(i128)]
enum SignedLiteralTooLarge {
    Value = 170141183460469231731687303715884105728,
}

fn main() {}