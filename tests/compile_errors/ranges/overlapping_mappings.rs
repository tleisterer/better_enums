use better_enums::better_enums;

#[better_enums]
#[repr(u8)]
enum OverlappingMappings {
    Value = [1, 1],
}

fn main() {}