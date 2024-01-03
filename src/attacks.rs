use crate::consts::{
    Colour,
};

static mut PAWN_TABLES: [u64; 128] = [0; 128];

pub struct Attacks;
impl Attacks {
   pub fn init_tables() {
        unsafe {
            //pawn tables
            for i in 0..64 {
                PAWN_TABLES[i] = Self::slow_pawn_attacks(i, 0);
                PAWN_TABLES[i + 64] = Self::slow_pawn_attacks(i, 1);
            }
        }
    }
    pub fn pawn_attacks(square : usize, colour_num : usize) -> u64 {
        unsafe { PAWN_TABLES[square + (64 * colour_num)] }
    }
    fn slow_pawn_attacks(square : usize, colour_num : usize) -> u64 {
        assert!(colour_num == 1 || colour_num == 0);

        let colour = if colour_num == 0 {Colour::White} else {Colour::Black};

        let square_diagonal_left = square as i16 + if colour == Colour::White {7} else {-9} as i16;
        let square_diagonal_right = square as i16 + if colour == Colour::White {9} else {-7} as i16;

        if colour == Colour::Black && square / 8 == 0 || colour == Colour::White && square / 8 == 7{
            return 0u64
        }

        let file = square % 8;

        match file {
            0 => 1u64 << square_diagonal_right,
            7 => 1u64 << square_diagonal_left,
            _ => (1u64 << square_diagonal_left) | (1u64 << square_diagonal_right),
        }

    }

    
}   