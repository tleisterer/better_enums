use better_enums::better_enums;

#[better_enums]
#[repr(u8)]
enum DataVariant {
    Value(u8),
}

fn main() {}