use better_enums::better_enums;

#[better_enums]
#[repr(u8)]
enum NegativeUnsigned {
    Value = -1,
}

fn main() {}