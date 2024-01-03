use crate::{
    consts::Colour,
    utils::{
        shifts
    }
};

static mut PAWN_TABLES: [u64; 128] = [0; 128];
static mut KNIGHT_TABLES: [u64; 128] = [0; 128];

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
            }
        }
    }

    pub fn pawn_attacks(square : usize, colour_num : usize) -> u64 {
        unsafe { PAWN_TABLES[square + (64 * colour_num)] }
    }

    pub fn knight_attacks(square : usize) -> u64 {
        unsafe { KNIGHT_TABLES[square] }
    }

    fn slow_pawn_attacks(square : usize, colour_num : usize) -> u64 {
        assert!(colour_num == 1 || colour_num == 0);

        let pawn = 1 << square;
        let pawn_push = if colour_num == 0 { shifts::shift_north(pawn) } else { shifts::shift_south(pawn) };

        shifts::shift_west(pawn_push) | shifts::shift_east(pawn_push)
    }

    fn slow_knight_attacks(square : usize) -> u64 {
        let knight = 1 << square;

        shifts::shift_north(shifts::shift_north(shifts::shift_east(knight))) |
        shifts::shift_north(shifts::shift_north(shifts::shift_west(knight))) |
        shifts::shift_east(shifts::shift_east(shifts::shift_north(knight))) |
        shifts::shift_east(shifts::shift_east(shifts::shift_south(knight))) |
        shifts::shift_south(shifts::shift_south(shifts::shift_east(knight))) |
        shifts::shift_south(shifts::shift_south(shifts::shift_west(knight))) |
        shifts::shift_west(shifts::shift_west(shifts::shift_north(knight))) |
        shifts::shift_west(shifts::shift_west(shifts::shift_south(knight)))
    }
}   