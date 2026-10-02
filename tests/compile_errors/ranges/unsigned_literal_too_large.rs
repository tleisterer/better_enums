use better_enums::better_enums;

#[better_enums]
#[repr(u128)]
enum UnsignedLiteralTooLarge {
    Value = 340282366920938463463374607431768211456,
}

fn main() {}