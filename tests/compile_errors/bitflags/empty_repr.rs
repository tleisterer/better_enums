use better_enums::better_enums;

#[better_enums(bitflags)]
#[repr()]
enum EmptyRepr {
    Value,
}

fn main() {}