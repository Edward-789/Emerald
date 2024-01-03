pub struct shifts;
impl shifts {
    pub fn shift_north(bitboard : u64) -> u64 {
        bitboard << 8
    }

    pub fn shift_south(bitboard : u64) -> u64 {
        bitboard >> 8
    }

    pub fn shift_east(bitboard : u64) -> u64 {
        (bitboard << 1) & !0x0101010101010101 // A-FILE
    }

    pub fn shift_west(bitboard : u64) -> u64 {
        (bitboard >> 1) & !0x8080808080808080 //H-FILE
    }
}
