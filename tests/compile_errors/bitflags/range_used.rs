use better_enums::better_enums;

#[better_enums(bitflags)]
#[repr(u8)]
enum DataVariant {
    Value = 1..5,
}

fn main() {}