use better_enums::better_enums;

#[better_enums]
#[repr(u8)]
enum test {
    a = 5..1
}

fn main() {}