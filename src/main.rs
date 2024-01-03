mod board;
mod consts;
mod attacks;
mod utils;

use crate::{
    board::Board,
    attacks::Attacks
};

fn main() { 
    Attacks::init_tables();
    println!("Attacks");

    Board::print_bitboard(Attacks::king_attacks(Board::find_square(1, 7)));
    println!();
    Board::print_bitboard(Attacks::king_attacks(Board::find_square(3, 3)));
    println!();
    Board::print_bitboard(Attacks::king_attacks(Board::find_square(3, 3)));
} 