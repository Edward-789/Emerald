static mut PAWN_TABLES: [u64; 128] = [0; 128];
static mut KNIGHT_TABLES: [u64; 64] = [0; 64];
static mut KING_TABLES: [u64; 64] = [0; 64];


pub struct Attacks;
impl Attacks {
   pub fn init_tables() {
        unsafe {
            for i in 0..64 {
                //pawn tables
                PAWN_TABLES[i] = Self::slow_pawn_attacks(i, 0);
                PAWN_TABLES[i + 64] = Self::slow_pawn_attacks(i, 1);

                //knight tables
                KNIGHT_TABLES[i] = Self::slow_knight_attacks(i);

                //king tables
                KING_TABLES[i] = Self::slow_king_attacks(i)
            }
        }
    }

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

    pub fn pawn_attacks(square : usize, colour_num : usize) -> u64 {
        unsafe { PAWN_TABLES[square + (64 * colour_num)] }
    }

    pub fn knight_attacks(square : usize) -> u64 {
        unsafe { KNIGHT_TABLES[square] }
    }

    pub fn king_attacks(square : usize) -> u64 {
        unsafe { KING_TABLES[square] }
    }

    fn slow_pawn_attacks(square : usize, colour_num : usize) -> u64 {
        assert!(colour_num == 1 || colour_num == 0);

        let pawn = 1 << square;
        let pawn_push = if colour_num == 0 { Self::shift_north(pawn) } else { Self::shift_south(pawn) };

        Self::shift_west(pawn_push) | Self::shift_east(pawn_push)
    }

    fn slow_knight_attacks(square : usize) -> u64 {
        let knight = 1 << square;

        Self::shift_north(Self::shift_north(Self::shift_east(knight))) |
        Self::shift_north(Self::shift_north(Self::shift_west(knight))) |
        Self::shift_east(Self::shift_east(Self::shift_north(knight))) |
        Self::shift_east(Self::shift_east(Self::shift_south(knight))) |
        Self::shift_south(Self::shift_south(Self::shift_east(knight))) |
        Self::shift_south(Self::shift_south(Self::shift_west(knight))) |
        Self::shift_west(Self::shift_west(Self::shift_north(knight))) |
        Self::shift_west(Self::shift_west(Self::shift_south(knight)))
    }

    fn slow_king_attacks(square : usize) -> u64 {
        let king = 1 << square;

        Self::shift_east(king) |
        Self::shift_west(king) |
        Self::shift_north(king) |
        Self::shift_south(king) |
        Self::shift_east(Self::shift_north(king)) |
        Self::shift_east(Self::shift_south(king)) |
        Self::shift_west(Self::shift_north(king)) |
        Self::shift_west(Self::shift_south(king))
    }
}   