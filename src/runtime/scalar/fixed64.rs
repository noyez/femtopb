crate::runtime::macros::fixed_width!(
    'a,
    u64,
    crate::item_encoding::Fixed64,
    8,
    encoding::WireType::SixtyFourBit,
    put_u64_le,
    get_u64_le
);
