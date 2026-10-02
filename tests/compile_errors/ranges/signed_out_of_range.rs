use better_enums::better_enums;

#[better_enums]
#[repr(i8)]
enum SignedOutOfRange {
    Value = 128,
}

fn main() {}