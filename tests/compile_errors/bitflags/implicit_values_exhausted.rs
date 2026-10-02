use better_enums::better_enums;

#[better_enums(bitflags)]
#[repr(u8)]
enum ImplicitValuesExhausted {
    Last = 0b10000000,
    Next,
}

fn main() {}