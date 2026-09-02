use better_enums::better_enums;

#[better_enums]
#[repr(u8)]
enum RangeEndpointOutOfRange {
    Value = 0..=256,
}

fn main() {}