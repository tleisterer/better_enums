use better_enums::better_enums;

#[better_enums]
#[repr(u8)]
enum InvalidDiscriminant {
    Value = "value",
}

fn main() {}