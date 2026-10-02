use better_enums::better_enums;

#[better_enums]
#[repr(u8)]
enum ImplicitValuesExhausted {
    Last = 255,
    Next,
}

fn main() {}