use better_enums::better_enums;

#[better_enums]
#[repr(u8)]
enum EmptyRange {
    Value = 10..10,
}

fn main() {}