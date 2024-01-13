mod board;
mod consts;
mod attacks;

use crate::{
    board::Board,
    attacks::Attacks,
};

fn main() { 
    println!("Attacks");

    Board::print_bitboard(Attacks::slow_rook_attacks(Board::find_square(3, 3), 1 << Board::find_square(4, 3)));
    println!();
    Board::print_bitboard(Attacks::slow_bishop_attacks(Board::find_square(3, 3), 1 << Board::find_square(6, 6)));
} 