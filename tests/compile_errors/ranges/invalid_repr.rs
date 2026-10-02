use better_enums::better_enums;

#[better_enums]
#[repr(bool)]
enum InvalidRepr {
    Value,
}

fn main() {}