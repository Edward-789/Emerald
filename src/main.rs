mod board;
mod consts;
mod attacks;

use crate::{
    board::Board,
    attacks::Attacks
};

fn main() { 
    Attacks::init_tables();
    println!("Attacks");

    Board::print_bitboard(Attacks::pawn_attacks(Board::find_square(1, 7), 0));
    println!();
    Board::print_bitboard(Attacks::pawn_attacks(Board::find_square(3, 3), 0));
    println!();
    Board::print_bitboard(Attacks::pawn_attacks(Board::find_square(3, 3), 1));
} 