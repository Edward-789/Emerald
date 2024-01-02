mod board;
mod consts;
mod attacks;

use crate::{
    board::Board,
    attacks::Attacks,
};

fn main() { 
    let board = Board::read_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -");
    board.print_board_info();
    println!("Attacks");

    Board::print_bitboard(Attacks::slow_pawn_attacks(Board::find_square(1, 7), 0));
    println!();
    Board::print_bitboard(Attacks::slow_pawn_attacks(Board::find_square(3, 3), 0));
    println!();
    Board::print_bitboard(Attacks::slow_pawn_attacks(Board::find_square(3, 3), 1));
}
