mod board;
mod consts;
mod attacks;
mod magics;

use crate::{
    board::Board,
    attacks::Attacks,
};

fn main() {
    println!("ROOKS");
    println!();
    Board::print_bitboard(Attacks::rook_attacks(Board::find_square(2, 0), (1 << Board::find_square(5, 0)) | (1 << Board::find_square(2, 4))));    println!();
    println!();    
    println!("BLOCKERS");
    println!();
    Board::print_bitboard((1 << Board::find_square(5, 0)) | (1 << Board::find_square(2, 4)));
    println!();
    println!("BISHOPS");
    println!();
    Board::print_bitboard(Attacks::bishop_attacks(Board::find_square(2, 0), (1 << Board::find_square(1, 1)) | (1 << Board::find_square(6, 4))));
    println!();
    println!("BLOCKERS");
    println!();
    Board::print_bitboard(1 << Board::find_square(1, 1) | (1 << Board::find_square(6, 4)));
    println!();
}