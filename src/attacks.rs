use crate::consts::{
    Mask
};

pub struct Attacks;
impl Attacks {

    // SHIFTS
    const fn shift_north(bitboard : u64) -> u64 {
        bitboard << 8
    }

    const fn shift_south(bitboard : u64) -> u64 {
        bitboard >> 8
    }

    const fn shift_east(bitboard : u64) -> u64 {
        (bitboard << 1) & !Mask::FILE_A_MASK
    }

    const fn shift_west(bitboard : u64) -> u64 {
        (bitboard >> 1) & !Mask::FILE_H_MASK   
    }

    // LOOKUP TABLES
    pub fn pawn_attacks(square : usize, colour_num : usize) -> u64 {
        PAWN_TABLES[square + (64 * colour_num)]
    }

    pub fn knight_attacks(square : usize) -> u64 {
        KNIGHT_TABLES[square]
    }

    pub fn king_attacks(square : usize) -> u64 {
        KING_TABLES[square]
    }

    // ATTACK FUNCTIONS
    const fn slow_pawn_attacks(square : usize, colour_num : usize) -> u64 {
        assert!(colour_num == 1 || colour_num == 0);

        let pawn = 1 << square;
        let pawn_push = if colour_num == 0 { Self::shift_north(pawn) } else { Self::shift_south(pawn) };

        Self::shift_west(pawn_push) | Self::shift_east(pawn_push)
    }

    const fn slow_knight_attacks(square : usize) -> u64 {
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

    const fn slow_king_attacks(square : usize) -> u64 {
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

    pub const fn slow_rook_attacks(square: usize, blockers: u64, calculating_occupancy: bool) -> u64 {
        let rook = 1 << square;
        let mut attacks = 0;
    
        let mut test_square = rook;
    
        while blockers & test_square == 0 && test_square > 0 {
            test_square = Self::shift_north(test_square);
            attacks |= test_square;
        }
    
        // mask out appropriate file/rank if calculating_occupancy
        if calculating_occupancy {
            attacks &= !Mask::RANK_8_MASK;
        }
        test_square = rook;
    
        while blockers & test_square == 0 && test_square > 0 {
            test_square = Self::shift_south(test_square);
            attacks |= test_square;
        }
    
        if calculating_occupancy {
            attacks &= !Mask::RANK_1_MASK;
        }
        test_square = rook;
    
        while blockers & test_square == 0 && test_square > 0 {
            test_square = Self::shift_west(test_square);
            attacks |= test_square;
        }
    
        if calculating_occupancy {
            attacks &= !Mask::FILE_A_MASK;
        }
        test_square = rook;
    
        while blockers & test_square == 0 && test_square > 0 {
            test_square = Self::shift_east(test_square);
            attacks |= test_square;
        }
    
        if calculating_occupancy {
            attacks &= !Mask::FILE_H_MASK;
        }
    
        attacks
    }
    
    pub const fn slow_bishop_attacks(square : usize, blockers : u64, calculating_occupancy : bool) -> u64 {
        let bishop = 1 << square;
        let mut attacks = 0;

        let mut test_square = bishop;

        while blockers & test_square == 0 && test_square > 0 {
            test_square = Self::shift_north(Self::shift_east(test_square));
            attacks |= test_square; 
        } 

        if calculating_occupancy {
            attacks &= !(Mask::FILE_H_MASK | Mask::RANK_8_MASK);
        }
        test_square = bishop;

        while blockers & test_square == 0 && test_square > 0 {
            test_square = Self::shift_south(Self::shift_east(test_square));
            attacks |= test_square; 
        } 

        if calculating_occupancy {
            attacks &= !(Mask::FILE_H_MASK | Mask::RANK_1_MASK);
        }
        test_square = bishop;

        while blockers & test_square == 0 && test_square > 0 {
            test_square = Self::shift_north(Self::shift_west(test_square));
            attacks |= test_square; 
        } 

        if calculating_occupancy {
            attacks &= !(Mask::FILE_A_MASK | Mask::RANK_8_MASK);
        }    
        test_square = bishop;

        while blockers & test_square == 0 && test_square > 0 {
            test_square = Self::shift_south(Self::shift_west(test_square));
            attacks |= test_square; 
        } 

        if calculating_occupancy {
            attacks &= !(Mask::FILE_A_MASK | Mask::RANK_1_MASK);
        }

        attacks
    }
}   

const PAWN_TABLES: [u64; 128] = {
    let mut table = [0; 128];
    let mut i = 0;
    while i < (table.len() - 64) {
        table[i] = Attacks::slow_pawn_attacks(i, 0);
        table[i + 64] = Attacks::slow_pawn_attacks(i, 1);

        i += 1
    }

    table
};

const KNIGHT_TABLES: [u64; 64] = {
    let mut table = [0; 64];
    let mut i = 0;
    while i < table.len() {
        table[i] = Attacks::slow_knight_attacks(i);

        i += 1;
    }

    table
};

const KING_TABLES: [u64; 64] = {
    let mut table = [0; 64];
    let mut i = 0;
    while i < table.len() {
        table[i] = Attacks::slow_king_attacks(i);

        i += 1
    }

    table
};