    use crate::consts::{
        Colour,
    };

pub struct Attacks;
impl Attacks {
    pub fn slow_pawn_attacks(square : usize, colour_num : usize) -> u64 {
        assert!(colour_num == 1 || colour_num == 0);

        let colour = if colour_num == 0 {Colour::White} else {Colour::Black};

        let square_diagonal_left = square as i16 + if colour == Colour::White {7} else {-9} as i16;
        let square_diagonal_right = square as i16 + if colour == Colour::White {9} else {-7} as i16;

        if colour == Colour::Black && square / 8 == 0 || colour == Colour::White && square / 8 == 7{
            return 0u64
        }

        let file = square % 8;

        if file == 0 {
            return 1u64 << square_diagonal_right
        }

        if file == 7 {
            return 1u64 << square_diagonal_left 
        }

        (1u64 << square_diagonal_left) | (1u64 << square_diagonal_right)
    }
}   