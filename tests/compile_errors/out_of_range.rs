use better_enums::better_enums;

#[better_enums]
#[repr(u8)]
enum OutOfRange {
    Value = 256,
}

fn main() {}