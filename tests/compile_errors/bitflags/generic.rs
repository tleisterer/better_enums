use better_enums::better_enums;

#[better_enums(bitflags)]
#[repr(u8)]
enum Generic<T> {
    Value,
}

fn main() {}