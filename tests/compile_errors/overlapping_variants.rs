use better_enums::better_enums;

#[better_enums]
#[repr(u8)]
enum OverlappingVariants {
    First = 1..10,
    Second = 9..20,
}

fn main() {}