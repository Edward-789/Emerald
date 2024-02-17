use crate::{
    magics::{
        BISHOP_MAGICS, ROOK_MAGICS
    }, utils::{
        Colour, Direction, Masks, Pieces
    } 
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
        (bitboard << 1) & !Masks::FILE_A
    }

    const fn shift_west(bitboard : u64) -> u64 {
        (bitboard >> 1) & !Masks::FILE_H
    }

    const fn shift_direction(bitboard : u64, direction : &Direction) -> u64 {
        match direction {
            Direction::North => Self::shift_north(bitboard),
            Direction::NorthEast => Self::shift_north(Self::shift_east(bitboard)),
            Direction::East => Self::shift_east(bitboard),
            Direction::SouthEast => Self::shift_south(Self::shift_east(bitboard)),
            Direction::South => Self::shift_south(bitboard),
            Direction::SouthWest => Self::shift_south(Self::shift_west(bitboard)),
            Direction::West => Self::shift_west(bitboard),
            Direction::NorthWest => Self::shift_north(Self::shift_west(bitboard)),
        }
    }

    pub const fn get_ray(square : usize, blockers : u64, direction : &Direction) -> u64 {
        let mut test_square = 1 << square;
        let mut attacks = 0;
        while blockers & test_square == 0 && test_square > 0 {
            test_square = Self::shift_direction(test_square, direction);
            attacks |= test_square; 
        } 

        attacks
    }
    
    // LOOKUP TABLES
    pub fn pawn_attacks(square : usize, colour : Colour) -> u64 {
        PAWN_TABLE[square + (64 * (colour as usize))]
    }

    pub fn knight_attacks(square : usize) -> u64 {
        KNIGHT_TABLE[square]
    }

    pub fn king_attacks(square : usize) -> u64 {
        KING_TABLE[square]
    }
    
    pub fn bishop_attacks(square : usize, blockers : u64) -> u64 {
        let magic = unsafe { BISHOP_MAGICS.get_unchecked(square) };

        let blockers_filtered = blockers & magic.mask;
        let index = (magic.factor.wrapping_mul(blockers_filtered) >> 55) as usize + magic.offset;
    
        unsafe { *SLIDER_ATTACKS.get_unchecked(index) }
    }

    pub fn rook_attacks(square : usize, blockers : u64) -> u64 {
        let magic = unsafe { ROOK_MAGICS.get_unchecked(square) };

        let blockers_filtered = blockers & magic.mask;
        let index = (magic.factor.wrapping_mul(blockers_filtered) >> 52) as usize + magic.offset;
    
        unsafe { *SLIDER_ATTACKS.get_unchecked(index) }
    }

    pub fn queen_attacks(square : usize, blockers : u64) -> u64 {
        Self::rook_attacks(square, blockers) | Self::bishop_attacks(square, blockers)
    }

    
    pub fn get_piece_attacks(square : usize, blockers : u64 , piece: Pieces) -> u64 {
        match piece {
            Pieces::Rook => Attacks::rook_attacks(square, blockers),
            Pieces::Bishop => Attacks::bishop_attacks(square, blockers),
            Pieces::King => Attacks::king_attacks(square),
            Pieces::Knight => Attacks::knight_attacks(square),
            Pieces::Queen => Attacks::queen_attacks(square, blockers),
            _ => 0
        }
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
    
    const fn slow_rook_attacks(square : usize, blockers : u64) -> u64 {        
        Self::get_ray(square, blockers, &Direction::North) | 
        Self::get_ray(square, blockers, &Direction::East) | 
        Self::get_ray(square, blockers, &Direction::West) | 
        Self::get_ray(square, blockers, &Direction::South)
    } 

    const fn slow_bishop_attacks(square : usize, blockers : u64) -> u64 {        
        Self::get_ray(square, blockers, &Direction::NorthEast) | 
        Self::get_ray(square, blockers, &Direction::SouthEast) | 
        Self::get_ray(square, blockers, &Direction::SouthWest) | 
        Self::get_ray(square, blockers, &Direction::NorthWest)
    } 
}       

const PAWN_TABLE: [u64; 128] = {
    let mut table = [0; 128];
    let mut i = 0;
    while i < (table.len() - 64) {
        table[i] = Attacks::slow_pawn_attacks(i, 0);
        table[i + 64] = Attacks::slow_pawn_attacks(i, 1);

        i += 1
    }

    table
};

const KNIGHT_TABLE: [u64; 64] = {
    let mut table = [0; 64];
    let mut i = 0;
    while i < table.len() {
        table[i] = Attacks::slow_knight_attacks(i);

        i += 1;
    }

    table
};

const KING_TABLE: [u64; 64] = {
    let mut table = [0; 64];
    let mut i = 0;
    while i < table.len() { 
        table[i] = Attacks::slow_king_attacks(i);

        i += 1
    }

    table
};

//magic stuff
#[allow(long_running_const_eval)]

static SLIDER_ATTACKS : [u64; 88772] = {
    let mut table = [0; 88772];
    let mut square = 0;
    while square < 64 {
        let magic = &BISHOP_MAGICS[square as usize];
        let mask = magic.mask;
        let mut blockers = 0;
        loop {
            let moves = Attacks::slow_bishop_attacks(square, blockers);
            let index = (magic.factor.wrapping_mul(blockers) >> 55) as usize + magic.offset;
            table[index] = moves;
            blockers = blockers.wrapping_sub(mask) & mask;
            if blockers == 0 {
                break;
            }
        }

        let magic = &ROOK_MAGICS[square as usize];
        let mask = magic.mask;
        let mut blockers = 0;
        loop {
            let moves = Attacks::slow_rook_attacks(square, blockers);
            let index = (magic.factor.wrapping_mul(blockers) >> 52) as usize + magic.offset;
            table[index] = moves;
            blockers = blockers.wrapping_sub(mask) & mask;
            if blockers == 0 {
                break;
            }
        }
        square += 1;
    }

    table
};