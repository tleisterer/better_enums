use better_enums::better_enums;

#[better_enums]
#[repr(u8)]
enum EmptyMapping {
    Value = [],
}

fn main() {}