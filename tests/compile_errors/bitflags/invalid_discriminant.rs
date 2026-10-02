use better_enums::better_enums;

#[better_enums(bitflags)]
#[repr(u8)]
enum InvalidDiscriminant {
    Value = "value",
}

fn main() {}