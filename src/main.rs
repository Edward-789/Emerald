mod board;
mod utils;
mod attacks;
mod magics;
mod moves;
mod perft;

use moves::Move;
use crate::{
    board::Board,
    perft::perft
};

fn main() {
    let mut board = Board::read_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    board.apply(Move::new(Board::square_from_str("f3"), Board::square_from_str("f5"), Move::NO_FLAG));
    board.apply(Move::new(Board::square_from_str("h8"), Board::square_from_str("h4"), 0));
    // board.apply(Move::new(Board::square_from_str("g2"), Board::square_from_str("g4"), 0));



    // board.print_board_info();
    perft::<true>(&board, 2);
    // Board::print_bitboard((1 << Move::ROOK_FROM_CASTLING[2]) | (1 << Move::ROOK_TO_CASTLING[2]));
}