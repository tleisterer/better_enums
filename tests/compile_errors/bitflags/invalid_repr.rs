use better_enums::better_enums;

#[better_enums(bitflags)]
#[repr(i8)]
enum InvalidRepr {
    Value,
}

fn main() {}